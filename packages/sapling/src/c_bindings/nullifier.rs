use libc::{c_uchar, size_t};
use zcash_primitives::primitives::PaymentAddress;
use zcash_primitives::zip32::ExtendedFullViewingKey;
use crate::c_init_lib;

use crate::common::errors::SaplingError;
use crate::common::utils::c_utils::{c_get_result_res, c_deserialize, c_ptr_catch_result};
use crate::transaction::compute_nullifier;

#[no_mangle]
pub extern "C" fn c_compute_nullifier_with_xfvk(
    xfvk: *const c_uchar,
    xfvk_len: size_t,
    address: *const c_uchar,
    address_len: size_t,
    value: u64,
    rcm: *const c_uchar,
    rcm_len: size_t,
    position: u64,
    nullifier_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let payment_address: PaymentAddress = unsafe { c_deserialize(address, address_len) }?;
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len) }?;

        let nullifier = compute_nullifier(&xfvk.fvk.vk, &payment_address, value, rcm, position)?;

        unsafe { c_get_result_res::<SaplingError>(nullifier.to_vec(), nullifier_len) }
    })
}