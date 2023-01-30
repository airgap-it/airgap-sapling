use std::convert::TryInto;

use wasm_bindgen::prelude::*;

use crate::common::utils::assert_utils::assert_value_or_error;
use crate::common::utils::wasm_utils::{js_error_from, js_result_from};
use crate::transaction::merkle_hash;
use crate::wasm_bindings::init::wasm_init_lib;

#[wasm_bindgen(catch, js_name = "merkleHash")]
pub fn wasm_merkle_hash(depth: usize, lhs: &[u8], rhs: &[u8]) -> Result<Vec<u8>, JsValue> {
    wasm_init_lib();
    
    assert_value_or_error(depth <= 62, js_error_from("merkleHash: depth should be not larger than 62"))?;

    let lhs: [u8; 32] = lhs.try_into()
        .or_else(|_| js_result_from("merkleHash: lhs must be of length 32"))?;
    let rhs: [u8; 32] = rhs.try_into()
        .or_else(|_| js_result_from("merkleHash: rhs must be of length 32"))?;

    Ok(merkle_hash(depth, lhs, rhs).to_vec())
}