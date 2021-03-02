use std::convert::TryInto;

use bellman::groth16::Parameters;
use bls12_381::Bls12;
use wasm_bindgen::prelude::*;
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::primitives::{Diversifier, PaymentAddress};
use zcash_primitives::zip32::ExtendedFullViewingKey;
use zcash_proofs::sapling::SaplingProvingContext;
use zcash_proofs::ZcashParameters;

use crate::common::utils::wasm_utils::{js_dereference, js_deserialize, js_error_from, js_result_from, js_serialize_res};
use crate::State;
use crate::transaction::{derive_epk, OutputDetails, prepare_output_description, prepare_partial_output_description};
use crate::wasm_bindings::init::init_lib;

#[wasm_bindgen(catch, js_name = "outputDescriptionFromXfvk")]
pub fn wasm_output_description_from_xfvk(ctx: u32, xfvk: &[u8], to: &[u8], rcm: &[u8], value: u64) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx: &mut SaplingProvingContext = unsafe { js_dereference(ctx) };

    let params: &ZcashParameters = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_output_description(
        ctx,
        xfvk.fvk.ovk,
        OutputDetails { to_address: address, value },
        rcm,
        None,
        proving_key
    );

    js_serialize_res(output_description)
}

#[wasm_bindgen(catch, js_name = "outputDescriptionFromXfvkWithMemo")]
pub fn wasm_output_description_from_xfvk_with_memo(ctx: u32, xfvk: &[u8], to: &[u8], rcm: &[u8], value: u64, memo: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx: &mut SaplingProvingContext = unsafe { js_dereference(ctx) };

    let params: &ZcashParameters = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_output_description(
        ctx,
        xfvk.fvk.ovk,
        OutputDetails { to_address: address, value },
        rcm,
        Some(memo),
        proving_key
    );

    js_serialize_res(output_description)
}

#[wasm_bindgen(catch, js_name = "outputDescriptionFromOvk")]
pub fn wasm_output_description_from_ovk(ctx: u32, ovk: &[u8], to: &[u8], rcm: &[u8], value: u64) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let ovk: OutgoingViewingKey = js_deserialize(ovk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx: &mut SaplingProvingContext = unsafe { js_dereference(ctx) };

    let params: &ZcashParameters = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_output_description(
        ctx,
        ovk,
        OutputDetails { to_address: address, value },
        rcm,
        None,
        proving_key
    );

    js_serialize_res(output_description)
}

#[wasm_bindgen(catch, js_name = "partialOutputDescription")]
pub fn wasm_partial_output_description(ctx: u32, to: &[u8], rcm: &[u8], esk: &[u8], value: u64) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;
    let esk: jubjub::Scalar = js_deserialize(esk)?;

    let ctx: &mut SaplingProvingContext = unsafe { js_dereference(ctx) };

    let params: &ZcashParameters = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_partial_output_description(
        ctx,
        OutputDetails { to_address: address, value },
        rcm,
        esk,
        proving_key
    );

    js_serialize_res(output_description)
}

#[wasm_bindgen(catch, js_name = "deriveEpkFromEsk")]
pub fn wasm_derive_epk_from_esk(diversifier: &[u8], esk: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let diversifier: [u8; 11] = diversifier.try_into()
        .or_else(|_| js_result_from("deriveEpkFromEsk: index must be an array of 11 bytes"))?;
    let diversifier = Diversifier(diversifier);
    let esk: jubjub::Scalar = js_deserialize(esk)?;

    let epk = derive_epk(diversifier, esk);

    js_serialize_res(epk)
}