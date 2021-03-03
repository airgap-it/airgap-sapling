use std::convert::TryInto;

use libc::{c_uchar, size_t};

use crate::common::utils::assert_utils::assert_value_or_error;
use crate::common::utils::c_utils::{c_get_result_res, c_deserialize_slice, c_ptr_catch_result};
use crate::transaction::merkle_hash;

#[no_mangle]
pub extern "C" fn c_merkle_hash(
    depth: u64,
    lhs: *const c_uchar,
    lhs_len: size_t,
    rhs: *const c_uchar,
    rhs_len: size_t,
    merkle_hash_len: *mut size_t,
) -> *mut c_uchar {
    c_ptr_catch_result(|| {
        assert_value_or_error(depth <= 62, "merkleHash: depth should be not larger than 62")?;

        let depth: usize = depth.try_into().map_err(|_| "merkleHash: invalid depth")?;

        let lhs: [u8; 32] = unsafe { c_deserialize_slice(lhs, lhs_len) }.try_into().map_err(|_| "merkleHash: lhs must be of length 32")?;
        let rhs: [u8; 32] = unsafe { c_deserialize_slice(rhs, rhs_len) }.try_into().map_err(|_| "merkleHash: rhs must be of length 32")?;

        let merkle_hash = merkle_hash(depth, lhs, rhs).to_vec();

        unsafe { c_get_result_res::<&str>(merkle_hash, merkle_hash_len) }
    })
}