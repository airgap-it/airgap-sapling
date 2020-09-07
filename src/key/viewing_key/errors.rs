use std::fmt;
use crate::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub struct ViewingKeyError {
    cause: Option<String>,
}

impl ViewingKeyError {
    pub fn new() -> Self {
        ViewingKeyError { cause: None }
    }

    pub fn caused_by<T: ToString>(cause: T) -> Self {
        ViewingKeyError { cause: Some(cause.to_string()) }
    }
}

impl DetailedError for ViewingKeyError {
    fn details(&self) -> String {
        match &self.cause {
            Some(cause) => format!("ViewingKeyError: {}", cause),
            None => String::from("ViewingKeyError")
        }
    }
}

impl fmt::Display for ViewingKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details())
    }
}