use zcash_primitives::transaction::components::SpendDescription;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::spend::errors::SpendDescriptionError;

impl Serializable<Vec<u8>, SaplingError> for SpendDescription {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        SpendDescription::read(&mut &serialized[..]).map_err(SpendDescriptionError::ReadFailed).map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(SpendDescriptionError::WriteFailed).map_err(SaplingError::caused_by)?;
        
        Ok(bytes)
    }
}