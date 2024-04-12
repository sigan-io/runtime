use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketHeaderError {
    #[error("The packet type provided is unknown.")]
    UnknownPacketType,
    #[error("The value provided does not represent an endian.")]
    InvalidEndianness,
}

#[derive(Debug, Error)]
pub enum HttpHeaderError {
    #[error("The header provided is unknown.")]
    UnknownHeader,
}
