use wasm_bindgen::prelude::*;

use crate::transaction::rand_scalar;

#[wasm_bindgen(catch)]
pub fn wasm_rand_r() -> Vec<u8> {
    let scalar = rand_scalar();

    scalar.to_bytes().to_vec()
}