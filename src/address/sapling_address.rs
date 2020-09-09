use std::convert::TryInto;
use std::io::Write;

use zcash_primitives::primitives::PaymentAddress;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;

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

impl Serializable<SaplingError> for SaplingAddress {
    fn from_bytes(bytes: &[u8]) -> Result<Self, SaplingError> {
        assert_byte_length(bytes).map_err(SaplingError::caused_by)?;

        let address = SaplingAddress {
            diversifier: bytes[..11].try_into().unwrap(),
            pkd: bytes[11..].try_into().unwrap(),
        };

        Ok(address)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, SaplingError> {
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
    if len != 43 {
        Err(SaplingAddressError::InvalidAddressLength(len))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// TODO
}