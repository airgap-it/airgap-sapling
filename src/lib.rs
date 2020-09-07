mod address;
mod key;
mod utils;
mod errors;

use std::convert::TryInto;
use wasm_bindgen::{
    JsValue,
    prelude::*,
};
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::{
    address::{
        SaplingAddress,
        SaplingAddressError,

        get_xfvk_address,
        get_next_xfvk_address,
    },
    key::{
        get_xsk,
        xsk_to_bytes,

        get_xfvk,
        xfvk_to_bytes,
        xfvk_from_bytes,
    },
    utils::wasm_utils::js_error_from
};

// Extended Spending Key

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    get_xsk(seed, derivation_path)
        .and_then(|xsk| xsk_to_bytes(&xsk))
        .or_else(js_error_from)
}

// Extended Full Viewing Key

#[wasm_bindgen(catch)]
pub fn get_extended_full_viewing_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    get_xfvk(seed, derivation_path)
        .and_then(|xfvk| xfvk_to_bytes(&xfvk))
        .or_else(js_error_from)
}

// Payment Address

#[wasm_bindgen(catch)]
pub fn get_default_payment_address_from_viewing_key(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    get_payment_address(xfvk, |xfvk| get_xfvk_address(xfvk, None))
}

#[wasm_bindgen(catch)]
pub fn get_next_payment_address_from_viewing_key(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_error_from("get_next_payment_address_from_viewing_key: index must be an array of 11 bytes"))?;

    get_payment_address(xfvk, |xfvk| get_next_xfvk_address(xfvk, index))
}

#[wasm_bindgen(catch)]
pub fn get_payment_address_from_viewing_key(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_error_from("get_payment_address_from_viewing_key: index must be an array of 11 bytes"))?;

    get_payment_address(xfvk, |xfvk| get_xfvk_address(xfvk, Some(index)))
}

fn get_payment_address<F>(xfvk: &[u8], xfvk_map: F) -> Result<Vec<u8>, JsValue>
    where F: Fn(&ExtendedFullViewingKey) -> Result<SaplingAddress, SaplingAddressError> {

    xfvk_from_bytes(xfvk)
        .map_err(SaplingAddressError::caused_by)
        .and_then(|xfvk| xfvk_map(&xfvk))
        .map(|addr| [&addr.index[..], &addr.diversifier[..], &addr.pkd[..]].concat())
        .or_else(js_error_from)
}