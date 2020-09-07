use std::fmt;

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

impl fmt::Display for SpendingKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.cause {
            Some(cause) => write!(f, "SpendingKeyError: {}", cause),
            None => write!(f, "SpendingKeyError")
        }
    }
}