use libc::{c_uchar, size_t};

#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(target_os = "android")]
pub use android_logger::{Config, FilterBuilder};
#[cfg(target_os = "android")]
use log::LevelFilter;

#[cfg(target_os = "ios")]
use oslog::OsLogger;
#[cfg(target_os = "ios")]
use log::LevelFilter;

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
    c_init_lib();

    c_bool_catch(|| {
        if State::proof_params().is_err() {
            let spend_params: &[u8] = unsafe { c_deserialize_slice(spend_params, spend_params_len) };
            let output_params: &[u8] = unsafe { c_deserialize_slice(output_params, output_params_len) };

            State::set_proof_params(prepare_proof_parameters(spend_params, output_params));
        }
    })
}

pub fn c_init_lib() {
    if !State::is_initialized() {
        init_logger();
        State::set_initialized();
    }
}

#[cfg(target_os = "android")]
fn init_logger() {
    android_logger::init_once(
        Config::default()
            .with_tag("Sapling")
            .with_max_level(LevelFilter::max())
    );
}

#[cfg(target_os = "ios")]
fn init_logger() {
    OsLogger::new("sapling")
        .level_filter(LevelFilter::Trace)
        .init()
        .unwrap();
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn init_logger() { /* no action */ }