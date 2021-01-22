use crate::common::errors::SaplingError;
use crate::common::traits::Serializable;
use crate::common::utils::option_utils::ct_unwrap;
use group::GroupEncoding;

impl Serializable<Vec<u8>, SaplingError> for jubjub::Scalar {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        match serialized.len() {
            64 => {
                let mut bytes = [0u8; 64];
                bytes.copy_from_slice(&serialized[..]);
                Ok(jubjub::Scalar::from_bytes_wide(&bytes))
            },
            32 => {
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&serialized[..]);
                ct_unwrap(jubjub::Scalar::from_bytes(&bytes)).ok_or_else(SaplingError::new)
            },
            _ => Err(SaplingError::new())
        }
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.to_bytes().to_vec())
    }
}

impl Serializable<Vec<u8>, SaplingError> for bls12_381::Scalar {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        match serialized.len() {
            64 => {
                let mut bytes = [0u8; 64];
                bytes.copy_from_slice(&serialized[..]);
                Ok(bls12_381::Scalar::from_bytes_wide(&bytes))
            },
            32 => {
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&serialized[..]);
                ct_unwrap(bls12_381::Scalar::from_bytes(&bytes)).ok_or_else(SaplingError::new)
            },
            _ => Err(SaplingError::new())
        }
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.to_bytes().to_vec())
    }
}

impl Serializable<Vec<u8>, SaplingError> for jubjub::ExtendedPoint {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&serialized[..]);
        ct_unwrap(jubjub::ExtendedPoint::from_bytes(&bytes)).ok_or_else(SaplingError::new)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.to_bytes().to_vec())
    }
}

impl Serializable<Vec<u8>, SaplingError> for jubjub::SubgroupPoint {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&serialized[..]);
        ct_unwrap(jubjub::SubgroupPoint::from_bytes(&bytes)).ok_or_else(SaplingError::new)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.to_bytes().to_vec())
    }
}