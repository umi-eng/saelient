use crate::id::Pgn;

/// Request to send (TP.CM_RTS) message.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct RequestToSend {
    total_size: u16,
    total_packets: u8,
    max_packets_per_response: Option<u8>,
    pgn: Pgn,
}

impl RequestToSend {
    const MUX: u8 = 16;

    /// Create a new request to send message.
    ///
    /// - `total_size` must be between 9 and 1785 bytes.
    /// - `max_packets_per_response` must be between
    pub fn new(total_size: u16, max_packets_per_response: Option<u8>, pgn: Pgn) -> Self {
        assert!(total_size <= 1785);
        assert!(total_size >= 9);

        let total_packets = total_size.div_ceil(7);
        assert!(total_packets >= 2);
        assert!(total_packets <= 255);
        let total_packets = total_packets as u8;

        if let Some(max) = max_packets_per_response {
            assert!(
                max < 255,
                "No limit is designated with `None` for`max_packets_per_response`"
            );
        }

        Self {
            total_size,
            total_packets,
            max_packets_per_response,
            pgn,
        }
    }

    /// Total number of bytes in this transfer.
    pub fn total_size(&self) -> u16 {
        self.total_size
    }

    /// Total number of packets in this transfer.
    pub fn total_packets(&self) -> u8 {
        self.total_packets
    }

    /// The maximum number of packets the sender is allowed to respond with for
    /// every TP.CM_CTS message.
    ///
    /// `None` signifies no limit.
    pub fn max_packets_per_response(&self) -> Option<u8> {
        self.max_packets_per_response
    }

    /// Tranfer contents PGN.
    pub fn pgn(&self) -> Pgn {
        self.pgn
    }
}

impl From<RequestToSend> for [u8; 8] {
    fn from(val: RequestToSend) -> Self {
        let total_size = val.total_size.to_le_bytes();
        let pgn = u32::from(val.pgn).to_le_bytes();
        [
            RequestToSend::MUX,
            total_size[0],
            total_size[1],
            val.total_packets,
            val.max_packets_per_response.unwrap_or(255),
            pgn[0],
            pgn[1],
            pgn[2],
        ]
    }
}

impl<'a> TryFrom<&'a [u8]> for RequestToSend {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        if value[0] != Self::MUX {
            return Err(value);
        }

        Ok(Self {
            total_size: u16::from_le_bytes([value[1], value[2]]),
            total_packets: value[3],
            max_packets_per_response: match value[4] {
                0..255 => Some(value[4]),
                255 => None,
            },
            pgn: Pgn::from(u32::from_le_bytes([value[5], value[6], value[7], 0x00])),
        })
    }
}

/// Clear to send (TP.CM_CTS) message.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct ClearToSend {
    max_packets_per_response: Option<u8>,
    next_sequence: u8,
    pgn: Pgn,
}

impl ClearToSend {
    const MUX: u8 = 17;

    /// Create a new CTS message.
    pub fn new(max_packets_per_response: Option<u8>, next_sequence: u8, pgn: Pgn) -> Self {
        Self {
            max_packets_per_response,
            next_sequence,
            pgn,
        }
    }

    /// Number of packets that can be sent sent.
    pub fn max_packets_per_response(&self) -> Option<u8> {
        self.max_packets_per_response
    }

    /// Next sequence number.
    pub fn next_sequence(&self) -> u8 {
        self.next_sequence
    }
}

impl From<&ClearToSend> for [u8; 8] {
    fn from(value: &ClearToSend) -> Self {
        let pgn = u32::from(value.pgn).to_le_bytes();

        [
            ClearToSend::MUX,
            value.max_packets_per_response.unwrap_or(255),
            value.next_sequence,
            0xFF, // reserved
            0xFF, // reserved
            pgn[0],
            pgn[1],
            pgn[2],
        ]
    }
}

impl<'a> TryFrom<&'a [u8]> for ClearToSend {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        if value[0] != Self::MUX {
            return Err(value);
        }

        let pgn = Pgn::from(u32::from_le_bytes([value[5], value[6], value[7], 0x00]));

        Ok(Self {
            max_packets_per_response: match value[1] {
                0..255 => Some(value[1]),
                255 => None,
            },
            next_sequence: value[2],
            pgn,
        })
    }
}

/// End of message acknowledge (TP.CM_EndOfMsgAck) message.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct EndOfMessageAck {
    total_size: u16,
    total_packets: u8,
    pgn: Pgn,
}

impl EndOfMessageAck {
    const MUX: u8 = 19;

    /// Creates a new end of message acknowledge message.
    pub fn new(total_size: u16, total_packets: u8, pgn: Pgn) -> Self {
        Self {
            total_size,
            total_packets,
            pgn,
        }
    }

    /// Total message size in bytes.
    pub fn total_size(&self) -> u16 {
        self.total_size
    }

    /// Total number of packets transferred.
    pub fn total_packets(&self) -> u8 {
        self.total_packets
    }

    /// Tranfer contents PGN.
    pub fn pgn(&self) -> Pgn {
        self.pgn
    }
}

impl From<&EndOfMessageAck> for [u8; 8] {
    fn from(value: &EndOfMessageAck) -> Self {
        let total_size = value.total_size.to_le_bytes();
        let pgn = u32::from(value.pgn).to_le_bytes();

        [
            EndOfMessageAck::MUX,
            total_size[0],
            total_size[1],
            value.total_packets,
            0xFF,
            pgn[0],
            pgn[1],
            pgn[2],
        ]
    }
}

impl<'a> TryFrom<&'a [u8]> for EndOfMessageAck {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        if value[0] != Self::MUX {
            return Err(value);
        }

        let total_size = u16::from_le_bytes([value[1], value[2]]);

        let total_packets = value[3];

        let pgn = Pgn::from(u32::from_le_bytes([value[5], value[6], value[7], 0x00]));

        Ok(Self {
            total_size,
            total_packets,
            pgn,
        })
    }
}

/// Connection abort (TP.Conn_Abort) message.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct ConnectionAbort {
    reason: AbortReason,
    sender_role: AbortSenderRole,
    pgn: Pgn,
}

impl ConnectionAbort {
    const MUX: u8 = 255;

    /// Create a new connection abort message.
    pub fn new(reason: AbortReason, sender_role: AbortSenderRole, pgn: Pgn) -> Self {
        Self {
            reason,
            sender_role,
            pgn,
        }
    }

    /// Abort reason.
    pub fn reason(&self) -> AbortReason {
        self.reason
    }

    /// Abort sender role.
    pub fn sender_role(&self) -> AbortSenderRole {
        self.sender_role
    }

    /// Tranfer contents PGN.
    pub fn pgn(&self) -> Pgn {
        self.pgn
    }
}

impl<'a> TryFrom<&'a [u8]> for ConnectionAbort {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        if value[0] != Self::MUX {
            return Err(value);
        }

        Ok(Self {
            reason: AbortReason::try_from(value[1]).unwrap_or(AbortReason::Custom),
            sender_role: AbortSenderRole::try_from(value[2] & 0b00000011)
                .unwrap_or(AbortSenderRole::NotSpecified),
            pgn: Pgn::from(u32::from_le_bytes([value[5], value[6], value[7], 0x00])),
        })
    }
}

impl From<&ConnectionAbort> for [u8; 8] {
    fn from(value: &ConnectionAbort) -> Self {
        let pgn = u32::from(value.pgn).to_le_bytes();

        [
            ConnectionAbort::MUX,
            u8::from(&value.reason),
            u8::from(&value.sender_role) & 0b11111100,
            0xFF,
            0xFF,
            pgn[0],
            pgn[1],
            pgn[2],
        ]
    }
}

/// Abort reason.
///
/// See J1939™-21 table 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum AbortReason {
    /// Already in one or more connection managed sessions and cannot support another.
    MaxConnections = 1,
    /// System resources were needed for another task, so this connection managed session was terminated.
    CanceledBySystem = 2,
    /// A timeout occurred, and this is the connection abort to close the session.
    Timeout = 3,
    /// CTS messages received when data transfer is in progress.
    CtsWhileDataTransfer = 4,
    /// Maximum retransmit request limit reached.
    RetransmitLimitReached = 5,
    /// Unexpected data transfer packet.
    UnexpectedDataTransfer = 6,
    /// Bad sequence number (software cannot recover).
    BadSequenceNumber = 7,
    /// Duplicate sequence number (software cannot recover).
    DuplicateSequenceNumber = 8,
    /// Total Message Size is greater than 1785 bytes.
    MessageTooLarge = 9,
    /// If a Connection Abort reason is identified that is not listed in the table use code 250.
    Custom = 250,
}

impl TryFrom<u8> for AbortReason {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::MaxConnections as u8 => Ok(Self::MaxConnections),
            x if x == Self::CanceledBySystem as u8 => Ok(Self::CanceledBySystem),
            x if x == Self::Timeout as u8 => Ok(Self::Timeout),
            x if x == Self::CtsWhileDataTransfer as u8 => Ok(Self::CtsWhileDataTransfer),
            x if x == Self::RetransmitLimitReached as u8 => Ok(Self::RetransmitLimitReached),
            x if x == Self::UnexpectedDataTransfer as u8 => Ok(Self::UnexpectedDataTransfer),
            x if x == Self::BadSequenceNumber as u8 => Ok(Self::BadSequenceNumber),
            x if x == Self::DuplicateSequenceNumber as u8 => Ok(Self::DuplicateSequenceNumber),
            x if x == Self::MessageTooLarge as u8 => Ok(Self::MessageTooLarge),
            x if x == Self::Custom as u8 => Ok(Self::Custom),
            _ => Err(value),
        }
    }
}

impl From<&AbortReason> for u8 {
    fn from(value: &AbortReason) -> Self {
        *value as u8
    }
}

/// Abort message sender role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum AbortSenderRole {
    Sender = 0b00,
    Receiver = 0b01,
    Reserved = 0b10,
    NotSpecified = 0b11,
}

impl TryFrom<u8> for AbortSenderRole {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Sender as u8 => Ok(Self::Sender),
            x if x == Self::Receiver as u8 => Ok(Self::Receiver),
            x if x == Self::NotSpecified as u8 => Ok(Self::NotSpecified),
            _ => Err(value),
        }
    }
}

impl From<&AbortSenderRole> for u8 {
    fn from(value: &AbortSenderRole) -> Self {
        *value as u8
    }
}

/// Data transfer (TP.DT) message.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct DataTransfer {
    sequence: u8,
    data: [u8; 7],
}

impl DataTransfer {
    pub fn new(sequence: u8, data: [u8; 7]) -> Self {
        Self { sequence, data }
    }

    /// Packet sequence number.
    pub fn sequence(&self) -> u8 {
        self.sequence
    }

    /// Payload data.
    pub fn data(&self) -> [u8; 7] {
        self.data
    }
}

impl From<&DataTransfer> for [u8; 8] {
    fn from(value: &DataTransfer) -> Self {
        [
            value.sequence,
            value.data[0],
            value.data[1],
            value.data[2],
            value.data[3],
            value.data[4],
            value.data[5],
            value.data[6],
        ]
    }
}

impl<'a> TryFrom<&'a [u8]> for DataTransfer {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        Ok(Self {
            sequence: value[0],
            data: [
                value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            ],
        })
    }
}
