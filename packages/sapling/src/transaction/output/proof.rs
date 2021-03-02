use bellman::groth16::{Parameters, Proof};
use bls12_381::Bls12;
use zcash_primitives::primitives::PaymentAddress;
use zcash_proofs::sapling::SaplingProvingContext;

pub struct OutputDetails {
    pub to_address: PaymentAddress,
    pub value: u64,
}

pub fn create_output_proof(
    ctx: &mut SaplingProvingContext,
    output_details: OutputDetails,
    esk: jubjub::Scalar,
    rcm: jubjub::Scalar,
    proving_key: &Parameters<Bls12>
) -> (Proof<Bls12>, jubjub::ExtendedPoint) {
    ctx.output_proof(esk, output_details.to_address, rcm, output_details.value, proving_key)
}