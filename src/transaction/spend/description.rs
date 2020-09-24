use std::convert::TryInto;

use bellman::groth16::{Parameters, PreparedVerifyingKey, Proof, VerifyingKey};
use bls12_381::Bls12;
use zcash_primitives::keys::FullViewingKey;
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::{Diversifier, Note, PaymentAddress, ProofGenerationKey, Rseed, ViewingKey};
use zcash_primitives::sapling::Node;
use zcash_primitives::transaction::components::{GROTH_PROOF_SIZE, SpendDescription};
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::rand::generate_rand_scalar;
use crate::transaction::spend::errors::SpendDescriptionError;

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

pub fn create_spend_description(
    ctx: &mut SaplingProvingContext,
    xsk: ExtendedSpendingKey,
    payment_address: PaymentAddress,
    rcm: jubjub::Scalar,
    value: u64,
    anchor: bls12_381::Scalar,
    merkle_path: MerklePath<Node>,
    position: u64,
    proving_key: &[u8],
    verifying_key: &[u8]
) -> Result<SpendDescription, SaplingError> {
    let xfvk = ExtendedFullViewingKey::from(&xsk);

    let proof_generation_key = ProofGenerationKey {
        ak: xfvk.fvk.vk.ak.clone(),
        nsk: xsk.expsk.nsk.clone(),
    };

    let rseed = Rseed::BeforeZip212(rcm);
    let ar = generate_rand_scalar();

    let proving_key = prepare_proving_key(proving_key)?;
    let verifying_key = prepare_verifying_key(verifying_key)?;

    let (proof, cv, rk) = ctx.spend_proof(
        proof_generation_key,
        payment_address.diversifier().clone(),
        rseed,
        ar,
        value,
        anchor,
        merkle_path,
        &proving_key,
        &verifying_key
    ).map_err(|_| SaplingError::new())?;

    let nullifier = compute_nullifier(&xfvk.fvk.vk, &payment_address, value, rcm, position)?;
    let zkproof = get_zkproof(proof)?;

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

fn prepare_proving_key(proving_key: &[u8]) -> Result<Parameters<Bls12>, SaplingError> {
    Parameters::read(proving_key, false).map_err(|_| SaplingError::new())
}

fn prepare_verifying_key(verifying_key: &[u8]) -> Result<PreparedVerifyingKey<Bls12>, SaplingError> {
    let vk = VerifyingKey::<Bls12>::read(verifying_key).map_err(|_| SaplingError::new())?;

    Ok(bellman::groth16::prepare_verifying_key(&vk))
}

fn compute_nullifier(vk: &ViewingKey, payment_address: &PaymentAddress, value: u64, rcm: jubjub::Scalar, position: u64) -> Result<[u8; 32], SaplingError> {
    let note = create_note(payment_address, value, rcm)?;
    let nullifier: [u8; 32] = note.nf(vk, position).try_into().map_err(|_| SaplingError::new())?;

    Ok(nullifier)
}

fn create_note(payment_address: &PaymentAddress, value: u64, rcm: jubjub::Scalar) -> Result<Note, SaplingError> {
    let rseed = Rseed::BeforeZip212(rcm);
    payment_address.create_note(value, rseed).ok_or_else(SaplingError::new)
}

fn get_zkproof(proof: Proof<Bls12>) -> Result<[u8; GROTH_PROOF_SIZE], SaplingError> {
    let mut zkproof = [0u8; GROTH_PROOF_SIZE];
    proof.write(&mut zkproof[..]).map_err(|_| SaplingError::new())?;

    Ok(zkproof)
}