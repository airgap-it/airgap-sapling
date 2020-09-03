use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ViewingKeyError {
    cause: Option<String>,
}

impl ViewingKeyError {
    pub fn new() -> ViewingKeyError {
        ViewingKeyError { cause: None }
    }

    pub fn caused_by<T: ToString>(cause: T) -> ViewingKeyError {
        ViewingKeyError { cause: Some(cause.to_string()) }
    }
}

impl fmt::Display for ViewingKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.cause {
            Some(cause) => write!(f, "ViewingKeyError: {}", cause),
            None => write!(f, "ViewingKeyError")
        }
    }
}