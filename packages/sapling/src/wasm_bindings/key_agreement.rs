use wasm_bindgen::prelude::*;
use zcash_primitives::note_encryption::sapling_ka_agree;

use crate::common::utils::wasm_utils::{js_deserialize, js_serialize};
use crate::wasm_bindings::init::init_lib;

#[wasm_bindgen(catch, js_name = "keyAgreement")]
pub fn wasm_key_agreement(p: &[u8], sk: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let p: jubjub::ExtendedPoint = js_deserialize(p)?;
    let sk: jubjub::Scalar = js_deserialize(sk)?;

    let ka = sapling_ka_agree(&sk, &p);
    
    js_serialize(ka)
}
