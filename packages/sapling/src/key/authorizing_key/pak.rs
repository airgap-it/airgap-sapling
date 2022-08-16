use zcash_primitives::primitives::ProofGenerationKey;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;
use crate::key::authorizing_key::ProofGenerationKeyError;

impl Serializable<Vec<u8>, SaplingError> for ProofGenerationKey {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        assert_value_or_error(serialized.len() == 64, SaplingError::new())?;

        let ak = jubjub::SubgroupPoint::deserialize(serialized[..32].to_vec()).map_err(|_| SaplingError::caused_by(ProofGenerationKeyError::ReadFailed))?;
        let nsk = jubjub::Fr::deserialize(serialized[32..].to_vec()).map_err(|_| SaplingError::caused_by(ProofGenerationKeyError::ReadFailed))?;

        Ok(ProofGenerationKey { ak, nsk })
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let ak = self.ak.serialize().map_err(|_| SaplingError::caused_by(ProofGenerationKeyError::WriteFailed))?;
        let nsk = self.nsk.serialize().map_err(|_| SaplingError::caused_by(ProofGenerationKeyError::WriteFailed))?;

        Ok([ak, nsk].concat())
    }
}