use std::{io, fmt};

use crate::errors::DetailedError;

#[derive(Debug)]
pub enum SpendingKeyError {
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for SpendingKeyError {
    fn details(&self) -> String {
        match self {
            SpendingKeyError::WriteFailed(err) => err.to_string(),
            SpendingKeyError::ReadFailed(err) => err.to_string()
        }
    }
}

impl PartialEq for SpendingKeyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SpendingKeyError::WriteFailed(err), SpendingKeyError::WriteFailed(other_err)) => err.to_string() == other_err.to_string(),
            (SpendingKeyError::ReadFailed(err), SpendingKeyError::ReadFailed(other_err)) => err.to_string() == other_err.to_string(),
            _ => false
        }
    }
}