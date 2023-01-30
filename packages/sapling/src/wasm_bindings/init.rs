use wasm_bindgen::prelude::*;

use crate::State;
use crate::transaction::prepare_proof_parameters;

#[wasm_bindgen(js_name = "initParams")]
pub fn wasm_init_params(spend_params: &[u8], output_params: &[u8]) {
    wasm_init_lib();
    if State::proof_params().is_err() {
        State::set_proof_params(prepare_proof_parameters(spend_params, output_params));
    }
}

pub fn wasm_init_lib() {
    if !State::is_initialized() {
        console_error_panic_hook::set_once();
        State::set_initialized();
    }
}