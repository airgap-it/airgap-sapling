use std::fmt;
use crate::errors::DetailedError;

#[derive(PartialEq, Debug, Clone)]
pub enum DerivationPathError {
    Empty,
    EmptyIndex,
    MissingPrefix,
    InvalidCharacter(Vec<String>),
    Unknown(String),
}

impl DerivationPathError {
    pub fn invalid_character(invalid: Vec<&str>) -> DerivationPathError {
        DerivationPathError::InvalidCharacter(invalid.iter().map(|&i| String::from(i)).collect())
    }

    pub fn unknown(message: &str) -> DerivationPathError {
        DerivationPathError::Unknown(String::from(message))
    }
}

impl DetailedError for DerivationPathError {
    fn details(&self) -> String {
        match self {
            DerivationPathError::Empty => String::from("the path can't be empty"),
            DerivationPathError::EmptyIndex => String::from("the path can't contain empty derivation indices"),
            DerivationPathError::MissingPrefix => String::from("the path must be prefixed with `m/`"),
            DerivationPathError::InvalidCharacter(unknown) => (format!("unknown character `{}`", unknown.join(", "))),
            DerivationPathError::Unknown(cause) => String::from(cause),
        }
    }
}

impl fmt::Display for DerivationPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid derivation path, {}", self.details())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_invalid_character_error() {
        let error = DerivationPathError::invalid_character(vec!["a", "b", "c"]);

        assert_eq!(error, DerivationPathError::InvalidCharacter(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]));
    }

    #[test]
    fn creates_unknown_error() {
        let error = DerivationPathError::unknown("error message");

        assert_eq!(error, DerivationPathError::Unknown("error message".to_owned()));
    }
}