use std::convert::TryInto;
use std::io::Write;

use zcash_primitives::primitives::PaymentAddress;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;

use super::errors::SaplingAddressError;

#[derive(Debug, PartialEq)]
pub struct SaplingAddress {
    pub diversifier: [u8; 11],
    pub pkd: [u8; 32],
}

impl SaplingAddress {
    pub fn from(payment_address: PaymentAddress) -> SaplingAddress {
        let bytes = payment_address.to_bytes();

        let diversifier: [u8; 11] = bytes[..11].try_into().unwrap();
        let pkd: [u8; 32] = bytes[11..].try_into().unwrap();

        SaplingAddress {
            diversifier,
            pkd,
        }
    }
}

impl Serializable<Vec<u8>, SaplingError> for SaplingAddress {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        assert_byte_length(&serialized).map_err(SaplingError::caused_by)?;

        let address = SaplingAddress {
            diversifier: serialized[..11].try_into().unwrap(),
            pkd: serialized[11..].try_into().unwrap(),
        };

        Ok(address)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];

        bytes.write_all(&self.diversifier)
            .and_then(|_| bytes.write_all(&self.pkd))
            .map_err(SaplingAddressError::SerializationFailed)
            .map_err(SaplingError::caused_by)?;

        Ok(bytes)
    }
}

fn assert_byte_length(bytes: &[u8]) -> Result<(), SaplingAddressError> {
    let len = bytes.len();
    assert_value_or_error(len == 43, SaplingAddressError::InvalidAddressLength(len))
}