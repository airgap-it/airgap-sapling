use bellman::groth16::Proof;
use bls12_381::Bls12;
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::{PaymentAddress, Rseed};
use zcash_primitives::redjubjub::PublicKey;
use zcash_primitives::sapling::Node;
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::SaplingError;
use crate::transaction::proof::{prepare_proof_generation_key, prepare_proving_key, prepare_verifying_key};

pub fn create_spend_proof(
    ctx: &mut SaplingProvingContext,
    xsk: &ExtendedSpendingKey,
    payment_address: &PaymentAddress,
    rcm: jubjub::Scalar,
    ar: jubjub::Scalar,
    value: u64,
    anchor: bls12_381::Scalar,
    merkle_path: MerklePath<Node>,
    proving_key: &[u8],
    verifying_key: &[u8]
) -> Result<(Proof<Bls12>, jubjub::ExtendedPoint, PublicKey), SaplingError> {
    let proof_generation_key = prepare_proof_generation_key(xsk);

    let rseed = Rseed::BeforeZip212(rcm);

    let proving_key = prepare_proving_key(proving_key)?;
    let verifying_key = prepare_verifying_key(verifying_key)?;

    ctx.spend_proof(
        proof_generation_key,
        payment_address.diversifier().clone(),
        rseed,
        ar,
        value,
        anchor,
        merkle_path,
        &proving_key,
        &verifying_key
    ).map_err(|_| SaplingError::new())
}