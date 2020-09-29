use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::common::utils::wasm_utils::{js_deserialize, js_serialize_res};
use crate::key::SaplingKey;

#[wasm_bindgen(catch)]
pub fn wasm_xfvk(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xfvk = ExtendedFullViewingKey::from_seed(seed, derivation_path);

    js_serialize_res(xfvk)
}

#[wasm_bindgen(catch)]
pub fn wasm_ovk_from_xfvk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;

    Ok(xfvk.fvk.ovk.0.to_vec())
}