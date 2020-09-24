use std::convert::TryInto;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use zcash_primitives::keys::{FullViewingKey, OutgoingViewingKey};
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::sapling::Node;
use zcash_primitives::transaction::components::SpendDescription;
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};
use zcash_proofs::sapling::SaplingProvingContext;

use crate::address::{get_next_xfvk_address, get_xfvk_address};
use crate::common::utils::wasm_utils::{js_deserialize, js_error_from, js_serialize};
use crate::key::SaplingKey;
use crate::transaction::{dereference_context, drop_context, init_context, prepare_output_description, prepare_spend_description, rand_scalar, sign_spend_description};

mod address;
mod key;
mod transaction;

mod common;

// Extended Spending Key

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path);

    js_serialize(xsk)
}

// Extended Full Viewing Key

#[wasm_bindgen(catch)]
pub fn get_extended_full_viewing_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    let xfvk = ExtendedFullViewingKey::from_seed(seed, derivation_path);

    js_serialize(xfvk)
}

// Outgoing Viewing Key

#[wasm_bindgen(catch)]
pub fn get_outgoing_viewing_key_from_xfvk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;

    Ok(xfvk.fvk.ovk.0.to_vec())
}

// Payment Address

#[wasm_bindgen(catch)]
pub fn get_default_payment_address_from_xfvk(xfvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let xfvk_address = get_xfvk_address(&xfvk, None);

    js_serialize(xfvk_address)
}

#[wasm_bindgen(catch)]
pub fn get_next_payment_address_from_xfvk(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_error_from("get_next_payment_address_from_xfvk: index must be an array of 11 bytes"))?;

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let xfvk_address = get_next_xfvk_address(&xfvk, index);

    js_serialize(xfvk_address)
}

#[wasm_bindgen(catch)]
pub fn get_payment_address_from_xfvk(xfvk: &[u8], index: &[u8]) -> Result<Vec<u8>, JsValue> {
    let index: [u8; 11] = index.try_into()
        .or_else(|_| js_error_from("get_payment_address_from_xfvk: index must be an array of 11 bytes"))?;

    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let xfvk_address = get_xfvk_address(&xfvk, Some(index));

    js_serialize(xfvk_address)
}

// Transaction

#[wasm_bindgen(catch)]
pub fn init_proving_context() -> Result<*mut SaplingProvingContext, JsValue> {
    Ok(init_context())
}

#[wasm_bindgen(catch)]
pub fn drop_proving_context(ctx: *mut SaplingProvingContext) -> Result<(), JsValue> {
    drop_context(ctx);

    Ok(())
}

#[wasm_bindgen(catch)]
pub fn prepare_spend_description_from_xsk(
    ctx: *mut SaplingProvingContext,
    xsk: &[u8],
    address: &[u8],
    value: u64,
    anchor: &[u8],
    merkle_path: &[u8],
    position: u64,
    proving_key: &[u8],
    verifying_key: &[u8]
) -> Result<Vec<u8>, JsValue> {
    let xsk: ExtendedSpendingKey = js_deserialize(xsk)?;
    let payment_address: PaymentAddress = js_deserialize(address)?;
    let anchor: bls12_381::Scalar = js_deserialize(anchor)?;
    let merkle_path: MerklePath<Node> = js_deserialize(merkle_path)?;

    let ctx = dereference_context(ctx);

    let rcm = rand_scalar();
    let ar = rand_scalar();

    let input_description = prepare_spend_description(
        ctx,
        xsk,
        payment_address,
        rcm,
        ar,
        value,
        anchor,
        merkle_path,
        position,
        proving_key,
        verifying_key
    );

    js_serialize(input_description)
}

#[wasm_bindgen(catch)]
pub fn sign_spend_description_with_xsk(spend_description: &[u8], xsk: &[u8], ar: &[u8], sighash: &[u8]) -> Result<Vec<u8>, JsValue> {
    let spend_description: SpendDescription = js_deserialize(spend_description)?;
    let xks: ExtendedSpendingKey = js_deserialize(xsk)?;
    let ar: jubjub::Scalar = js_deserialize(ar)?;

    let sighash: [u8; 32] = sighash.try_into()
        .or_else(|_| js_error_from("sign_spend_description: sighash must be an array of 32 bytes"))?;

    let spend_description = sign_spend_description(spend_description, xks, ar, sighash);

    js_serialize(spend_description)
}

#[wasm_bindgen(catch)]
pub fn prepare_output_description_from_xfvk(ctx: *mut SaplingProvingContext, xfvk: &[u8], to: &[u8], value: u64, proving_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;

    let ctx = dereference_context(ctx);
    let rcm = rand_scalar();

    let output_description = prepare_output_description(ctx, xfvk.fvk.ovk, address, rcm, value, None, proving_key);

    js_serialize(output_description)
}

#[wasm_bindgen(catch)]
pub fn prepare_output_description_from_xfvk_with_memo(ctx: *mut SaplingProvingContext, xfvk: &[u8], to: &[u8], value: u64, proving_key: &[u8], memo: &[u8]) -> Result<Vec<u8>, JsValue> {
    let xfvk: ExtendedFullViewingKey = js_deserialize(xfvk)?;
    let address: PaymentAddress = js_deserialize(to)?;

    let ctx = dereference_context(ctx);
    let rcm = rand_scalar();

    let output_description = prepare_output_description(ctx, xfvk.fvk.ovk, address, rcm, value, Some(memo), proving_key);

    js_serialize(output_description)
}

#[wasm_bindgen(catch)]
pub fn prepare_output_description_from_ovk(ctx: *mut SaplingProvingContext, ovk: &[u8], to: &[u8], value: u64, proving_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let ovk: OutgoingViewingKey = js_deserialize(ovk)?;
    let address: PaymentAddress = js_deserialize(to)?;

    let ctx = dereference_context(ctx);
    let rcm = rand_scalar();

    let output_description = prepare_output_description(ctx, ovk, address, rcm, value, None, proving_key);

    js_serialize(output_description)
}