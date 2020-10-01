use zcash_proofs::sapling::SaplingProvingContext;
use bls12_381::Bls12;
use bellman::groth16::Proof;
use crate::common::errors::SaplingError;
use crate::transaction::proof::prepare_proving_key;
use zcash_primitives::primitives::PaymentAddress;

pub fn create_output_proof(
    ctx: &mut SaplingProvingContext,
    esk: jubjub::Scalar,
    to: PaymentAddress,
    rcm: jubjub::Scalar,
    value: u64,
    proving_key: &[u8]
) -> Result<(Proof<Bls12>, jubjub::ExtendedPoint), SaplingError> {
    let proving_key = prepare_proving_key(proving_key)?;

    Ok(ctx.output_proof(esk, to, rcm, value, &proving_key))
}