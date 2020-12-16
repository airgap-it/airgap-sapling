use std::convert::TryInto;

use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::{PaymentAddress, ViewingKey};
use zcash_primitives::sapling::Node;
use zcash_primitives::transaction::components::SpendDescription;
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::note::create_note;
use crate::transaction::proof::prepare_zkproof;
use crate::transaction::signature::create_spend_sig;
use crate::transaction::spend::errors::SpendDescriptionError;
use crate::transaction::spend::proof::create_spend_proof;
use bellman::groth16::{Parameters, PreparedVerifyingKey};
use bls12_381::Bls12;

impl Serializable<Vec<u8>, SaplingError> for SpendDescription {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        SpendDescription::read(&mut &serialized[..]).map_err(SpendDescriptionError::ReadFailed).map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(SpendDescriptionError::WriteFailed).map_err(SaplingError::caused_by)?;
        
        Ok(bytes)
    }
}

pub fn prepare_spend_description(
    ctx: &mut SaplingProvingContext,
    xsk: ExtendedSpendingKey,
    payment_address: PaymentAddress,
    rcm: jubjub::Scalar,
    ar: jubjub::Scalar,
    value: u64,
    anchor: bls12_381::Scalar,
    merkle_path: MerklePath<Node>,
    proving_key: &Parameters<Bls12>,
    verifying_key: &PreparedVerifyingKey<Bls12>
) -> Result<SpendDescription, SaplingError> {
    let xfvk = ExtendedFullViewingKey::from(&xsk);
    let nullifier = compute_nullifier(&xfvk.fvk.vk, &payment_address, value, rcm, merkle_path.position)?;

    let (proof, cv, rk) = create_spend_proof(
        ctx,
        &xsk,
        &payment_address,
        rcm,
        ar,
        value,
        anchor,
        merkle_path,
        proving_key,
        verifying_key
    )?;

    let zkproof = prepare_zkproof(proof)?;

    let spend_description = SpendDescription {
        cv,
        anchor,
        nullifier,
        rk,
        zkproof,
        spend_auth_sig: None
    };

    Ok(spend_description)
}

pub fn sign_spend_description(spend_description: SpendDescription, xsk: ExtendedSpendingKey, ar: jubjub::Scalar, sighash: [u8; 32]) -> Result<SpendDescription, SaplingError> {
    let spend_sig = create_spend_sig(&xsk, ar, sighash)?;

    let spend_description = SpendDescription {
        cv: spend_description.cv,
        anchor: spend_description.anchor,
        nullifier: spend_description.nullifier,
        rk: spend_description.rk,
        zkproof: spend_description.zkproof,
        spend_auth_sig: Some(spend_sig)
    };

    Ok(spend_description)
}

fn compute_nullifier(vk: &ViewingKey, payment_address: &PaymentAddress, value: u64, rcm: jubjub::Scalar, position: u64) -> Result<[u8; 32], SaplingError> {
    let note = create_note(payment_address, value, rcm)?;
    let nullifier: [u8; 32] = note.nf(vk, position)[..32].try_into().unwrap();

    Ok(nullifier)
}