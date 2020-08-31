use std::fmt;

#[derive(Debug, PartialEq)]
pub struct SpendingKeyError {
    cause: String,
}

impl SpendingKeyError {
    pub fn caused_by<T: ToString>(cause: T) -> SpendingKeyError {
        SpendingKeyError { cause: cause.to_string() }
    }
}

impl fmt::Display for SpendingKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SpendingKeyError: {}", self.cause)
    }
}