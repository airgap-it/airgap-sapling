use std::convert::TryInto;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use zcash_primitives::primitives::{Diversifier, PaymentAddress};
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::address::{get_ivk_address, get_next_xfvk_address, get_xfvk_address};
use crate::common::utils::wasm_utils::{js_deserialize, js_result_from, js_serialize_res, js_serialize};
use crate::init_lib;

#[wasm_bindgen(catch, js_name = "defaultPaymentAddressFromXfvk")]
pub fn wasm_default_payment_address_from_xfvk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let xfvk_address = get_xfvk_address(&xfvk, None);

    js_serialize_res(xfvk_address)
}

#[wasm_bindgen(catch, js_name = "nextPaymentAddressFromXfvk")]
pub fn wasm_next_payment_address_from_xfvk(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_result_from("nextPaymentAddressFromXfvk: index must be an array of 11 bytes"))?;

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let xfvk_address = get_next_xfvk_address(&xfvk, index);

    js_serialize_res(xfvk_address)
}

#[wasm_bindgen(catch, js_name = "paymentAddressFromXfvk")]
pub fn wasm_payment_address_from_xfvk(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_result_from("paymentAddressFromXfvk: index must be an array of 11 bytes"))?;

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let xfvk_address = get_xfvk_address(&xfvk, Some(index));

    js_serialize_res(xfvk_address)
}

#[wasm_bindgen(catch, js_name = "paymentAddressFromIvk")]
pub fn wasm_payment_address_from_ivk(ivk: &[u8], diversifier: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let ivk: jubjub::Scalar = js_deserialize(ivk)?;

    let diversifier: [u8; 11] = diversifier.try_into()
        .or_else(|_| js_result_from("paymentAddressfromIvk: index must be an array of 11 bytes"))?;
    let diversifier = Diversifier(diversifier);

    let address = get_ivk_address(ivk, diversifier);

    js_serialize_res(address)
}

#[wasm_bindgen(catch, js_name = "diversifierFromPaymentAddress")]
pub fn wasm_diversifier_from_payment_address(address: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let address: PaymentAddress = js_deserialize(address)?;

    Ok(address.diversifier().0.to_vec())
}

#[wasm_bindgen(catch, js_name = "pkdFromPaymentAddress")]
pub fn wasm_pkd_from_payment_address(address: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let address: PaymentAddress = js_deserialize(address)?;

    js_serialize(*address.pk_d())
}