use std::io;

use crate::common::errors::DetailedError;

#[derive(Debug)]
pub enum SpendDescriptionError {
    CreateSpendProofFailed,
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for SpendDescriptionError {
    fn details(&self) -> String {
        use SpendDescriptionError::*;

        match self {
            CreateSpendProofFailed => String::from("Could not create a spend proof"),
            WriteFailed(err) => err.to_string(),
            ReadFailed(err) => err.to_string(),
        }
    }
}

impl PartialEq for SpendDescriptionError {
    fn eq(&self, other: &Self) -> bool {
        self.details() == other.details()
    }
}