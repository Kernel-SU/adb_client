use std::fmt::Display;

pub type ADBCliResult<T> = Result<T, ADBCliError>;

pub enum ADBCliError {
    Standard(Box<dyn std::error::Error>),
}

impl Display for ADBCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard(error) => write!(f, "{error}"),
        }
    }
}

impl From<std::io::Error> for ADBCliError {
    fn from(value: std::io::Error) -> Self {
        Self::Standard(Box::new(value))
    }
}

impl From<adb_client::RustADBError> for ADBCliError {
    fn from(value: adb_client::RustADBError) -> Self {
        Self::Standard(Box::new(value))
    }
}
