mod sapling_keys;
mod utils;

use wasm_bindgen::{
    JsValue,
    prelude::*,
};

use sapling_keys::get_extended_spending_key_bytes;

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    get_extended_spending_key_bytes(seed, derivation_path).or_else(|err| Err(JsValue::from(err.to_string())))
}
