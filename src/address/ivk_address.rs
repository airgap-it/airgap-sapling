use zcash_primitives::primitives::{PaymentAddress, Diversifier};
use crate::common::errors::{SaplingError, CausedBy};

pub fn get_ivk_address(ivk: jubjub::Scalar, diversifier: Diversifier) -> Result<PaymentAddress, SaplingError> {
    let g_d = diversifier.g_d().ok_or_else(|| SaplingError::caused_by("invalid diversifier"))?;
    let pk_d = g_d * ivk;

    PaymentAddress::from_parts(diversifier, pk_d).ok_or_else(|| SaplingError::caused_by("invalid pk_d"))
}