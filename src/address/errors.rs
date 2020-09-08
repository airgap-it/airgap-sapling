use std::fmt;

use crate::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub enum SaplingAddressError {
    DiversifierSpaceExhausted,
}

impl DetailedError for SaplingAddressError {
    fn details(&self) -> String {
        match self {
            SaplingAddressError::DiversifierSpaceExhausted => String::from("diversifier space is exhausted"),
        }
    }
}