use std::io;
use crate::common::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub enum MerklePathError {
    CannotWrite,
    ReadFailed,
}

impl DetailedError for MerklePathError {
    fn details(&self) -> String {
        use MerklePathError::*;

        match self {
            CannotWrite => String::from("Cannot serialize merkle path"),
            ReadFailed => String::from("Could not read merkle path from bytes"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NoteError {
    NoteEmpty
}

impl DetailedError for NoteError {
    fn details(&self) -> String {
        use NoteError::*;

        match self {
            NoteEmpty => String::from("Could not create a note from a payment address")
        }
    }
}

#[derive(Debug)]
pub enum ProofError {
    WriteFailed(io::Error),
    ReadFailed(io::Error),
}

impl DetailedError for ProofError {
    fn details(&self) -> String {
        use ProofError::*;

        match self {
            WriteFailed(err) => err.to_string(),
            ReadFailed(err) => err.to_string(),
        }
    }
}

impl PartialEq for ProofError {
    fn eq(&self, other: &Self) -> bool {
        self.details() == other.details()
    }
}