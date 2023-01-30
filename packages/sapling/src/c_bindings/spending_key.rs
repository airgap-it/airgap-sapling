use libc::{c_char, c_uchar, size_t};
use zcash_primitives::zip32::ExtendedSpendingKey;
use crate::c_init_lib;

use crate::common::utils::c_utils::{c_deserialize_slice, c_deserialize_str, c_serialize_res, c_ptr_catch_result};
use crate::key::SaplingKey;

#[no_mangle]
pub extern "C" fn c_xsk(
    seed: *const c_uchar,
    seed_len: size_t,
    derivation_path: *const c_char,
    xsk_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let seed: &[u8] = unsafe { c_deserialize_slice(seed, seed_len) };
        let derivation_path: &str = unsafe { c_deserialize_str(derivation_path) };

        let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path);

        unsafe { c_serialize_res(xsk, xsk_len) }
    })
}