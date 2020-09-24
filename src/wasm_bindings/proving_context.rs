use wasm_bindgen::prelude::*;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::transaction::{drop_context, init_context};

#[wasm_bindgen(catch)]
pub fn init_proving_context() -> Result<*mut SaplingProvingContext, JsValue> {
    Ok(init_context())
}

#[wasm_bindgen(catch)]
pub fn drop_proving_context(ctx: *mut SaplingProvingContext) -> Result<(), JsValue> {
    drop_context(ctx);

    Ok(())
}