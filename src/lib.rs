mod sapling_keys;
mod utils;

use std::convert::TryInto;
use wasm_bindgen::{
    JsValue,
    prelude::*,
};

use sapling_keys::{
    get_extended_spending_key_bytes,
    get_extended_full_viewing_key_bytes,
    get_address_from_viewing_key_bytes,
};

// Extended Spending Key

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    get_extended_spending_key_bytes(seed, derivation_path).or_else(|err| Err(JsValue::from(err.to_string())))
}

// Extended Full Viewing Key

#[wasm_bindgen(catch)]
pub fn get_extended_full_viewing_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    get_extended_full_viewing_key_bytes(seed, derivation_path).or_else(|err| Err(JsValue::from(err.to_string())))
}

// Payment Address

#[wasm_bindgen(catch)]
pub fn get_default_payment_address_from_viewing_key(viewing_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    get_address_from_viewing_key(viewing_key, None)
}

#[wasm_bindgen(catch)]
pub fn get_payment_address_from_viewing_key(viewing_key: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into().expect("get_address_from_viewing_key: index must be an array of 11 bytes");
    get_address_from_viewing_key(viewing_key, Some(index))
}

fn get_address_from_viewing_key(viewing_key: &[u8], index: Option<[u8; 11]>) -> Result<Vec<u8>, JsValue> {
    get_address_from_viewing_key_bytes(viewing_key, index)
        .map(|addr| [&addr.index[..], &addr.raw[..]].concat())
        .or_else(|err| Err(JsValue::from(err.to_string())))
}