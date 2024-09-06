use std::{error::Error, fmt};

#[derive(Debug)]
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
    pub fn new(message: String) -> CustomError {
        return CustomError { message };
    }
}
