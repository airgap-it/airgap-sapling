use std::io;

use crate::common::errors::DetailedError;

#[derive(Debug)]
pub enum OutputDescriptionError {
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for OutputDescriptionError {
    fn details(&self) -> String {
        use OutputDescriptionError::*;

        match self {
            WriteFailed(err) => err.to_string(),
            ReadFailed(err) => err.to_string()
        }
    }
}

impl PartialEq for OutputDescriptionError {
    fn eq(&self, other: &Self) -> bool {
        use OutputDescriptionError::*;

        match (self, other) {
            (WriteFailed(err), WriteFailed(other_err)) => err.to_string() == other_err.to_string(),
            (ReadFailed(err), ReadFailed(other_err)) => err.to_string() == other_err.to_string(),
            _ => false
        }
    }
}