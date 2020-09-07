use std::fmt;
use crate::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub struct SaplingAddressError {
    cause: Option<String>,
}

impl SaplingAddressError {
    pub fn new() -> SaplingAddressError {
        SaplingAddressError { cause: None }
    }

    pub fn caused_by<T: ToString>(cause: T) -> SaplingAddressError {
        SaplingAddressError { cause: Some(cause.to_string()) }
    }
}

impl DetailedError for SaplingAddressError {
    fn details(&self) -> String {
        match &self.cause {
            Some(cause) => format!("SaplingAddressError: {}", cause),
            None => String::from("SaplingAddressError")
        }
    }
}

impl fmt::Display for SaplingAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details())
    }
}