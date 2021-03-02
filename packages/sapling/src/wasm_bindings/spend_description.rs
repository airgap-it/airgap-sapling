use std::convert::TryInto;

use bellman::groth16::{Parameters, PreparedVerifyingKey};
use bls12_381::Bls12;
use wasm_bindgen::prelude::*;
use zcash_primitives::merkle_tree::MerklePath;
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::sapling::Node;
use zcash_primitives::zip32::ExtendedSpendingKey;
use zcash_proofs::sapling::SaplingProvingContext;
use zcash_proofs::ZcashParameters;

use crate::common::utils::wasm_utils::{js_dereference, js_deserialize, js_error_from, js_result_from, js_serialize_res};
use crate::State;
use crate::transaction::{prepare_spend_description, sign_spend_description, SpendDetails, SpendParameters, UnsignedSpendDescription};
use crate::wasm_bindings::init::init_lib;

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(catch, js_name = "spendDescriptionFromXsk")]
pub fn wasm_spend_description_from_xsk(
    ctx: u32,
    xsk: &[u8],
    address: &[u8],
    rcm: &[u8],
    ar: &[u8],
    value: u64,
    anchor: &[u8],
    merkle_path: &[u8],
) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let xsk: ExtendedSpendingKey = js_deserialize(xsk)?;
    let payment_address: PaymentAddress = js_deserialize(address)?;
    let rcm: jubjub::Scalar = js_deserialize(rcm)?;
    let ar: jubjub::Scalar = js_deserialize(ar)?;
    let anchor: bls12_381::Scalar = js_deserialize(anchor)?;
    let merkle_path: MerklePath<Node> = js_deserialize(merkle_path)?;

    let ctx: &mut SaplingProvingContext = unsafe { js_dereference(ctx) };

    let params: &ZcashParameters = State::proof_params().map_err(js_error_from)?;
    let proving_key: &Parameters<Bls12> = &params.spend_params;
    let verifying_key: &PreparedVerifyingKey<Bls12> = &params.spend_vk;

    let spend_description = prepare_spend_description(
        ctx,
        SpendDetails { from_xsk: xsk, to_address: payment_address, value },
        rcm,
        ar,
        anchor,
        merkle_path,
        SpendParameters { proving_key, verifying_key }
    );

    js_serialize_res(spend_description)
}

#[wasm_bindgen(catch, js_name = "signSpendDescriptionWithXsk")]
pub fn wasm_sign_spend_description_with_xsk(spend_description: &[u8], xsk: &[u8], ar: &[u8], sighash: &[u8]) -> Result<Vec<u8>, JsValue> {
    init_lib();

    let spend_description: UnsignedSpendDescription = js_deserialize(spend_description)?;
    let xks: ExtendedSpendingKey = js_deserialize(xsk)?;
    let ar: jubjub::Scalar = js_deserialize(ar)?;

    let sighash: [u8; 32] = sighash.try_into()
        .or_else(|_| js_result_from("signSpendDescriptionWithXsk: sighash must be an array of 32 bytes"))?;

    let spend_description = sign_spend_description(spend_description, xks, ar, sighash);

    js_serialize_res(spend_description)
}