use libc::{c_uchar, size_t};
use zcash_primitives::primitives::PaymentAddress;
use crate::c_init_lib;

use crate::common::utils::c_utils::{c_deserialize, c_serialize_res, c_ptr_catch_result};
use crate::transaction::create_note;

#[no_mangle]
pub extern "C" fn c_compute_cmu(
    address: *const c_uchar,
    address_len: size_t,
    value: u64,
    rcm: *const c_uchar,
    rcm_len: size_t,
    cmu_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let address: PaymentAddress = unsafe { c_deserialize(address, address_len) }?;
        let rcm: jubjub::Scalar = unsafe { c_deserialize(rcm, rcm_len) }?;

        let cmu = create_note(&address, value, rcm).map(|note| note.cmu());

        unsafe { c_serialize_res(cmu, cmu_len) }
    })
}