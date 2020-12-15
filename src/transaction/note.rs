use zcash_primitives::primitives::{Note, PaymentAddress, Rseed};

use crate::common::errors::{SaplingError, CausedBy};
use crate::transaction::errors::NoteError;

pub fn create_note(payment_address: &PaymentAddress, value: u64, rcm: jubjub::Scalar) -> Result<Note, SaplingError> {
    let rseed = Rseed::BeforeZip212(rcm);
    payment_address.create_note(value, rseed).ok_or_else(|| NoteError::NoteEmpty).map_err(SaplingError::caused_by)
}