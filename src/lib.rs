#![allow(dead_code)]
use wasm_bindgen::prelude::*;

pub use wasm_bindings::{
    commitment::*,
    key_agreement::*,
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
use crate::transaction::ProofParameters;

mod address;
mod common;
mod key;
mod transaction;

mod wasm_bindings;

mod state;

#[wasm_bindgen(js_name = "initParams")]
pub fn wasm_init_params(spend_params: &[u8], output_params: &[u8]) {
    init_lib();
    if State::proof_params().is_err() {
        State::set_proof_params(ProofParameters::from(spend_params, output_params));
    }
}

pub fn init_lib() {
    if !State::is_initialized() {
        console_error_panic_hook::set_once();
        State::set_initialized();
    }
}
