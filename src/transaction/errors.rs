use std::io;
use crate::common::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub enum SaplingCommitmentError {
    InvalidLength,
    InvalidRcm,
}

impl DetailedError for SaplingCommitmentError {
    fn details(&self) -> String {
        use SaplingCommitmentError::*;

        match self {
            InvalidLength => String::from("a commitment must be of length 32"),
            InvalidRcm => String::from("invalid rcm, an rcm must be of length 32"),
        }
    }
}