use bellman::groth16::{Parameters, PreparedVerifyingKey, Proof, VerifyingKey};
use bls12_381::Bls12;
use zcash_primitives::primitives::{ProofGenerationKey};
use zcash_primitives::transaction::components::GROTH_PROOF_SIZE;
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};

use crate::common::errors::{CausedBy, SaplingError};
use crate::transaction::errors::ProofError;

pub fn prepare_zkproof(proof: Proof<Bls12>) -> Result<[u8; GROTH_PROOF_SIZE], SaplingError> {
    let mut zkproof = [0u8; GROTH_PROOF_SIZE];
    proof.write(&mut zkproof[..])
        .map_err(ProofError::WriteFailed)
        .map_err(SaplingError::caused_by)?;

    Ok(zkproof)
}

pub fn prepare_proof_generation_key(xsk: &ExtendedSpendingKey) -> ProofGenerationKey {
    let xfvk = ExtendedFullViewingKey::from(xsk);

    ProofGenerationKey {
        ak: xfvk.fvk.vk.ak.clone(),
        nsk: xsk.expsk.nsk.clone(),
    }
}

pub fn prepare_proving_key(proving_key: &[u8]) -> Result<Parameters<Bls12>, SaplingError> {
    Parameters::read(proving_key, false)
        .map_err(ProofError::ReadFailed)
        .map_err(SaplingError::caused_by)
}

pub fn prepare_verifying_key(verifying_key: &[u8]) -> Result<PreparedVerifyingKey<Bls12>, SaplingError> {
    let vk = VerifyingKey::<Bls12>::read(verifying_key)
        .map_err(ProofError::ReadFailed)
        .map_err(SaplingError::caused_by)?;

    Ok(bellman::groth16::prepare_verifying_key(&vk))
}