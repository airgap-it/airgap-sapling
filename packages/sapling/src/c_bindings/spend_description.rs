use std::convert::TryInto;

use bellman::groth16::{Parameters, PreparedVerifyingKey};
use bls12_381::Bls12;
use libc::{c_uchar, size_t};
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::sapling::Node;
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;
use zcash_proofs::ZcashParameters;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::utils::c_utils::{c_dereference, c_deserialize, c_deserialize_slice, c_serialize_res, c_ptr_catch_result};
use crate::State;
use crate::transaction::{prepare_spend_description, sign_spend_description, SpendDetails, SpendParameters, UnsignedSpendDescription};

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "C" fn c_spend_description_from_xsk(
    ctx: *mut SaplingProvingContext,
    xsk: *const c_uchar,
    xsk_len: size_t,
    address: *const c_uchar,
    address_len: size_t,
    rcm: *const c_uchar,
    rcm_len: size_t,
    ar: *const c_uchar,
    ar_len: size_t,
    value: u64,
    anchor: *const c_uchar,
    anchor_len: size_t,
    merkle_path: *const c_uchar,
    merkle_path_len: size_t,
    description_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let xsk: ExtendedSpendingKey = unsafe { c_deserialize(xsk, xsk_len) }?;
        let payment_address: PaymentAddress = unsafe { c_deserialize(address, address_len) }?;
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len) }?;
        let ar: jubjub::Scalar = unsafe { c_deserialize(ar, ar_len) }?;
        let anchor: bls12_381::Scalar = unsafe { c_deserialize(anchor, anchor_len) }?;
        let merkle_path: MerklePath<Node> = unsafe { c_deserialize(merkle_path, merkle_path_len) }?;

        let ctx: &mut SaplingProvingContext = unsafe { c_dereference(ctx) };

        let params: &ZcashParameters = State::proof_params()?;
        let proving_key: &Parameters<Bls12> = &params.spend_params;
        let verifying_key: &PreparedVerifyingKey<Bls12> = &params.spend_vk;

        let spend_description = prepare_spend_description(
            ctx,
            SpendDetails { from_xsk: xsk, to_address: payment_address, value },
            rcm,
            ar,
            anchor,
            merkle_path,
            SpendParameters { proving_key, verifying_key },
        );

        unsafe { c_serialize_res(spend_description, description_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_sign_spend_description_with_xsk(
    spend_description: *const c_uchar,
    spend_description_len: size_t,
    xsk: *const c_uchar,
    xsk_len: size_t,
    ar: *const c_uchar,
    ar_len: size_t,
    sighash: *const c_uchar,
    sighash_len: size_t,
    description_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let spend_description: UnsignedSpendDescription = unsafe { c_deserialize(spend_description, spend_description_len) }?;
        let xks: ExtendedSpendingKey = unsafe { c_deserialize(xsk, xsk_len) }?;
        let ar: jubjub::Scalar = unsafe { c_deserialize(ar, ar_len) }?;

        let sighash: [u8; 32] = unsafe { c_deserialize_slice(sighash, sighash_len) }.try_into()
            .map_err(|_| SaplingError::caused_by("signSpendDescriptionWithXsk: sighash must be an array of 32 bytes"))?;

        let spend_description = sign_spend_description(spend_description, xks, ar, sighash);

        unsafe { c_serialize_res(spend_description, description_len) }
    })
}