use wasm_bindgen::prelude::*;

use crate::transaction::rand_scalar;
use crate::common::utils::wasm_utils::js_serialize;

#[wasm_bindgen(catch)]
pub fn wasm_rand_r() -> Result<Vec<u8>, JsValue> {
    let scalar = rand_scalar();

    js_serialize(scalar)
}