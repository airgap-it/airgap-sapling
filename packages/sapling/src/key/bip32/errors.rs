use crate::common::errors::DetailedError;

#[derive(Debug, PartialEq)]
pub enum Bip32PathError {
    Empty,
    MissingPrefix,
}

impl DetailedError for Bip32PathError {
    fn details(&self) -> String {
        match self {
            Bip32PathError::Empty => String::from("the path can't be empty"),
            Bip32PathError::MissingPrefix => String::from("the path must be prefixed with `m/`"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Bip32IndexError {
    Empty,
    InvalidCharacter(Vec<String>),
    ParseError,
}

impl DetailedError for Bip32IndexError {
    fn details(&self) -> String {
        match self {
            Bip32IndexError::Empty => String::from("the index can't be empty"),
            Bip32IndexError::InvalidCharacter(unknown) => (format!("unknown character `{}`", unknown.join(", "))),
            Bip32IndexError::ParseError => String::from("could not parse bip32 index"),
        }
    }
}