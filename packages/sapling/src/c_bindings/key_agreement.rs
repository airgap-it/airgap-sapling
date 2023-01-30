use libc::{c_uchar, size_t};
use zcash_primitives::note_encryption::sapling_ka_agree;
use crate::c_init_lib;

use crate::common::utils::c_utils::{c_deserialize, c_serialize, c_ptr_catch_result};

#[no_mangle]
pub extern "C" fn c_key_agreement(
    p: *const c_uchar,
    p_len: size_t,
    sk: *const c_uchar,
    sk_len: size_t,
    ka_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let p: jubjub::ExtendedPoint = unsafe { c_deserialize(p, p_len) }?;
        let sk: jubjub::Scalar = unsafe { c_deserialize(sk, sk_len) }?;

        let ka = sapling_ka_agree(&sk, &p);

        unsafe { c_serialize(ka, ka_len) }
    })
}
