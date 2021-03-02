use bellman::groth16::{Parameters, PreparedVerifyingKey, Proof};
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

pub struct SpendDetails {
    pub from_xsk: ExtendedSpendingKey,
    pub to_address: PaymentAddress,
    pub value: u64,
}

pub struct SpendParameters<'a> {
    pub proving_key: &'a Parameters<Bls12>,
    pub verifying_key: &'a PreparedVerifyingKey<Bls12>,
}

pub fn create_spend_proof(
    ctx: &mut SaplingProvingContext,
    spend_details: &SpendDetails,
    rcm: jubjub::Scalar,
    ar: jubjub::Scalar,
    anchor: bls12_381::Scalar,
    merkle_path: MerklePath<Node>,
    parameters: &SpendParameters
) -> Result<(Proof<Bls12>, jubjub::ExtendedPoint, PublicKey), SaplingError> {
    let proof_generation_key = prepare_proof_generation_key(&spend_details.from_xsk);

    let rseed = Rseed::BeforeZip212(rcm);

    ctx.spend_proof(
        proof_generation_key,
        *spend_details.to_address.diversifier(),
        rseed,
        ar,
        spend_details.value,
        anchor,
        merkle_path,
        parameters.proving_key,
        parameters.verifying_key
    ).map_err(|_| SpendDescriptionError::CreateSpendProofFailed).map_err(SaplingError::caused_by)
}