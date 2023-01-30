use libc::{c_char, c_uchar, size_t};
use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};
use crate::c_init_lib;

use crate::common::errors::SaplingError;
use crate::common::utils::c_utils::{c_get_result_res, c_deserialize, c_deserialize_slice, c_deserialize_str, c_serialize, c_serialize_res, c_ptr_catch_result};
use crate::key::{crh_ivk, SaplingKey};

#[no_mangle]
pub extern "C" fn c_xfvk(
    seed: *const c_uchar,
    seed_len: size_t,
    derivation_path: *const c_char,
    xfvk_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let seed: &[u8] = unsafe { c_deserialize_slice(seed, seed_len) };
        let derivation_path: &str = unsafe { c_deserialize_str(derivation_path) };

        let xfvk = ExtendedFullViewingKey::from_seed(seed, derivation_path);

        unsafe { c_serialize_res(xfvk, xfvk_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_xfvk_from_xsk(xsk: *const c_uchar, xsk_len: size_t, xfvk_len: *mut size_t) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xsk: ExtendedSpendingKey = unsafe { c_deserialize(xsk, xsk_len) }?;
        let xfvk = ExtendedFullViewingKey::from(&xsk);

        unsafe { c_serialize(xfvk, xfvk_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_ovk_from_xfvk(xfvk: *const c_uchar, xfvk_len: size_t, ovk_len: *mut size_t) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let ovk = xfvk.fvk.ovk.0.to_vec();

        unsafe { c_get_result_res::<SaplingError>(ovk, ovk_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_xfvk_to_ivk(xfvk: *const c_uchar, xfvk_len: size_t, ivk_len: *mut size_t) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let ivk = crh_ivk(&xfvk);

        unsafe { c_get_result_res::<SaplingError>(ivk, ivk_len) }
    })
}