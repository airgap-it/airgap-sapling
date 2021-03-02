use rand_core::OsRng;
use zcash_primitives::redjubjub::{PrivateKey, Signature};
use zcash_primitives::sapling::spend_sig;
use zcash_primitives::transaction::components::Amount;
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::errors::SignatureError;

impl Serializable<Vec<u8>, SaplingError> for Signature {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        Signature::read(&serialized[..]).map_err(SignatureError::ReadFailed).map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(SignatureError::WriteFailed).map_err(SaplingError::caused_by)?;
        
        Ok(bytes)
    }
}

pub fn create_spend_sig(xsk: &ExtendedSpendingKey, ar: jubjub::Scalar, sighash: [u8; 32]) -> Result<Signature, SaplingError> {
    let mut rng = OsRng;
    let ask = PrivateKey::read(&xsk.expsk.ask.to_bytes()[..])
        .map_err(SignatureError::PrivateKeyReadFailed)
        .map_err(SaplingError::caused_by)?;
    let signature = spend_sig(ask, ar, &sighash, &mut rng);

    Ok(signature)
}

pub fn create_binding_sig(
    ctx: &mut SaplingProvingContext,
    value_balance: i64,
    sighash: [u8; 32]
) -> Result<Signature, SaplingError> {
    let value_balance = get_amount(value_balance)?;

    ctx.binding_sig(value_balance, &sighash)
        .map_err(|_| SignatureError::ValueBalanceInvalid)
        .map_err(SaplingError::caused_by)
}

fn get_amount(balance: i64) -> Result<Amount, SaplingError> {
    Amount::from_i64(balance).map_err(|_| SignatureError::ValueBalanceOutsideRange).map_err(SaplingError::caused_by)
}