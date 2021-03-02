use zcash_primitives::primitives::PaymentAddress;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;

use super::errors::SaplingAddressError;

impl Serializable<Vec<u8>, SaplingError> for PaymentAddress {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        assert_byte_length(&serialized).map_err(SaplingError::caused_by)?;

        let mut bytes = [0u8; 43];
        bytes.copy_from_slice(&serialized[..]);

        PaymentAddress::from_bytes(&bytes).ok_or_else(SaplingError::new)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.to_bytes().to_vec())
    }
}

fn assert_byte_length(bytes: &[u8]) -> Result<(), SaplingAddressError> {
    let len = bytes.len();
    assert_value_or_error(len == 43, SaplingAddressError::InvalidAddressLength(len))
}