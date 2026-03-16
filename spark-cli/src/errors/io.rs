use std::io;
use std::num::ParseIntError;
#[derive(Debug)]
pub enum Error {
    PermissionDenied,
    NotFound(String),
    InvalidFormat(String),
    Unknown(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PermissionDenied => write!(f, "Access denied. Please run spark with higher privileges."),
            Self::NotFound(path) => write!(f, "The system could not find the specified path: {}", path),
            Self::InvalidFormat(e) => write!(f, "Data format error: {}",e),
            Self::Unknown(e) => write!(f, "An unexpected system error occurred: {}", e),
        }
    }
}

impl std::error::Error for Error {}
// Convers the raw std::io::Error to io::Error
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            io::ErrorKind::NotFound => Self::NotFound("unknown path".to_string()),
            _ => Self::Unknown(err),
        }
    }
}
// Converts the raw ParseIntError to the SparkError (used in the module gpt.rs and others). 
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidFormat(err.to_string())
    }
}
