#![allow(dead_code)]

#[cfg(feature = "c_bindings")]
#[macro_use] extern crate log;

#[cfg(feature = "c_bindings")]
mod c_bindings;

#[cfg(feature = "c_bindings")]
pub use c_bindings::{
    commitment::*,
    init::*,
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

#[cfg(feature = "wasm_bindings")]
mod wasm_bindings;

#[cfg(feature = "wasm_bindings")]
pub use wasm_bindings::{
    commitment::*,
    init::*,
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

mod address;
mod common;
mod key;
mod transaction;

mod state;
