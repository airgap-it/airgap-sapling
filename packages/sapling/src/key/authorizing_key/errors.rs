use crate::common::errors::DetailedError;

#[derive(Debug)]
pub enum ProofGenerationKeyError {
    WriteFailed,
    ReadFailed,
}

impl DetailedError for ProofGenerationKeyError {
    fn details(&self) -> String {
        match self {
            ProofGenerationKeyError::WriteFailed => String::from("ProofGenerationKey write failed."),
            ProofGenerationKeyError::ReadFailed => String::from("ProofGenerationKey read failed."),
        }
    }
}

impl PartialEq for ProofGenerationKeyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ProofGenerationKeyError::WriteFailed, ProofGenerationKeyError::WriteFailed) => true,
            (ProofGenerationKeyError::ReadFailed, ProofGenerationKeyError::ReadFailed) => true,
            _ => false
        }
    }
}