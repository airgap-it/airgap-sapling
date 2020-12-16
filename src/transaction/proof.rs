use bellman::groth16::{Parameters, PreparedVerifyingKey, Proof};
use bls12_381::Bls12;
use zcash_primitives::primitives::ProofGenerationKey;
use zcash_primitives::transaction::components::GROTH_PROOF_SIZE;
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};
use zcash_proofs::parse_parameters;

use crate::common::errors::{CausedBy, SaplingError};
use crate::transaction::errors::ProofError;

pub struct ProofParams {
    pub spend_params: Parameters<Bls12>,
    pub spend_vk: PreparedVerifyingKey<Bls12>,
    pub output_params: Parameters<Bls12>,
    pub output_vk: PreparedVerifyingKey<Bls12>,
}

pub fn parse_params(spend_params: &[u8], output_params: &[u8]) -> ProofParams {
    let (
        spend_params,
        spend_vk,
        output_params,
        output_vk,
        _,
    ) = parse_parameters(spend_params, output_params, None);

    ProofParams {
        spend_params,
        spend_vk,
        output_params,
        output_vk,
    }
}

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
        ak: xfvk.fvk.vk.ak,
        nsk: xsk.expsk.nsk,
    }
}