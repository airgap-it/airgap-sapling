use std::{io, fmt};

use crate::errors::DetailedError;

#[derive(Debug)]
pub enum ViewingKeyError {
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for ViewingKeyError {
    fn details(&self) -> String {
        match self {
            ViewingKeyError::WriteFailed(err) => err.to_string(),
            ViewingKeyError::ReadFailed(err) => err.to_string()
        }
    }
}

impl PartialEq for ViewingKeyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ViewingKeyError::WriteFailed(err), ViewingKeyError::WriteFailed(other_err)) => err.to_string() == other_err.to_string(),
            (ViewingKeyError::ReadFailed(err), ViewingKeyError::ReadFailed(other_err)) => err.to_string() == other_err.to_string(),
            _ => false
        }
    }
}