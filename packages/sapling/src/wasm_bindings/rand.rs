extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use crate::common::utils::wasm_utils::js_serialize;
use crate::transaction::rand_scalar;
use crate::wasm_bindings::init::init_lib;

#[wasm_bindgen(catch, js_name = "randR")]
pub fn wasm_rand_r() -> Result<Vec<u8>, JsValue> {
    init_lib();

    let scalar = rand_scalar();

    js_serialize(scalar)
}