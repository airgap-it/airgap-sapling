use bellman::groth16::{Proof, Parameters, PreparedVerifyingKey, VerifyingKey};
use bls12_381::Bls12;
use zcash_primitives::primitives::{Rseed, ProofGenerationKey};
use zcash_primitives::zip32::{ExtendedSpendingKey, ExtendedFullViewingKey};

use crate::common::errors::SaplingError;
use zcash_primitives::transaction::components::GROTH_PROOF_SIZE;

pub fn prepare_zkproof(proof: Proof<Bls12>) -> Result<[u8; GROTH_PROOF_SIZE], SaplingError> {
    let mut zkproof = [0u8; GROTH_PROOF_SIZE];
    proof.write(&mut zkproof[..]).map_err(|_| SaplingError::new())?;

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
    Parameters::read(proving_key, false).map_err(|_| SaplingError::new())
}

pub fn prepare_verifying_key(verifying_key: &[u8]) -> Result<PreparedVerifyingKey<Bls12>, SaplingError> {
    let vk = VerifyingKey::<Bls12>::read(verifying_key).map_err(|_| SaplingError::new())?;

    Ok(bellman::groth16::prepare_verifying_key(&vk))
}