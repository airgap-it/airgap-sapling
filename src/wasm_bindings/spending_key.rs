use wasm_bindgen::prelude::*;
use zcash_primitives::zip32::ExtendedSpendingKey;

use crate::common::utils::wasm_utils::js_serialize_res;
use crate::init_lib;
use crate::key::SaplingKey;

#[wasm_bindgen(catch, js_name = "xsk")]
pub fn wasm_xsk(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path);

    js_serialize_res(xsk)
}