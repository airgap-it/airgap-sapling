use wasm_bindgen::prelude::*;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::common::utils::wasm_utils::js_deserialize;
use crate::init_lib;
use crate::transaction::compute_nullifier;

#[wasm_bindgen(catch, js_name = "computeNullifier")]
pub fn wasm_compute_nullifier_with_xfvk(xfvk: &[u8], address: &[u8], value: u64, rcm: &[u8], position: u64) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let payment_address: PaymentAddress = js_deserialize(address)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let nullifier = compute_nullifier(&xfvk.fvk.vk, &payment_address, value, rcm, position)
        .map_err(|err| JsValue::from(err.to_string()))?;

    Ok(nullifier.to_vec())
}