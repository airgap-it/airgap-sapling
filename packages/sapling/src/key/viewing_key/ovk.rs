use std::convert::TryInto;

use zcash_primitives::keys::OutgoingViewingKey;

use crate::common::errors::SaplingError;
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;

impl Serializable<Vec<u8>, SaplingError> for OutgoingViewingKey {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        assert_value_or_error(serialized.len() == 32, SaplingError::new())?;

        Ok(OutgoingViewingKey(serialized[..32].try_into().unwrap()))
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.0.to_vec())
    }
}