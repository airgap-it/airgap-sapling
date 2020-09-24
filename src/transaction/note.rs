use zcash_primitives::primitives::{Note, PaymentAddress, Rseed};

use crate::common::errors::SaplingError;

pub fn create_note(payment_address: &PaymentAddress, value: u64, rcm: jubjub::Scalar) -> Result<Note, SaplingError> {
    let rseed = Rseed::BeforeZip212(rcm);
    payment_address.create_note(value, rseed).ok_or_else(SaplingError::new)
}