use bellman::groth16::{Parameters, Proof};
use bls12_381::Bls12;
use zcash_primitives::primitives::PaymentAddress;
use zcash_proofs::sapling::SaplingProvingContext;

pub fn create_output_proof(
    ctx: &mut SaplingProvingContext,
    esk: jubjub::Scalar,
    to: PaymentAddress,
    rcm: jubjub::Scalar,
    value: u64,
    proving_key: &Parameters<Bls12>
) -> (Proof<Bls12>, jubjub::ExtendedPoint) {
    ctx.output_proof(esk, to, rcm, value, proving_key)
}