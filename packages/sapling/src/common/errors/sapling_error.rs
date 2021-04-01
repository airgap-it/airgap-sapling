use std::fmt;

use crate::common::errors::{CausedBy, DetailedError};

#[derive(Debug, PartialEq)]
pub struct SaplingError(Option<String>);

impl SaplingError {
    pub fn new() -> SaplingError {
        SaplingError(None)
    }
}

impl CausedBy<SaplingError> for SaplingError {
    fn caused_by(cause: SaplingError) -> SaplingError {
        cause
    }
}

impl CausedBy<&str> for SaplingError {
    fn caused_by(cause: &str) -> SaplingError {
        SaplingError(Some(String::from(cause)))
    }
}

impl CausedBy<String> for SaplingError {
    fn caused_by(cause: String) -> SaplingError {
        SaplingError(Some(cause))
    }
}

impl <T: DetailedError> CausedBy<T> for SaplingError {
    fn caused_by(cause: T) -> SaplingError {
        SaplingError(Some(cause.details()))
    }
}

impl fmt::Display for SaplingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(cause) => write!(f, "sapling error, {}", cause),
            None => write!(f, "sapling error")
        }
    }
}