use libc::{c_uchar, size_t};
use zcash_primitives::zip32::ExtendedSpendingKey;
use crate::common::utils::c_utils::{c_deserialize, c_ptr_catch_result, c_serialize};
use crate::c_init_lib;

#[no_mangle]
pub extern "C" fn c_pak_from_xsk(xsk: *const c_uchar, xsk_len: size_t, pak_len: *mut size_t) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xsk: ExtendedSpendingKey = unsafe { c_deserialize(xsk, xsk_len) }?;
        let pak = xsk.expsk.proof_generation_key();

        unsafe { c_serialize(pak, pak_len) }
    })
}