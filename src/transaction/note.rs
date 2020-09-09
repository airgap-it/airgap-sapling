use group::cofactor::CofactorGroup;
use group::GroupEncoding;
use jubjub::{ExtendedPoint, Scalar};
use zcash_primitives::primitives::{Diversifier, Note, Rseed};

use crate::address::SaplingAddress;
use crate::common::errors::SaplingError;
use crate::common::utils::option_utils::ct_unwrap;

pub fn create_note(address: &SaplingAddress, value: u64, rcm: &[u8; 32]) -> Result<Note, SaplingError> {
    let diversifier = Diversifier(address.diversifier);
    let g_d = diversifier.g_d().ok_or_else(SaplingError::new)?;

    let pk_d = ExtendedPoint::from_bytes(&address.pkd).and_then(|pk_d| pk_d.into_subgroup());
    let pk_d = ct_unwrap(pk_d).ok_or_else(SaplingError::new)?;

    let rseed = Scalar::from_bytes(rcm).map(Rseed::BeforeZip212);
    let rseed = ct_unwrap(rseed).ok_or_else(SaplingError::new)?;

    let note = Note {
        value,
        g_d,
        pk_d,
        rseed
    };

    Ok(note)
}