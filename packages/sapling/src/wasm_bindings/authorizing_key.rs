use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::ExtendedSpendingKey;

use crate::common::utils::wasm_utils::{js_deserialize, js_serialize};
use crate::init_lib;

#[wasm_bindgen(catch, js_name = "pakFromXsk")]
pub fn wasm_pak_from_xsk(xsk: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xsk: ExtendedSpendingKey = js_deserialize(xsk)?;
    let pak = xsk.expsk.proof_generation_key();

    js_serialize(pak)
}