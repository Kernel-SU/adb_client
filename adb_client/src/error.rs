use thiserror::Error;

/// Custom Result type thrown by this crate.
pub type Result<T> = std::result::Result<T, RustADBError>;

/// Represents all error types that can be thrown by the crate.
#[derive(Error, Debug)]
pub enum RustADBError {
    /// Indicates that an error occurred with I/O.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Indicates that an error occurred when sending ADB request.
    #[error("ADB request failed - {0}")]
    ADBRequestFailed(String),
    /// Indicated that an unexpected command has been received
    #[error("Wrong response command received: {0}. Expected {1}")]
    WrongResponseReceived(String, String),
    /// Indicates that an error occurred during UTF-8 parsing.
    #[error(transparent)]
    Utf8StringError(#[from] std::string::FromUtf8Error),
    /// Indicates an error with the integer conversion.
    #[error(transparent)]
    IntegerConversionError(#[from] std::num::TryFromIntError),
    /// Indicates that an error occurred when converting a value.
    #[error("Conversion error")]
    ConversionError,
    /// Integrity of the received message cannot be validated
    #[error("Invalid integrity. Expected CRC32 {0}, got {1}")]
    InvalidIntegrity(u32, u32),
    /// Error while locking mutex
    #[error("error while locking data")]
    PoisonError,
}

impl<T> From<std::sync::PoisonError<T>> for RustADBError {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Self::PoisonError
    }
}
