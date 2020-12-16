#![allow(dead_code)]
use wasm_bindgen::prelude::*;

pub use wasm_bindings::{
    merkle_tree::*,
    output_description::*,
    payment_address::*,
    proving_context::*,
    rand::*,
    signature::*,
    spend_description::*,
    spending_key::*,
    viewing_key::*,
};

use crate::state::State;
use crate::transaction::parse_params;

mod address;
mod common;
mod key;
mod transaction;

mod wasm_bindings;

mod state;

#[wasm_bindgen(js_name = "initParams")]
pub fn wasm_init_params(spend_params: &[u8], output_params: &[u8]) {
    console_error_panic_hook::set_once();

    let proof_params = parse_params(spend_params, output_params);

    State::set_proof_params(proof_params);
}
