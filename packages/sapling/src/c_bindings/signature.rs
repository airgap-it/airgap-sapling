use std::convert::TryInto;

use libc::{c_uchar, size_t};
use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::utils::c_utils::{c_dereference, c_deserialize_slice, c_serialize_res, c_ptr_catch_result};
use crate::transaction::create_binding_sig;

#[no_mangle]
pub extern "C" fn c_binding_signature(
    ctx: *mut SaplingProvingContext,
    value_balance: i64,
    sighash: *const c_uchar,
    sighash_len: size_t,
    signature_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let ctx: &mut SaplingProvingContext = unsafe { c_dereference(ctx) };
        let sighash: [u8; 32] = unsafe { c_deserialize_slice(sighash, sighash_len) }.try_into()
            .map_err(|_| SaplingError::caused_by("bindingSignature: sighash must be an array of 32 bytes"))?;

        let binding_sig = create_binding_sig(ctx, value_balance, sighash);

        unsafe { c_serialize_res(binding_sig, signature_len) }
    })
}