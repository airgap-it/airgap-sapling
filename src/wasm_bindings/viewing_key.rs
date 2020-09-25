use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::common::utils::wasm_utils::{js_deserialize, js_serialize};
use crate::key::SaplingKey;

#[wasm_bindgen(catch)]
pub fn wasm_extended_full_viewing_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xfvk = ExtendedFullViewingKey::from_seed(seed, derivation_path);

    js_serialize(xfvk)
}

#[wasm_bindgen(catch)]
pub fn wasm_outgoing_viewing_key_from_xfvk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;

    Ok(xfvk.fvk.ovk.0.to_vec())
}