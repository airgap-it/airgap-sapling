use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::wasm_utils::{js_dereference, js_result_from, js_serialize_res};
use crate::transaction::create_binding_sig;
use crate::wasm_bindings::init::wasm_init_lib;

#[wasm_bindgen(catch, js_name = "bindingSignature")]
pub fn wasm_binding_signature(ctx: u32, value_balance: &str, sighash: &[u8]) -> Result<Vec<u8>, JsValue> {
    wasm_init_lib();

    let ctx: &mut SaplingProvingContext = unsafe { js_dereference(ctx) };
    let sighash: [u8; 32] = sighash.try_into()
        .or_else(|_| js_result_from("bindingSignature: sighash must be an array of 32 bytes"))?;
    let value_balance: i64 = value_balance.parse().or_else(|_| js_result_from("bindingSignature: invalid value_balance"))?;

    let binding_sig = create_binding_sig(ctx, value_balance, sighash);

    js_serialize_res(binding_sig)
}