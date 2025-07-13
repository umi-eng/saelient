/// DM14 - Memory Access Request
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct MemoryAccessRequest {
    raw: [u8; 8],
}

impl MemoryAccessRequest {
    /// Create a new memory access request.
    ///
    /// Panics if `length` is greater than 2^11.
    pub fn new(command: Command, pointer: Pointer, length: u16, key_or_user_level: u16) -> Self {
        assert!(length <= 0b11111111111);

        let mut raw = [0; 8];

        let length = length.to_le_bytes();
        raw[0] |= length[0];
        raw[1] |= length[1] << 5;

        raw[1] |= u8::from(command) << 1;

        let pointer = match pointer {
            Pointer::Direct(value) => value,
            Pointer::Spatial(value) => value,
        };
        raw[2..6].copy_from_slice(&pointer.to_le_bytes());

        raw[6..8].copy_from_slice(&key_or_user_level.to_le_bytes());

        Self { raw }
    }

    /// The number of bytes to apply the memory operation to.
    pub fn length(&self) -> u16 {
        u16::from_le_bytes([self.raw[0], (self.raw[1] >> 5) & 0b111])
    }

    /// The command type.
    pub fn command(&self) -> Command {
        Command::from((self.raw[1] >> 1) & 0b111)
    }

    /// Memory address or object identifier.
    pub fn pointer(&self) -> Pointer {
        let value = u32::from_le_bytes([self.raw[2], self.raw[3], self.raw[4], self.raw[5]]);
        if self.raw[1] & 0b10000 != 0 {
            Pointer::Spatial(value)
        } else {
            Pointer::Direct(value)
        }
    }

    /// Security key or user level, depending on context.
    pub fn key_or_user_level(&self) -> u16 {
        u16::from_le_bytes([self.raw[6], self.raw[7]])
    }
}

impl From<&MemoryAccessRequest> for [u8; 8] {
    fn from(req: &MemoryAccessRequest) -> Self {
        req.raw
    }
}

impl<'a> TryFrom<&'a [u8]> for MemoryAccessRequest {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: value.try_into().map_err(|_| value)?,
        })
    }
}

/// Memory access request command.
#[derive(Debug, Clone, Copy, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum Command {
    Erase,
    Read,
    Write,
    StatusRequest,
    OperationCompleted,
    OperationFailed,
    BootLoad,
    EdcpGeneration,
    Other(u8),
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        // Cast to underlying value to compare
        u8::from(*self) == u8::from(*other)
    }
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        match value {
            Command::Erase => 0,
            Command::Read => 1,
            Command::Write => 2,
            Command::StatusRequest => 3,
            Command::OperationCompleted => 4,
            Command::OperationFailed => 5,
            Command::BootLoad => 6,
            Command::EdcpGeneration => 7,
            Command::Other(v) => v,
        }
    }
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value {
            0 => Command::Erase,
            1 => Command::Read,
            2 => Command::Write,
            3 => Command::StatusRequest,
            4 => Command::OperationCompleted,
            5 => Command::OperationFailed,
            6 => Command::BootLoad,
            7 => Command::EdcpGeneration,
            n => Command::Other(n),
        }
    }
}

/// Direct or spatial memory addressing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum Pointer {
    Direct(u32),
    Spatial(u32),
}

/// DM15 - Memory Access Response
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct MemoryAccessResponse {
    raw: [u8; 8],
}

impl MemoryAccessResponse {
    /// Create a new memory access response.
    ///
    /// Panics if `length` is greater than 2 ^ 11.
    pub fn new(status: Status, error_indicator: ErrorIndicator, length: u16, seed: u16) -> Self {
        assert!(length <= 0b11111111111);

        let mut raw = [0; 8];

        let length = length.to_le_bytes();
        raw[0] |= length[0];
        raw[1] |= length[1] << 5;

        raw[1] |= u8::from(status) << 1;

        let error_indicator: u32 = error_indicator.into();
        raw[2..5].copy_from_slice(&error_indicator.to_le_bytes()[..3]);

        raw[6..8].copy_from_slice(&seed.to_le_bytes());

        Self { raw }
    }

    pub fn length(&self) -> u16 {
        u16::from_le_bytes([self.raw[0], (self.raw[1] >> 5) & 0b111])
    }

    pub fn status(&self) -> Status {
        Status::from((self.raw[1] >> 1) & 0b111)
    }

    pub fn error_indicator(&self) -> ErrorIndicator {
        let indicator = u32::from_le_bytes([self.raw[2], self.raw[3], self.raw[4], 0]);
        ErrorIndicator::from(indicator)
    }

    pub fn seed(&self) -> u16 {
        u16::from_le_bytes([self.raw[6], self.raw[7]])
    }
}

impl From<&MemoryAccessResponse> for [u8; 8] {
    fn from(res: &MemoryAccessResponse) -> Self {
        res.raw
    }
}

impl<'a> TryFrom<&'a [u8]> for MemoryAccessResponse {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: value.try_into().map_err(|_| value)?,
        })
    }
}

/// Memory access response status.
#[derive(Debug, Clone, Copy, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum Status {
    Proceed,
    Busy,
    OperationCompleted,
    OperationFailed,
    Other(u8),
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        // Cast to underlying value to compare
        u8::from(*self) == u8::from(*other)
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        match value {
            Status::Proceed => 0,
            Status::Busy => 1,
            Status::OperationCompleted => 4,
            Status::OperationFailed => 5,
            Status::Other(o) => o,
        }
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Proceed,
            1 => Self::Busy,
            4 => Self::OperationCompleted,
            5 => Self::OperationFailed,
            _ => Self::Other(value),
        }
    }
}

/// Error indicator state.
#[derive(Debug, Clone, Copy, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum ErrorIndicator {
    None,
    NotIdentified,
    BusyForSomeoneElse,
    BusyErase,
    BusyRead,
    BusyWrite,
    BusyStatus,
    BusyBootLoad,
    BusyEdcpGeneration,
    BusyUnspecified,
    EdcPrameterNotCorrect,
    RamVerifyOnWrite,
    FlashVerifyOnWrite,
    PromVerifyOnWrite,
    InternalFailure,
    AddressingGeneral,
    AddressingBoundary,
    AddressingLength,
    AddressingOutOfBounds,
    AddressingRequiresEraseData,
    AddressingRequiresEraseProgram,
    AddressingRequiresTransferAndEraseProgram,
    AddressingBootLoadExecutableMemory,
    AddressingBootLoadInvalidBoundary,
    DataValueRange,
    DataNameRange,
    Security,
    SecurityInvalidPassword,
    SecurityInvalidUserLevel,
    SecurityInvalidKey,
    SecurityNotInDiagnosticMode,
    SecurityNotInDevelopmentMode,
    SecurityEngineRunning,
    SecurityNotInPark,
    AbortFromSoftwareProcess,
    TooManyRetries,
    NoResponseInTimeAllowed,
    TransportDataNotInitiated,
    TransportDataNotCompleted,
    NoIndicatorAvailable,
    Other(u32),
}

impl PartialEq for ErrorIndicator {
    fn eq(&self, other: &Self) -> bool {
        // Cast to underlying value to compare
        u32::from(*self) == u32::from(*other)
    }
}

impl From<ErrorIndicator> for u32 {
    fn from(value: ErrorIndicator) -> Self {
        let result = match value {
            ErrorIndicator::None => 0x000000,
            ErrorIndicator::NotIdentified => 0x000001,
            ErrorIndicator::BusyForSomeoneElse => 0x000002,
            ErrorIndicator::BusyErase => 0x000010,
            ErrorIndicator::BusyRead => 0x000011,
            ErrorIndicator::BusyWrite => 0x000012,
            ErrorIndicator::BusyStatus => 0x000013,
            ErrorIndicator::BusyBootLoad => 0x000016,
            ErrorIndicator::BusyEdcpGeneration => 0x000017,
            ErrorIndicator::BusyUnspecified => 0x00001F,
            ErrorIndicator::EdcPrameterNotCorrect => 0x000020,
            ErrorIndicator::RamVerifyOnWrite => 0x000021,
            ErrorIndicator::FlashVerifyOnWrite => 0x000022,
            ErrorIndicator::PromVerifyOnWrite => 0x000023,
            ErrorIndicator::InternalFailure => 0x000024,
            ErrorIndicator::AddressingGeneral => 0x000100,
            ErrorIndicator::AddressingBoundary => 0x000101,
            ErrorIndicator::AddressingLength => 0x000102,
            ErrorIndicator::AddressingOutOfBounds => 0x000103,
            ErrorIndicator::AddressingRequiresEraseData => 0x000104,
            ErrorIndicator::AddressingRequiresEraseProgram => 0x000105,
            ErrorIndicator::AddressingRequiresTransferAndEraseProgram => 0x000106,
            ErrorIndicator::AddressingBootLoadExecutableMemory => 0x000107,
            ErrorIndicator::AddressingBootLoadInvalidBoundary => 0x000108,
            ErrorIndicator::DataValueRange => 0x000109,
            ErrorIndicator::DataNameRange => 0x00010A,
            ErrorIndicator::Security => 0x001000,
            ErrorIndicator::SecurityInvalidPassword => 0x001001,
            ErrorIndicator::SecurityInvalidUserLevel => 0x001002,
            ErrorIndicator::SecurityInvalidKey => 0x001003,
            ErrorIndicator::SecurityNotInDiagnosticMode => 0x001004,
            ErrorIndicator::SecurityNotInDevelopmentMode => 0x001005,
            ErrorIndicator::SecurityEngineRunning => 0x001006,
            ErrorIndicator::SecurityNotInPark => 0x001007,
            ErrorIndicator::AbortFromSoftwareProcess => 0x010000,
            ErrorIndicator::TooManyRetries => 0x010001,
            ErrorIndicator::NoResponseInTimeAllowed => 0x010002,
            ErrorIndicator::TransportDataNotInitiated => 0x010003,
            ErrorIndicator::TransportDataNotCompleted => 0x010004,
            ErrorIndicator::NoIndicatorAvailable => 0xFFFFFF,
            ErrorIndicator::Other(o) => o,
        };

        // ensure the returned value is only 24-bits.
        assert!(result <= 0xFFFFFF);

        result
    }
}

impl From<u32> for ErrorIndicator {
    fn from(value: u32) -> Self {
        assert!(value <= 0xFFFFFF);

        match value {
            0x000000 => Self::None,
            0x000001 => ErrorIndicator::NotIdentified,
            0x000002 => ErrorIndicator::BusyForSomeoneElse,
            0x000010 => ErrorIndicator::BusyErase,
            0x000011 => ErrorIndicator::BusyRead,
            0x000012 => ErrorIndicator::BusyWrite,
            0x000013 => ErrorIndicator::BusyStatus,
            0x000016 => ErrorIndicator::BusyBootLoad,
            0x000017 => ErrorIndicator::BusyEdcpGeneration,
            0x00001F => ErrorIndicator::BusyUnspecified,
            0x000020 => ErrorIndicator::EdcPrameterNotCorrect,
            0x000021 => ErrorIndicator::RamVerifyOnWrite,
            0x000022 => ErrorIndicator::FlashVerifyOnWrite,
            0x000023 => ErrorIndicator::PromVerifyOnWrite,
            0x000024 => ErrorIndicator::InternalFailure,
            0x000100 => ErrorIndicator::AddressingGeneral,
            0x000101 => ErrorIndicator::AddressingBoundary,
            0x000102 => ErrorIndicator::AddressingLength,
            0x000103 => ErrorIndicator::AddressingOutOfBounds,
            0x000104 => ErrorIndicator::AddressingRequiresEraseData,
            0x000105 => ErrorIndicator::AddressingRequiresEraseProgram,
            0x000106 => ErrorIndicator::AddressingRequiresTransferAndEraseProgram,
            0x000107 => ErrorIndicator::AddressingBootLoadExecutableMemory,
            0x000108 => ErrorIndicator::AddressingBootLoadInvalidBoundary,
            0x000109 => ErrorIndicator::DataValueRange,
            0x00010A => ErrorIndicator::DataNameRange,
            0x001000 => ErrorIndicator::Security,
            0x001001 => ErrorIndicator::SecurityInvalidPassword,
            0x001002 => ErrorIndicator::SecurityInvalidUserLevel,
            0x001003 => ErrorIndicator::SecurityInvalidKey,
            0x001004 => ErrorIndicator::SecurityNotInDiagnosticMode,
            0x001005 => ErrorIndicator::SecurityNotInDevelopmentMode,
            0x001006 => ErrorIndicator::SecurityEngineRunning,
            0x001007 => ErrorIndicator::SecurityNotInPark,
            0x010000 => ErrorIndicator::AbortFromSoftwareProcess,
            0x010001 => ErrorIndicator::TooManyRetries,
            0x010002 => ErrorIndicator::NoResponseInTimeAllowed,
            0x010003 => ErrorIndicator::TransportDataNotInitiated,
            0x010004 => ErrorIndicator::TransportDataNotCompleted,
            0xFFFFFF => ErrorIndicator::NoIndicatorAvailable,
            o => ErrorIndicator::Other(o),
        }
    }
}

/// EDCP Extension State.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum EdcpExtensionState {
    Completed,
    ConcatenateFollowingAsHigherOrder,
    ConcatenateFollowingAsLowerOrder,
    IndicatorIsError,
    IndiactorIsErrorWithSeedTimeToCompletion,
    NoIndicatorAvailable,
}

/// DM17 - Boot Load Data
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct BootLoadData {
    raw: [u8; 8],
}

impl BootLoadData {
    pub fn data(&self) -> [u8; 8] {
        self.raw
    }
}

impl From<&BootLoadData> for [u8; 8] {
    fn from(bl: &BootLoadData) -> Self {
        bl.raw
    }
}

impl<'a> TryFrom<&'a [u8]> for BootLoadData {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: value.try_into().map_err(|_| value)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_access_request() {
        let raw: &[u8] = &[0x20, 0x22, 0x45, 0x23, 0x01, 0x00, 0x00, 0x00];

        let rq = MemoryAccessRequest::try_from(raw).unwrap();
        assert_eq!(rq.length(), 288);
        assert_eq!(rq.command(), Command::Read);
        assert_eq!(rq.pointer(), Pointer::Direct(0x012345));

        // check we get the same result when we serialize back into bytes.
        let bytes: [u8; 8] = (&rq).into();
        assert_eq!(raw, bytes);
    }
}
