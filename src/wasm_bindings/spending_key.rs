use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::{ExtendedSpendingKey, ExtendedFullViewingKey};

use crate::common::utils::wasm_utils::{js_serialize_res, js_deserialize, js_serialize};
use crate::key::SaplingKey;

#[wasm_bindgen(catch, js_name = "xsk")]
pub fn wasm_xsk(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path);

    js_serialize_res(xsk)
}

#[wasm_bindgen(catch, js_name = "xfvkFromXsk")]
pub fn wasm_xfvk_from_xsk(xsk: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xsk: ExtendedSpendingKey = js_deserialize(xsk)?;
    let xfvk = ExtendedFullViewingKey::from(&xsk);

    js_serialize(xfvk)
}