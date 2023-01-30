extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};

use crate::common::utils::wasm_utils::{js_deserialize, js_serialize, js_serialize_res};
use crate::key::{crh_ivk, SaplingKey};
use crate::wasm_bindings::init::wasm_init_lib;

#[wasm_bindgen(catch, js_name = "xfvk")]
pub fn wasm_xfvk(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    wasm_init_lib();

    let xfvk = ExtendedFullViewingKey::from_seed(seed, derivation_path);

    js_serialize_res(xfvk)
}

#[wasm_bindgen(catch, js_name = "xfvkFromXsk")]
pub fn wasm_xfvk_from_xsk(xsk: &[u8]) -> Result<Vec<u8>, JsValue> {
    wasm_init_lib();

    let xsk: ExtendedSpendingKey = js_deserialize(xsk)?;
    let xfvk = ExtendedFullViewingKey::from(&xsk);

    js_serialize(xfvk)
}

#[wasm_bindgen(catch, js_name = "ovkFromXfvk")]
pub fn wasm_ovk_from_xfvk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    wasm_init_lib();

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;

    Ok(xfvk.fvk.ovk.0.to_vec())
}

#[wasm_bindgen(catch, js_name = "xfvkToIvk")]
pub fn wasm_xfvk_to_ivk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    wasm_init_lib();

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let ivk = crh_ivk(&xfvk);

    Ok(ivk)
}