use thiserror::Error;

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("The header provided is unknown.")]
    UnknownHeader,
    #[error("The value provided does not represent an endian.")]
    Endianness,
}
