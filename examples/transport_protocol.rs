use saelient::{
    Pgn,
    transport::{DataTransfer, RequestToSend, Response, Transfer},
};

fn main() {
    // Request to send received from the sender.
    let rts = RequestToSend::new(128, Some(1), Pgn::ProprietaryA);

    // We then use the RTS to start the transfer.
    let mut transfer = Transfer::new(rts);

    // Data that the sender wants to transfer to the receiver.
    let data = [0_u8; 128];

    for (seq, chunk) in data.chunks(7).enumerate() {
        let mut padded = [0xFF; 7];
        padded[..chunk.len()].clone_from_slice(chunk);
        let dt = DataTransfer::new(seq as u8 + 1, padded);
        match transfer.next(dt) {
            Ok(Some(Response::Cts(cts))) => println!("{:?}", cts),
            Ok(Some(Response::End(end))) => println!("{:?}", end),
            Ok(None) => println!("No message"),
            Err((err, res)) => eprintln!("{:?}: {:?}", err, res),
        }
    }
}
