use std::convert::TryInto;

use jubjub::Scalar;
use rand_core::{OsRng, RngCore};

use crate::common::errors::SaplingError;
use crate::common::traits::Serializable;
use crate::common::utils::assert_utils::assert_value_or_error;
use crate::common::utils::option_utils::ct_unwrap;

impl Serializable<Vec<u8>, SaplingError> for Scalar {
    fn deserialize(serialized: Vec<u8>) -> Result<Scalar, SaplingError> {
        assert_value_or_error(serialized.len() == 32 || serialized.len() == 64, SaplingError::new())?;

        let scalar = if serialized.len() == 32 {
            let fr = Scalar::from_bytes(&serialized[..32].try_into().unwrap());
            ct_unwrap(fr).ok_or_else(SaplingError::new)
        } else {
            let mut buffer = [0u8; 64];
            buffer.copy_from_slice(&serialized);

            Ok(Scalar::from_bytes_wide(&buffer))
        }?;

        Ok(scalar)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Ok(self.to_bytes().to_vec())
    }
}

pub fn generate_random_scalar() -> Result<Scalar, SaplingError> {
    let mut rng = OsRng;
    let mut buffer = [0u8; 64];
    rng.fill_bytes(&mut buffer);

    let fr = Scalar::from_bytes_wide(&buffer);

    Ok(fr)
}