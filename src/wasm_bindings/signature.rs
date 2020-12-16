use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::wasm_utils::{dereference, js_result_from, js_serialize_res};
use crate::transaction::create_binding_sig;

#[wasm_bindgen(catch, js_name = "bindingSignature")]
pub fn wasm_binding_signature(ctx: u32, value_balance: i64, sighash: &[u8]) -> Result<Vec<u8>, JsValue> {
    let ctx: &mut SaplingProvingContext = unsafe { dereference(ctx) };
    let sighash: [u8; 32] = sighash.try_into()
        .or_else(|_| js_result_from("bindingSignature: sighash must be an array of 32 bytes"))?;

    let binding_sig = create_binding_sig(ctx, value_balance, sighash);

    js_serialize_res(binding_sig)
}