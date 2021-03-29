use libc::{c_uchar, size_t};

use crate::common::utils::c_utils::{c_bool_catch, c_deserialize_slice};
use crate::state::State;
use crate::transaction::prepare_proof_parameters;

#[no_mangle]
pub extern "C" fn c_init_params(
    spend_params: *const c_uchar,
    spend_params_len: size_t,
    output_params: *const c_uchar,
    output_params_len: size_t,
) -> bool {
    c_bool_catch(|| {
        if State::proof_params().is_err() {
            let spend_params: &[u8] = unsafe { c_deserialize_slice(spend_params, spend_params_len) };
            let output_params: &[u8] = unsafe { c_deserialize_slice(output_params, output_params_len) };

            State::set_proof_params(prepare_proof_parameters(spend_params, output_params));
        }
    })
}