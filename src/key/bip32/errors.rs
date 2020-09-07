use std::fmt;
use crate::errors::DetailedError;

#[derive(PartialEq, Debug, Clone)]
pub enum Bip32Error {
    EmptyPath,
    EmptyIndex,
    MissingPrefix,
    InvalidCharacter(Vec<String>),
    Unknown(String),
}

impl Bip32Error {
    pub fn invalid_character(invalid: Vec<&str>) -> Bip32Error {
        Bip32Error::InvalidCharacter(invalid.iter().map(|&i| String::from(i)).collect())
    }

    pub fn unknown(message: &str) -> Bip32Error {
        Bip32Error::Unknown(String::from(message))
    }
}

impl DetailedError for Bip32Error {
    fn details(&self) -> String {
        match self {
            Bip32Error::EmptyPath => String::from("the path can't be empty"),
            Bip32Error::EmptyIndex => String::from("the path can't contain empty bip32 indices"),
            Bip32Error::MissingPrefix => String::from("the path must be prefixed with `m/`"),
            Bip32Error::InvalidCharacter(unknown) => (format!("unknown character `{}`", unknown.join(", "))),
            Bip32Error::Unknown(cause) => String::from(cause),
        }
    }
}

impl fmt::Display for Bip32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid bip32 path, {}", self.details())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_invalid_character_error() {
        let error = Bip32Error::invalid_character(vec!["a", "b", "c"]);

        assert_eq!(error, Bip32Error::InvalidCharacter(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]));
    }

    #[test]
    fn creates_unknown_error() {
        let error = Bip32Error::unknown("error message");

        assert_eq!(error, Bip32Error::Unknown("error message".to_owned()));
    }
}