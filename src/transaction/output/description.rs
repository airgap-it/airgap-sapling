use std::convert::TryInto;

use rand_core::OsRng;
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::note_encryption::{Memo, SaplingNoteEncryption};
use zcash_primitives::primitives::{Note, PaymentAddress};
use zcash_primitives::transaction::components::OutputDescription;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::note::create_note;
use crate::transaction::output::errors::OutputDescriptionError;
use crate::transaction::output::proof::create_output_proof;
use crate::transaction::proof::prepare_zkproof;
use group::GroupEncoding;

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

pub fn prepare_output_description(
    ctx: &mut SaplingProvingContext,
    ovk: OutgoingViewingKey,
    to: PaymentAddress,
    rcm: jubjub::Scalar,
    value: u64,
    memo: Option<&[u8]>,
    proving_key: &[u8]
) -> Result<OutputDescription, SaplingError> {
    let note = create_note(&to, value, rcm)?;
    let memo = get_memo(memo);

    let encryptor = create_encryptor(ovk, &note, &to, memo)?;

    let (proof, cv) = create_output_proof(ctx, *encryptor.esk(), to, rcm, value, proving_key)?;
    let cmu = note.cmu();
    let ephemeral_key = get_epk(&encryptor)?;

    let enc_ciphertext = encryptor.encrypt_note_plaintext();
    let out_ciphertext = encryptor.encrypt_outgoing_plaintext(&cv, &cmu);
    let zkproof = prepare_zkproof(proof)?;

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

fn get_memo(memo: Option<&[u8]>) -> Memo {
    memo.and_then(|m| Memo::from_bytes(m)).unwrap_or_else(Memo::default)
}

fn create_encryptor(ovk: OutgoingViewingKey, note: &Note, to: &PaymentAddress, memo: Memo) -> Result<SaplingNoteEncryption, SaplingError> {
    let mut rng = OsRng;

    let encryptor = SaplingNoteEncryption::new(
        ovk,
        note.clone(),
        to.clone(),
        memo,
        &mut rng,
    );

    Ok(encryptor)
}

fn get_epk(encryptor: &SaplingNoteEncryption) -> Result<jubjub::ExtendedPoint, SaplingError> {
    encryptor.epk().clone().try_into().map_err(|_| SaplingError::new())
}