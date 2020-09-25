use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::wasm_utils::{js_error_from, js_serialize};
use crate::transaction::{create_binding_sig, deref_context};

#[wasm_bindgen(catch)]
pub fn wasm_binding_sig(ctx: *mut SaplingProvingContext, value_balance: i64, sighash: &[u8]) -> Result<Vec<u8>, JsValue> {
    let ctx = deref_context(ctx);
    let sighash: [u8; 32] = sighash.try_into()
        .or_else(|_| js_error_from("wasm_binding_sig: sighash must be an array of 32 bytes"))?;

    let binding_sig = create_binding_sig(ctx, value_balance, sighash);

    js_serialize(binding_sig)
}