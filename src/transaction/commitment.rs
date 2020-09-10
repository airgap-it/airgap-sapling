use std::convert::TryInto;

use crate::address::SaplingAddress;
use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;
use crate::transaction::errors::SaplingCommitmentError;
use crate::transaction::note::create_note;

#[derive(Debug, PartialEq)]
pub struct SaplingCommitment([u8; 32]);

impl SaplingCommitment {
    pub fn new(address: &SaplingAddress, value: u64, rcm: &[u8]) -> Result<SaplingCommitment, SaplingError> {
        assert_value_or_error(rcm.len() == 32, SaplingError::caused_by(SaplingCommitmentError::InvalidRcm))?;
        let note = create_note(address, value, rcm.try_into().unwrap())?;
        let commitment = SaplingCommitment(note.cmu().to_bytes());

        Ok(commitment)
    }
}

impl Serializable<Vec<u8>, SaplingError> for SaplingCommitment {
    fn deserialize(serialized: Vec<u8>) -> Result<SaplingCommitment, SaplingError> {
        assert_value_or_error(serialized.len() == 32, SaplingError::caused_by(SaplingCommitmentError::InvalidLength))?;

        Ok(SaplingCommitment(serialized[..32].try_into().unwrap()))
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.0.to_vec())
    }
}