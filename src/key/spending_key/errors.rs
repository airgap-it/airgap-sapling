use std::fmt;
use crate::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub struct SpendingKeyError {
    cause: Option<String>,
}

impl SpendingKeyError {
    pub fn new() -> SpendingKeyError {
        SpendingKeyError { cause: None }
    }

    pub fn caused_by<T: ToString>(cause: T) -> SpendingKeyError {
        SpendingKeyError { cause: Some(cause.to_string()) }
    }
}

impl DetailedError for SpendingKeyError {
    fn details(&self) -> String {
        match &self.cause {
            Some(cause) => format!("SpendingKeyError: {}", cause),
            None => String::from("SpendingKeyError")
        }
    }
}

impl fmt::Display for SpendingKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details())
    }
}