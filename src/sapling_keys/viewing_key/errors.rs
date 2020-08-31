use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ViewingKeyError {
    cause: String,
}

impl ViewingKeyError {
    pub fn caused_by<T: ToString>(cause: T) -> ViewingKeyError {
        ViewingKeyError { cause: cause.to_string() }
    }
}

impl fmt::Display for ViewingKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ViewingKeyError: {}", self.cause)
    }
}