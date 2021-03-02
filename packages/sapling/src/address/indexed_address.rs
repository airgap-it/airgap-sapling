use std::convert::TryInto;
use std::io::Write;

use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::DiversifierIndex;

use crate::address::errors::IndexedAddressError;
use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;

#[derive(Debug, PartialEq)]
pub struct IndexedAddress(pub [u8; 11], pub PaymentAddress);

impl IndexedAddress {
    pub fn new(diversifier_index: DiversifierIndex, payment_address: PaymentAddress) -> IndexedAddress {
        IndexedAddress(diversifier_index.0, payment_address)
    }
}

impl Serializable<Vec<u8>, SaplingError> for IndexedAddress {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        assert_byte_length(&serialized).map_err(SaplingError::caused_by)?;

        let diversifier_index = serialized[..11].try_into().unwrap();
        let address = PaymentAddress::deserialize(serialized[11..].to_vec())?;

        Ok(IndexedAddress(diversifier_index, address))
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];

        let address = &self.1.serialize()?;

        bytes.write_all(&self.0)
            .and_then(|_| bytes.write_all(address))
            .map_err(IndexedAddressError::SerializationFailed)
            .map_err(SaplingError::caused_by)?;

        Ok(bytes)
    }
}

fn assert_byte_length(bytes: &[u8]) -> Result<(), IndexedAddressError> {
    let len = bytes.len();
    assert_value_or_error(len == 43, IndexedAddressError::InvalidAddressLength(len))
}