use std::convert::TryInto;

use bellman::groth16::Parameters;
use bls12_381::Bls12;
use libc::{c_uchar, size_t};
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::primitives::{Diversifier, PaymentAddress};
use zcash_primitives::zip32::ExtendedFullViewingKey;
use zcash_proofs::sapling::SaplingProvingContext;
use zcash_proofs::ZcashParameters;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::utils::c_utils::{c_dereference, c_deserialize, c_deserialize_slice, c_serialize_res, c_ptr_catch_result};
use crate::{c_init_lib, State};
use crate::transaction::{derive_epk, OutputDetails, prepare_output_description, prepare_partial_output_description};

#[no_mangle]
pub extern "C" fn c_output_description_from_xfvk(
    ctx: *mut SaplingProvingContext,
    xfvk: *const c_uchar,
    xfvk_len: size_t,
    to: *const c_uchar,
    to_len: size_t,
    rcm: *const c_uchar,
    rcm_len: size_t,
    value: u64,
    description_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let address: PaymentAddress = unsafe { c_deserialize(to, to_len) }?;
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len) }?;

        let ctx: &mut SaplingProvingContext = unsafe { c_dereference(ctx) };

        let params: &ZcashParameters = State::proof_params()?;
        let proving_key: &Parameters<Bls12> = &params.output_params;

        let output_description = prepare_output_description(
            ctx,
            xfvk.fvk.ovk,
            OutputDetails { to_address: address, value },
            rcm,
            None,
            proving_key,
        );

        unsafe { c_serialize_res(output_description, description_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_output_description_from_xfvk_with_memo(
    ctx: *mut SaplingProvingContext,
    xfvk: *const c_uchar,
    xfvk_len: size_t,
    to: *const c_uchar,
    to_len: size_t,
    rcm: *const c_uchar,
    rcm_len: size_t,
    value: u64,
    memo: *const c_uchar,
    memo_len: size_t,
    description_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let address: PaymentAddress = unsafe { c_deserialize(to, to_len) }?;
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len) }?;
        let memo: &[u8] = unsafe { c_deserialize_slice(memo, memo_len) };

        let ctx: &mut SaplingProvingContext = unsafe { c_dereference(ctx) };

        let params: &ZcashParameters = State::proof_params()?;
        let proving_key: &Parameters<Bls12> = &params.output_params;

        let output_description = prepare_output_description(
            ctx,
            xfvk.fvk.ovk,
            OutputDetails { to_address: address, value },
            rcm,
            Some(memo),
            proving_key,
        );

        unsafe { c_serialize_res(output_description, description_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_output_description_from_ovk(
    ctx: *mut SaplingProvingContext,
    ovk: *const c_uchar,
    ovk_len: size_t,
    to: *const c_uchar,
    to_len: size_t,
    rcm: *const c_uchar,
    rcm_len: size_t,
    value: u64,
    description_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let ovk: OutgoingViewingKey = unsafe { c_deserialize(ovk, ovk_len) }?;
        let address: PaymentAddress = unsafe { c_deserialize(to, to_len) }?;
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len) }?;

        let ctx: &mut SaplingProvingContext = unsafe { c_dereference(ctx) };

        let params: &ZcashParameters = State::proof_params()?;
        let proving_key: &Parameters<Bls12> = &params.output_params;

        let output_description = prepare_output_description(
            ctx,
            ovk,
            OutputDetails { to_address: address, value },
            rcm,
            None,
            proving_key,
        );

        unsafe { c_serialize_res(output_description, description_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_partial_output_description(
    ctx: *mut SaplingProvingContext,
    to: *const c_uchar,
    to_len: size_t,
    rcm: *const c_uchar,
    rcm_len: size_t,
    esk: *const c_uchar,
    esk_len: size_t,
    value: u64,
    description_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let address: PaymentAddress = unsafe { c_deserialize(to, to_len)? };
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len)? };
        let esk: jubjub::Scalar = unsafe { c_deserialize(esk, esk_len)? };

        let ctx: &mut SaplingProvingContext = unsafe { c_dereference(ctx) };

        let params: &ZcashParameters = State::proof_params()?;
        let proving_key: &Parameters<Bls12> = &params.output_params;

        let output_description = prepare_partial_output_description(
            ctx,
            OutputDetails { to_address: address, value },
            rcm,
            esk,
            proving_key,
        );

        unsafe { c_serialize_res(output_description, description_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_derive_epk_from_esk(
    diversifier: *const c_uchar,
    diversifier_len: size_t,
    esk: *const c_uchar,
    esk_len: size_t,
    epk_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let diversifier: [u8; 11] = unsafe { c_deserialize_slice(diversifier, diversifier_len) }.try_into()
            .map_err(|_| SaplingError::caused_by("deriveEpkFromEsk: index must be an array of 11 bytes"))?;
        let diversifier = Diversifier(diversifier);
        let esk: jubjub::Scalar = unsafe { c_deserialize(esk, esk_len) }?;

        let epk = derive_epk(diversifier, esk);

        unsafe { c_serialize_res(epk, epk_len) }
    })
}