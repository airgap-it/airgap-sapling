use bellman::groth16::{Proof, Parameters, PreparedVerifyingKey};
use bls12_381::Bls12;
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::{PaymentAddress, Rseed};
use zcash_primitives::redjubjub::PublicKey;
use zcash_primitives::sapling::Node;
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::transaction::proof::prepare_proof_generation_key;
use crate::transaction::spend::errors::SpendDescriptionError;

pub fn create_spend_proof(
    ctx: &mut SaplingProvingContext,
    xsk: &ExtendedSpendingKey,
    payment_address: &PaymentAddress,
    rcm: jubjub::Scalar,
    ar: jubjub::Scalar,
    value: u64,
    anchor: bls12_381::Scalar,
    merkle_path: MerklePath<Node>,
    proving_key: &Parameters<Bls12>,
    verifying_key: &PreparedVerifyingKey<Bls12>
) -> Result<(Proof<Bls12>, jubjub::ExtendedPoint, PublicKey), SaplingError> {
    let proof_generation_key = prepare_proof_generation_key(xsk);

    let rseed = Rseed::BeforeZip212(rcm);

    ctx.spend_proof(
        proof_generation_key,
        *payment_address.diversifier(),
        rseed,
        ar,
        value,
        anchor,
        merkle_path,
        proving_key,
        verifying_key
    ).map_err(|_| SpendDescriptionError::CreateSpendProofFailed).map_err(SaplingError::caused_by)
}