use std::io;

use crate::common::errors::DetailedError;

#[derive(Debug)]
pub enum SpendDescriptionError {
    CreateSpendProofFailed,
    PrivateKeyReadFailed(io::Error),
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for SpendDescriptionError {
    fn details(&self) -> String {
        use SpendDescriptionError::*;

        match self {
            CreateSpendProofFailed => String::from("Could not create a spend proof"),
            PrivateKeyReadFailed(err) => err.to_string(),
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