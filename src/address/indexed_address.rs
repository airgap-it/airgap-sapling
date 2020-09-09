use std::convert::TryInto;
use std::io::Write;

use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::DiversifierIndex;

use crate::address::errors::IndexedAddressError;
use crate::address::SaplingAddress;
use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;

#[derive(Debug)]
pub struct IndexedAddress(pub [u8; 11], pub SaplingAddress);

impl IndexedAddress {
    pub fn new(diversifier_index: DiversifierIndex, payment_address: PaymentAddress) -> IndexedAddress {
        IndexedAddress(diversifier_index.0, SaplingAddress::from(payment_address))
    }
}

impl Serializable<SaplingError> for IndexedAddress {
    fn from_bytes(bytes: &[u8]) -> Result<Self, SaplingError> {
        assert_byte_length(bytes).map_err(SaplingError::caused_by)?;

        let diversifier_index = bytes[..11].try_into().unwrap();
        let address = SaplingAddress::from_bytes(&bytes[11..])?;

        Ok(IndexedAddress(diversifier_index, address))
    }

    fn to_bytes(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];

        let address = &self.1.to_bytes()?;

        bytes.write_all(&self.0)
            .and_then(|_| bytes.write_all(address))
            .map_err(IndexedAddressError::SerializationFailed)
            .map_err(SaplingError::caused_by)?;

        Ok(bytes)
    }
}

fn assert_byte_length(bytes: &[u8]) -> Result<(), IndexedAddressError> {
    let len = bytes.len();
    if len != 43 {
        Err(IndexedAddressError::InvalidAddressLength(len))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// TODO
}