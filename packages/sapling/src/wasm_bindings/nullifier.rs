use wasm_bindgen::prelude::*;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::common::utils::wasm_utils::{js_deserialize, js_result_from};
use crate::transaction::compute_nullifier;
use crate::wasm_bindings::init::init_lib;

#[wasm_bindgen(catch, js_name = "computeNullifier")]
pub fn wasm_compute_nullifier_with_xfvk(xfvk: &[u8], address: &[u8], value: &str, rcm: &[u8], position: &str) -> Result<Vec<u8>, JsValue> {
    init_lib();
    
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let payment_address: PaymentAddress = js_deserialize(address)?;
    let value: u64 = value.parse().or_else(|_| js_result_from("computeNullifier: invalid value"))?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;
    let position: u64 = position.parse().or_else(|_| js_result_from("computeNullifier: invalid position"))?;

    let nullifier = compute_nullifier(&xfvk.fvk.vk, &payment_address, value, rcm, position)
        .map_err(|err| JsValue::from(err.to_string()))?;

    Ok(nullifier.to_vec())
}