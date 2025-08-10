//! Transport protocol (J1939-21)

mod message;

use managed::ManagedSlice;
pub use message::{
    AbortReason, AbortSenderRole, ClearToSend, ConnectionAbort, DataTransfer, EndOfMessageAck,
    RequestToSend,
};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum Error {
    StorageTooSmall,
    Sequence,
    PreviousAbort,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum Response {
    Cts(ClearToSend),
    End(EndOfMessageAck),
}

impl From<&Response> for [u8; 8] {
    fn from(value: &Response) -> Self {
        match value {
            Response::Cts(cts) => cts.into(),
            Response::End(end) => end.into(),
        }
    }
}

/// An ongoing transport-protocol transfer.
#[derive(Debug)]
pub struct Transfer<'a> {
    rts: RequestToSend,
    rx_packets: u8,
    storage: ManagedSlice<'a, u8>,
    abort: bool,
}

impl<'a> Transfer<'a> {
    /// Create a new transfer from a RTS message received from the sender.
    #[cfg(feature = "alloc")]
    pub fn new(rts: RequestToSend) -> Self {
        Self {
            rts,
            rx_packets: 0,
            storage: Vec::new().into(),
            abort: false,
        }
    }

    /// Create a new transfer from a RTS message received from the sender using provided storage.
    pub fn new_with_storage(rts: RequestToSend, storage: impl Into<ManagedSlice<'a, u8>>) -> Self {
        Self {
            rts,
            rx_packets: 0,
            storage: storage.into(),
            abort: false,
        }
    }

    /// Return read-only acess to the internal buffer.
    ///
    /// The contents of this buffer are only valid after the transfer is complete.
    pub fn finished(&self) -> Option<&[u8]> {
        if self.rx_packets >= self.rts.total_packets() && !self.abort {
            Some(&self.storage[..self.rts.total_size() as usize])
        } else {
            None
        }
    }

    /// Feed the transfer with the next data transfer.
    pub fn next(
        &mut self,
        msg: DataTransfer,
    ) -> Result<Option<Response>, (Error, ConnectionAbort)> {
        if self.abort {
            return Err((
                Error::PreviousAbort,
                ConnectionAbort::new(
                    AbortReason::UnexpectedDataTransfer,
                    AbortSenderRole::Receiver,
                    self.rts.pgn(),
                ),
            ));
        }

        if msg.sequence() != self.rx_packets + 1 {
            self.abort = true;
            return Err((
                Error::Sequence,
                ConnectionAbort::new(
                    AbortReason::BadSequenceNumber,
                    AbortSenderRole::Receiver,
                    self.rts.pgn(),
                ),
            ));
        }

        match &mut self.storage {
            #[cfg(feature = "alloc")]
            ManagedSlice::Owned(vec) => {
                vec.extend_from_slice(&msg.data());
                vec.truncate(self.rts.total_size() as usize);
            }
            ManagedSlice::Borrowed(slice) => {
                let Some(chunk) = slice.chunks_mut(7).nth(self.rx_packets as usize) else {
                    self.abort = true;
                    return Err((
                        Error::StorageTooSmall,
                        ConnectionAbort::new(
                            AbortReason::Custom,
                            AbortSenderRole::Receiver,
                            self.rts.pgn(),
                        ),
                    ));
                };
                chunk.clone_from_slice(&msg.data()[..chunk.len()]);
            }
        }

        self.rx_packets += 1;

        if self.rx_packets == self.rts.total_packets() {
            return Ok(Some(Response::End(EndOfMessageAck::new(
                self.rts.total_size(),
                self.rts.total_packets(),
                self.rts.pgn(),
            ))));
        }

        if let Some(packets_per_response) = self.rts.max_packets_per_response() {
            // send cts on nth data transfer
            if msg.sequence() % packets_per_response == 0 {
                return Ok(Some(Response::Cts(ClearToSend::new(
                    self.rts.max_packets_per_response(),
                    self.rx_packets + 1,
                    self.rts.pgn(),
                ))));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::Pgn;

    #[test]
    fn transmission() {
        let rts = message::RequestToSend::new(16, Some(2), Pgn::ProprietaryA);
        let mut transfer = Transfer::new(rts);

        // send first data transfer
        let dt = message::DataTransfer::try_from([1, 1, 2, 3, 4, 5, 6, 7].as_ref()).unwrap();
        transfer.next(dt).unwrap();

        // send second data transfer which should trigger a CTS response.
        let dt = message::DataTransfer::try_from([2, 1, 2, 3, 4, 5, 6, 7].as_ref()).unwrap();
        let cts_response = transfer.next(dt).unwrap().expect("Response frame");
        assert!(matches!(&cts_response, Response::Cts(cts) if cts.next_sequence() == 3));

        // send third data transfer which should trigger a EndOfMsgAck response.
        let dt = message::DataTransfer::try_from([3, 1, 2, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_ref())
            .unwrap();
        let ack_response = transfer.next(dt).unwrap().expect("Response frame");
        assert!(matches!(&ack_response, Response::End(end) if end.total_size() == 16));
        assert!(matches!(&ack_response, Response::End(end) if end.total_packets() == 3));

        assert_eq!(
            transfer.finished().unwrap(),
            &[1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 1, 2]
        );
    }
}
