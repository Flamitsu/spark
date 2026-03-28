#[derive(Debug)]
/// Errors related to the bad usage of the ignix command. Like for example an invalid argument 
#[allow(unused)]
pub enum Error {
    InvalidArgument(String),
    EFINotFound(String),
    UserAborted,
    NotEFIPartitionFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            Error::EFINotFound(path)=>write!(f,"Not valid EFI binary have been found in {}",path),
            Error::UserAborted => write!(f, "User aborted the process."),
            Error::NotEFIPartitionFound => write!(f, "Not UEFI partition found in the system."),
        }
    }
}

impl std::error::Error for Error {}
