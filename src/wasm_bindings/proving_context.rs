use wasm_bindgen::prelude::*;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::transaction::{drop_context, init_context};

#[wasm_bindgen(catch)]
pub fn wasm_init_proving_context() -> *mut SaplingProvingContext {
    init_context()
}

#[wasm_bindgen(catch)]
pub fn wasm_drop_proving_context(ctx: *mut SaplingProvingContext) {
    drop_context(ctx);
}