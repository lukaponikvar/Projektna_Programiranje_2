use std::{error::Error, fmt};

/// An error which allows us to communicate with a client through displayed `message`.
#[derive(Debug, Clone)]
pub struct CustomError {
    pub message: String,
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CustomError {
    /// Creates a new `CustomError`.
    pub fn new(message: String) -> CustomError {
        return CustomError { message };
    }
}
