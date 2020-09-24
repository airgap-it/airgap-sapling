use wasm_bindgen::prelude::*;
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::ExtendedFullViewingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::wasm_utils::{js_deserialize, js_serialize};
use crate::transaction::{dereference_context, prepare_output_description, rand_scalar};

#[wasm_bindgen(catch)]
pub fn prepare_output_description_from_xfvk(ctx: *mut SaplingProvingContext, xfvk: &[u8], to: &[u8], rcm: &[u8], value: u64, proving_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx = dereference_context(ctx);

    let output_description = prepare_output_description(ctx, xfvk.fvk.ovk, address, rcm, value, None, proving_key);

    js_serialize(output_description)
}

#[wasm_bindgen(catch)]
pub fn prepare_output_description_from_xfvk_with_memo(ctx: *mut SaplingProvingContext, xfvk: &[u8], to: &[u8], , rcm: &[u8], value: u64, proving_key: &[u8], memo: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx = dereference_context(ctx);

    let output_description = prepare_output_description(ctx, xfvk.fvk.ovk, address, rcm, value, Some(memo), proving_key);

    js_serialize(output_description)
}

#[wasm_bindgen(catch)]
pub fn prepare_output_description_from_ovk(ctx: *mut SaplingProvingContext, ovk: &[u8], to: &[u8], rcm: &[u8], value: u64, proving_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let ovk: OutgoingViewingKey = js_deserialize(ovk)?;
    let address: PaymentAddress = js_deserialize(to)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let ctx = dereference_context(ctx);

    let output_description = prepare_output_description(ctx, ovk, address, rcm, value, None, proving_key);

    js_serialize(output_description)
}