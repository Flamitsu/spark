#[derive(Debug)]
#[allow(unused)]
#[allow(dead_code)]

// Possible errors that can occur while the program is executed. 
pub enum SparkError{
    InvalidArgument(String)
}
impl std::fmt::Display for SparkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Depends on the error, it will write a different error message
        match self{
            SparkError::InvalidArgument(argument) => write!(f, "Invalid argument: {}", argument)
        }
    }
}
impl std::error::Error for SparkError {}
