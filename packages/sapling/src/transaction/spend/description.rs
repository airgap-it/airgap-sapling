use std::io::{Read, Write};
use std::io;

use ff::PrimeField;
use group::GroupEncoding;
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::{Nullifier, PaymentAddress, ViewingKey};
use zcash_primitives::redjubjub::PublicKey;
use zcash_primitives::sapling::Node;
use zcash_primitives::transaction::components::{GROTH_PROOF_SIZE, SpendDescription};
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::note::create_note;
use crate::transaction::proof::prepare_zkproof;
use crate::transaction::signature::create_spend_sig;
use crate::transaction::spend::errors::SpendDescriptionError;
use crate::transaction::spend::proof::{create_spend_proof, SpendDetails, SpendParameters};

pub struct UnsignedSpendDescription {
    pub cv: jubjub::ExtendedPoint,
    pub anchor: bls12_381::Scalar,
    pub nullifier: Nullifier,
    pub rk: PublicKey,
    pub zkproof: [u8; GROTH_PROOF_SIZE],
}

// Based on [`SpendDescription`](https://github.com/zcash/librustzcash/blob/master/zcash_primitives/src/transaction/components.rs#L279)
impl UnsignedSpendDescription {
    pub fn read<R: Read>(mut reader: &mut R) -> io::Result<Self> {
        // Consensus rules (§4.4):
        // - Canonical encoding is enforced here.
        // - "Not small order" is enforced in SaplingVerificationContext::check_spend()
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

        // Consensus rule (§7.3): Canonical encoding is enforced here
        let anchor = {
            let mut f = [0u8; 32];
            reader.read_exact(&mut f)?;
            bls12_381::Scalar::from_repr(f)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "anchor not in field"))?
        };

        let mut nullifier = Nullifier([0u8; 32]);
        reader.read_exact(&mut nullifier.0)?;

        // Consensus rules (§4.4):
        // - Canonical encoding is enforced here.
        // - "Not small order" is enforced in SaplingVerificationContext::check_spend()
        let rk = PublicKey::read(&mut reader)?;

        // Consensus rules (§4.4):
        // - Canonical encoding is enforced by the API of SaplingVerificationContext::check_spend()
        //   due to the need to parse this into a bellman::groth16::Proof.
        // - Proof validity is enforced in SaplingVerificationContext::check_spend()
        let mut zkproof = [0u8; GROTH_PROOF_SIZE];
        reader.read_exact(&mut zkproof)?;

        Ok(UnsignedSpendDescription {
            cv,
            anchor,
            nullifier,
            rk,
            zkproof,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.cv.to_bytes())?;
        writer.write_all(self.anchor.to_repr().as_ref())?;
        writer.write_all(&self.nullifier.0)?;
        self.rk.write(&mut writer)?;
        writer.write_all(&self.zkproof)
    }
}

impl Serializable<Vec<u8>, SaplingError> for UnsignedSpendDescription {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        UnsignedSpendDescription::read(&mut &serialized[..]).map_err(SpendDescriptionError::ReadFailed).map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(SpendDescriptionError::WriteFailed).map_err(SaplingError::caused_by)?;

        Ok(bytes)
    }
}

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
    spend_details: SpendDetails,
    rcm: jubjub::Scalar,
    ar: jubjub::Scalar,
    anchor: bls12_381::Scalar,
    merkle_path: MerklePath<Node>,
    parameters: SpendParameters
) -> Result<UnsignedSpendDescription, SaplingError> {
    let vk = &spend_details.from_pak.to_viewing_key();
    let nullifier = compute_nullifier(
        &vk,
        &spend_details.to_address,
        spend_details.value,
        rcm,
        merkle_path.position
    )?;

    let (proof, cv, rk) = create_spend_proof(
        ctx,
        &spend_details,
        rcm,
        ar,
        anchor,
        merkle_path,
        &parameters
    )?;

    let zkproof = prepare_zkproof(proof)?;

    let spend_description = UnsignedSpendDescription {
        cv,
        anchor,
        nullifier,
        rk,
        zkproof,
    };

    Ok(spend_description)
}

pub fn sign_spend_description(spend_description: UnsignedSpendDescription, xsk: ExtendedSpendingKey, ar: jubjub::Scalar, sighash: [u8; 32]) -> Result<SpendDescription, SaplingError> {
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

pub fn compute_nullifier(vk: &ViewingKey, payment_address: &PaymentAddress, value: u64, rcm: jubjub::Scalar, position: u64) -> Result<Nullifier, SaplingError> {
    let note = create_note(payment_address, value, rcm)?;
    let nullifier = note.nf(vk, position);

    Ok(nullifier)
}