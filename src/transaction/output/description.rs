use std::convert::TryInto;

use bellman::groth16::{Parameters, Proof};
use bls12_381::Bls12;
use rand_core::OsRng;
use zcash_primitives::keys::{FullViewingKey, OutgoingViewingKey};
use zcash_primitives::note_encryption::{Memo, SaplingNoteEncryption};
use zcash_primitives::primitives::{Note, PaymentAddress, Rseed};
use zcash_primitives::transaction::components::{GROTH_PROOF_SIZE, OutputDescription};
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::output::errors::OutputDescriptionError;
use crate::transaction::rand::{generate_rand_bytes, generate_rand_scalar};

impl Serializable<Vec<u8>, SaplingError> for OutputDescription {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        OutputDescription::read(&mut &serialized[..]).map_err(OutputDescriptionError::ReadFailed).map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(OutputDescriptionError::WriteFailed).map_err(SaplingError::caused_by)?;

        Ok(bytes)
    }
}

pub fn create_output_description(
    ctx: *mut SaplingProvingContext,
    ovk: Option<OutgoingViewingKey>,
    to: PaymentAddress,
    value: u64,
    memo: Option<&[u8]>,
    proving_key: &[u8]
) -> Result<OutputDescription, SaplingError> {
    let ctx = unsafe { &mut *ctx };
    let mut rng = OsRng;

    let rcm = generate_rand_scalar(Some(&mut rng));

    let ovk = prepare_ovk(ovk)?;
    let note = create_note(&to, value, rcm)?;
    let memo = get_memo(memo);

    let encryptor = SaplingNoteEncryption::new(
        ovk,
        note.clone(),
        to.clone(),
        memo,
        &mut rng,
    );

    let proving_key = prepare_proving_key(proving_key)?;

    let (proof, cv) = ctx.output_proof(*encryptor.esk(), to, rcm, value, &proving_key);
    let cmu = note.cmu();
    let ephemeral_key = get_epk(&encryptor)?;
    let enc_ciphertext = encryptor.encrypt_note_plaintext();
    let out_ciphertext = encryptor.encrypt_outgoing_plaintext(&cv, &cmu);
    let zkproof = get_zkproof(proof)?;

    let output_description = OutputDescription {
        cv,
        cmu,
        ephemeral_key,
        enc_ciphertext,
        out_ciphertext,
        zkproof,
    };

    Ok(output_description)
}

fn prepare_proving_key(proving_key: &[u8]) -> Result<Parameters<Bls12>, SaplingError> {
    Parameters::read(proving_key, false).map_err(|_| SaplingError::new())
}

fn prepare_ovk(ovk: Option<OutgoingViewingKey>) -> Result<OutgoingViewingKey, SaplingError> {
    match ovk {
        Some(ovk) => Ok(ovk),
        None => {
            let ovk: [u8; 32] = generate_rand_bytes(32)[..32].try_into().map_err(|_| SaplingError::new())?;
            let ovk = OutgoingViewingKey(ovk);

            Ok(ovk)
        },
    }
}

fn create_note(address: &PaymentAddress, value: u64, rcm: jubjub::Scalar) -> Result<Note, SaplingError> {
    let rseed = Rseed::BeforeZip212(rcm);
    address.create_note(value, rseed).ok_or_else(SaplingError::new)
}

fn get_memo(memo: Option<&[u8]>) -> Memo {
    memo.and_then(|m| Memo::from_bytes(m)).unwrap_or_else(Memo::default)
}

fn get_epk(encryptor: &SaplingNoteEncryption) -> Result<jubjub::ExtendedPoint, SaplingError> {
    encryptor.epk().clone().try_into().map_err(|_| SaplingError::new())
}

fn get_zkproof(proof: Proof<Bls12>) -> Result<[u8; GROTH_PROOF_SIZE], SaplingError> {
    let mut zkproof = [0u8; GROTH_PROOF_SIZE];
    proof.write(&mut zkproof[..]).map_err(|_| SaplingError::new())?;

    Ok(zkproof)
}