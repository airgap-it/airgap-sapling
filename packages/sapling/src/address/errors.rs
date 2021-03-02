use std::io;

use crate::common::errors::DetailedError;

#[derive(Debug)]
pub enum SaplingAddressError {
    DiversifierSpaceExhausted,
    InvalidAddressLength(usize),
    SerializationFailed(io::Error),
}

impl DetailedError for SaplingAddressError {
    fn details(&self) -> String {
        use SaplingAddressError::*;

        match self {
            DiversifierSpaceExhausted => String::from("diversifier space is exhausted"),
            InvalidAddressLength(len) => format!("invalid address length, expected 43, got {}", len),
            SerializationFailed(err) => err.to_string(),
        }
    }
}

impl PartialEq for SaplingAddressError {
    fn eq(&self, other: &Self) -> bool {
        use SaplingAddressError::*;

        match (self, other) {
            (DiversifierSpaceExhausted, DiversifierSpaceExhausted) => true,
            (InvalidAddressLength(size), InvalidAddressLength(other_size)) => size == other_size,
            (SerializationFailed(err), SerializationFailed(other_err)) => err.to_string() == other_err.to_string(),
            _ => false
        }
    }
}

#[derive(Debug)]
pub enum IndexedAddressError {
    InvalidAddressLength(usize),
    SerializationFailed(io::Error),
}

impl DetailedError for IndexedAddressError {
    fn details(&self) -> String {
        use IndexedAddressError::*;

        match self {
            InvalidAddressLength(len) => format!("invalid address length, expected 43, got {}", len),
            SerializationFailed(err) => err.to_string(),
        }
    }
}

impl PartialEq for IndexedAddressError {
    fn eq(&self, other: &Self) -> bool {
        use IndexedAddressError::*;

        match (self, other) {
            (InvalidAddressLength(size), InvalidAddressLength(other_size)) => size == other_size,
            (SerializationFailed(err), SerializationFailed(other_err)) => err.to_string() == other_err.to_string(),
            _ => false
        }
    }
}