use wasm_bindgen::prelude::*;
use zcash_primitives::primitives::PaymentAddress;

use crate::common::utils::wasm_utils::{js_deserialize, js_result_from, js_serialize_res};
use crate::transaction::create_note;
use crate::wasm_bindings::init::init_lib;

#[wasm_bindgen(catch, js_name = "computeCommitment")]
pub fn wasm_compute_cmu(address: &[u8], value: &str, rcm: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let address: PaymentAddress = js_deserialize(address)?;
    let value: u64 = value.parse().or_else(|_| js_result_from("computeCommitment: invalid value"))?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;

    let cmu = create_note(&address, value, rcm).map(|note| note.cmu());

    js_serialize_res(cmu)
}