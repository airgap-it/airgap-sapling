extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use crate::common::utils::wasm_utils::js_serialize;
use crate::init_lib;
use crate::transaction::rand_scalar;

#[wasm_bindgen(catch, js_name = "randR")]
pub fn wasm_rand_r() -> Result<Vec<u8>, JsValue> {
    init_lib();

    let scalar = rand_scalar();

    js_serialize(scalar)
}