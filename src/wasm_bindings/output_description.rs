use bellman::groth16::Parameters;
use bls12_381::Bls12;
use wasm_bindgen::prelude::*;
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::ExtendedFullViewingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::wasm_utils::{dereference, js_deserialize, js_error_from, js_serialize_res};
use crate::State;
use crate::transaction::{prepare_output_description, ProofParams};

#[wasm_bindgen(catch, js_name = "outputDescriptionFromXfvk")]
pub fn wasm_output_description_from_xfvk(ctx: u32, xfvk: &[u8], to: &[u8], rcm: &[u8], value: u64) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx: &mut SaplingProvingContext = unsafe { dereference(ctx) };

    let params: &ProofParams = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_output_description(
        ctx,
        xfvk.fvk.ovk,
        address,
        rcm,
        value,
        None,
        proving_key
    );

    js_serialize_res(output_description)
}

#[wasm_bindgen(catch, js_name = "outputDescriptionFromXfvkWithMemo")]
pub fn wasm_output_description_from_xfvk_with_memo(ctx: u32, xfvk: &[u8], to: &[u8], rcm: &[u8], value: u64, memo: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx: &mut SaplingProvingContext = unsafe { dereference(ctx) };

    let params: &ProofParams = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_output_description(
        ctx,
        xfvk.fvk.ovk,
        address,
        rcm,
        value,
        Some(memo),
        proving_key
    );

    js_serialize_res(output_description)
}

#[wasm_bindgen(catch, js_name = "outputDescriptionFromOvk")]
pub fn wasm_output_description_from_ovk(ctx: u32, ovk: &[u8], to: &[u8], rcm: &[u8], value: u64) -> Result<Vec<u8>, JsValue> {
    let ovk: OutgoingViewingKey = js_deserialize(ovk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx: &mut SaplingProvingContext = unsafe { dereference(ctx) };

    let params: &ProofParams = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.output_params;

    let output_description = prepare_output_description(
        ctx,
        ovk,
        address,
        rcm,
        value,
        None,
        proving_key
    );

    js_serialize_res(output_description)
}