use std::convert::TryInto;

use rand_core::{OsRng, RngCore};
use zcash_primitives::keys::OutgoingViewingKey;

use crate::common::errors::SaplingError;

pub fn rand_ovk() -> Result<OutgoingViewingKey, SaplingError> {
    let ovk: [u8; 32] = rand_bytes(32)[..32].try_into().map_err(|_| SaplingError::new())?;
    let ovk = OutgoingViewingKey(ovk);

    Ok(ovk)
}

pub fn rand_bytes(len: usize) -> Vec<u8> {
    let mut rng = OsRng;
    let mut buffer = vec![0u8; len];
    rng.fill_bytes(&mut buffer);
    
    buffer
}

pub fn rand_scalar() -> jubjub::Scalar {
    let mut buffer = [0u8; 64];

    let mut rng = OsRng;
    rng.fill_bytes(&mut buffer);

    jubjub::Scalar::from_bytes_wide(&buffer)
}

pub fn rand_scalar_bytes() -> [u8; 32] {
    rand_scalar().to_bytes()
}