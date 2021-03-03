use libc::{c_uchar, size_t};

use crate::common::utils::c_utils::{c_serialize, c_ptr_catch_result};
use crate::transaction::rand_scalar;

#[no_mangle]
pub extern "C" fn c_rand_r(r_len: *mut size_t) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        let scalar = rand_scalar();
        unsafe { c_serialize(scalar, r_len) }
    })
}