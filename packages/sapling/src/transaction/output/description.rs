use std::convert::TryInto;

use rand_core::OsRng;
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::note_encryption::{Memo, SaplingNoteEncryption};
use zcash_primitives::primitives::{Note, PaymentAddress, Diversifier};
use zcash_primitives::transaction::components::{OutputDescription, GROTH_PROOF_SIZE};
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::note::create_note;
use crate::transaction::output::errors::OutputDescriptionError;
use crate::transaction::output::proof::create_output_proof;
use crate::transaction::proof::prepare_zkproof;
use bellman::groth16::Parameters;
use bls12_381::Bls12;
use crate::transaction::output::OutputDetails;
use std::io::{Read, Write};
use std::io;
use group::GroupEncoding;
use ff::PrimeField;

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

pub struct PartialOutputDescription {
    pub cv: jubjub::ExtendedPoint,
    pub cmu: bls12_381::Scalar,
    pub zkproof: [u8; GROTH_PROOF_SIZE],
}

// Based on [`OutputDescription`](https://github.com/zcash/librustzcash/blob/master/zcash_primitives/src/transaction/components.rs#L369)
impl PartialOutputDescription {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        // Consensus rules (§4.5):
        // - Canonical encoding is enforced here.
        // - "Not small order" is enforced in SaplingVerificationContext::check_output()
        //   (located in zcash_proofs::sapling::verifier).
        let cv = {
            let mut bytes = [0u8; 32];
            reader.read_exact(&mut bytes)?;
            let cv = jubjub::ExtendedPoint::from_bytes(&bytes);
            if cv.is_none().into() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid cv"));
            }
            cv.unwrap()
        };

        // Consensus rule (§7.4): Canonical encoding is enforced here
        let cmu = {
            let mut f = [0u8; 32];
            reader.read_exact(&mut f)?;
            bls12_381::Scalar::from_repr(f)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "cmu not in field"))?
        };

        // Consensus rules (§4.5):
        // - Canonical encoding is enforced by the API of SaplingVerificationContext::check_output()
        //   due to the need to parse this into a bellman::groth16::Proof.
        // - Proof validity is enforced in SaplingVerificationContext::check_output()
        let mut zkproof = [0u8; GROTH_PROOF_SIZE];
        reader.read_exact(&mut zkproof)?;

        Ok(PartialOutputDescription {
            cv,
            cmu,
            zkproof,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.cv.to_bytes())?;
        writer.write_all(self.cmu.to_repr().as_ref())?;
        writer.write_all(&self.zkproof)
    }
}

impl Serializable<Vec<u8>, SaplingError> for PartialOutputDescription {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        PartialOutputDescription::read(&mut &serialized[..]).map_err(OutputDescriptionError::ReadFailed).map_err(SaplingError::caused_by)
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
    output_details: OutputDetails,
    rcm: jubjub::Scalar,
    memo: Option<&[u8]>,
    proving_key: &Parameters<Bls12>
) -> Result<OutputDescription, SaplingError> {
    let note = create_note(&output_details.to_address, output_details.value, rcm)?;
    let memo = get_memo(memo);

    let mut encryptor = create_encryptor(ovk, &note, &output_details.to_address, memo)?;

    let (proof, cv) = create_output_proof(ctx, output_details, *encryptor.esk(), rcm, proving_key);
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

pub fn prepare_partial_output_description(
    ctx: &mut SaplingProvingContext,
    output_details: OutputDetails,
    rcm: jubjub::Scalar,
    esk: jubjub::Scalar,
    proving_key: &Parameters<Bls12>
) -> Result<PartialOutputDescription, SaplingError> {
    let note = create_note(&output_details.to_address, output_details.value, rcm)?;

    let (proof, cv) = create_output_proof(ctx, output_details, esk, rcm, proving_key);
    let cmu = note.cmu();

    let zkproof = prepare_zkproof(proof)?;

    let output_description = PartialOutputDescription {
        cv,
        cmu,
        zkproof,
    };

    Ok(output_description)
}

pub fn derive_epk(diversifier: Diversifier, esk: jubjub::Scalar) -> Result<jubjub::SubgroupPoint, SaplingError> {
    let g_d = diversifier.g_d().ok_or_else(SaplingError::new)?;
    let epk = g_d * esk;

    Ok(epk)
}

fn get_memo(memo: Option<&[u8]>) -> Memo {
    memo.and_then(|m| Memo::from_bytes(m)).unwrap_or_else(Memo::default)
}

fn create_encryptor(ovk: OutgoingViewingKey, note: &Note, to: &PaymentAddress, memo: Memo) -> Result<SaplingNoteEncryption<OsRng>, SaplingError> {
    let rng = OsRng;
    let encryptor = SaplingNoteEncryption::new(
        Some(ovk),
        note.clone(),
        to.clone(),
        memo,
        rng,
    );

    Ok(encryptor)
}

fn get_epk(encryptor: &SaplingNoteEncryption<OsRng>) -> Result<jubjub::ExtendedPoint, SaplingError> {
    encryptor.epk().clone().try_into().map_err(|_| SaplingError::new())
}