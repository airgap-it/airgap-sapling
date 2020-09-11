use std::io;

use crate::common::errors::DetailedError;

#[derive(Debug)]
pub enum SpendDescriptionError {
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for SpendDescriptionError {
    fn details(&self) -> String {
        use SpendDescriptionError::*;

        match self {
            WriteFailed(err) => err.to_string(),
            ReadFailed(err) => err.to_string()
        }
    }
}

impl PartialEq for SpendDescriptionError {
    fn eq(&self, other: &Self) -> bool {
        use SpendDescriptionError::*;

        match (self, other) {
            (WriteFailed(err), WriteFailed(other_err)) => err.to_string() == other_err.to_string(),
            (ReadFailed(err), ReadFailed(other_err)) => err.to_string() == other_err.to_string(),
            _ => false
        }
    }
}