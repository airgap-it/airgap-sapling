use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::sapling::Node;
use zcash_primitives::transaction::components::SpendDescription;
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::wasm_utils::{js_deserialize, js_error_from, js_serialize_res};
use crate::transaction::{deref_context, prepare_spend_description, sign_spend_description};

#[wasm_bindgen(catch)]
pub fn wasm_spend_description_from_xsk(
    ctx: *mut SaplingProvingContext,
    xsk: &[u8],
    address: &[u8],
    rcm: &[u8],
    ar: &[u8],
    value: u64,
    anchor: &[u8],
    merkle_path: &[u8],
    proving_key: &[u8],
    verifying_key: &[u8]
) -> Result<Vec<u8>, JsValue> {
    let xsk: ExtendedSpendingKey = js_deserialize(xsk)?;
    let payment_address: PaymentAddress = js_deserialize(address)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;
    let ar: jubjub::Scalar = js_deserialize(ar)?;
    let anchor: bls12_381::Scalar = js_deserialize(anchor)?;
    let merkle_path: MerklePath<Node> = js_deserialize(merkle_path)?;

    let ctx = deref_context(ctx);

    let input_description = prepare_spend_description(
        ctx,
        xsk,
        payment_address,
        rcm,
        ar,
        value,
        anchor,
        merkle_path,
        proving_key,
        verifying_key
    );

    js_serialize_res(input_description)
}

#[wasm_bindgen(catch)]
pub fn wasm_sign_spend_description_with_xsk(spend_description: &[u8], xsk: &[u8], ar: &[u8], sighash: &[u8]) -> Result<Vec<u8>, JsValue> {
    let spend_description: SpendDescription = js_deserialize(spend_description)?;
    let xks: ExtendedSpendingKey = js_deserialize(xsk)?;
    let ar: jubjub::Scalar = js_deserialize(ar)?;

    let sighash: [u8; 32] = sighash.try_into()
        .or_else(|_| js_error_from("wasm_sign_spend_description_with_xsk: sighash must be an array of 32 bytes"))?;

    let spend_description = sign_spend_description(spend_description, xks, ar, sighash);

    js_serialize_res(spend_description)
}