use thiserror::Error;

#[derive(Debug, Error)]
pub enum PimParseError {
    #[error("Packet too short")]
    PacketTooShort,
    #[error("Invalid PIM version: {0}")]
    InvalidVersion(u8),
    #[error("Invalid PIM type: {0}")]
    InvalidType(u8),
}
