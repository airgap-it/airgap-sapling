use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::ExtendedSpendingKey;

use crate::common::utils::wasm_utils::js_serialize;
use crate::key::SaplingKey;

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path);

    js_serialize(xsk)
}