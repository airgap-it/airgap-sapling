use std::convert::TryInto;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};

use crate::address::{get_next_xfvk_address, get_xfvk_address};
use crate::key::SaplingKey;
use crate::common::utils::wasm_utils::{js_deserialize, js_error_from, js_serialize};

mod address;
mod key;

mod common;

// Extended Spending Key

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path);

    js_serialize(xsk)
}

// Extended Full Viewing Key

#[wasm_bindgen(catch)]
pub fn get_extended_full_viewing_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xfvk = ExtendedFullViewingKey::from_seed(seed, derivation_path);

    js_serialize(xfvk)
}

// Payment Address

#[wasm_bindgen(catch)]
pub fn get_default_payment_address_from_viewing_key(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk = js_deserialize(xfvk)?;
    let xfvk_address = get_xfvk_address(&xfvk, None);

    js_serialize(xfvk_address)
}

#[wasm_bindgen(catch)]
pub fn get_next_payment_address_from_viewing_key(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_error_from("get_next_payment_address_from_viewing_key: index must be an array of 11 bytes"))?;

    let xfvk = js_deserialize(xfvk)?;
    let xfvk_address = get_next_xfvk_address(&xfvk, index);

    js_serialize(xfvk_address)
}

#[wasm_bindgen(catch)]
pub fn get_payment_address_from_viewing_key(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_error_from("get_payment_address_from_viewing_key: index must be an array of 11 bytes"))?;

    let xfvk = js_deserialize(xfvk)?;
    let xfvk_address = get_xfvk_address(&xfvk, Some(index));

    js_serialize(xfvk_address)
}