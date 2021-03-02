use std::any::Any;
use std::ffi::CStr;
use std::fmt::Display;
use std::panic::{catch_unwind, UnwindSafe};
use std::slice;

use libc::{c_char, c_uchar, size_t};
use wyz::TapResult;

use crate::common::traits::Serializable;

pub fn c_size_catch_result<F, E>(f: F) -> size_t
    where F: FnOnce() -> Result<size_t, E> + UnwindSafe,
          E: Display {

    catch_result(f).unwrap_or(0)
}

pub fn c_bool_catch_result<F, R, E>(f: F) -> bool
    where F: FnOnce() -> Result<R, E> + UnwindSafe,
          E: Display {

    catch_result(f).is_ok()
}

pub fn c_bool_catch<F, R>(f: F) -> bool
    where F: FnOnce() -> R + UnwindSafe {

    catch(f).is_ok()
}

pub unsafe fn c_copy_result(bytes: Vec<u8>, result: *mut *const c_uchar) -> size_t {
    let result = &mut *result;
    let len = bytes.len();

    *result = Box::new(bytes).as_ptr();
    len
}

pub unsafe fn c_copy_result_res<E>(bytes: Vec<u8>, result: *mut *const c_uchar) -> Result<size_t, E> {
    Ok(c_copy_result(bytes, result))
}

pub unsafe fn c_serialize<S, E>(value: S, result: *mut *const c_uchar) -> Result<size_t, E>
    where S: Serializable<Vec<u8>, E>,
          E: ToString {

    let bytes = value.serialize()?;
    c_copy_result_res(bytes, result)
}

pub unsafe fn c_serialize_res<S, E>(value: Result<S, E>, result: *mut *const c_uchar) -> Result<size_t, E>
    where S: Serializable<Vec<u8>, E>,
          E: ToString {

    let bytes = value.and_then(|s| s.serialize())?;
    c_copy_result_res(bytes, result)
}

pub unsafe fn c_deserialize<S, E>(bytes: *const c_uchar, len: size_t) -> Result<S, E>
    where S: Serializable<Vec<u8>, E>,
          E: ToString {

    let bytes: &[u8] = c_deserialize_slice(bytes, len);
    S::deserialize(bytes.to_vec())
}

pub unsafe fn c_deserialize_slice<'a>(bytes: *const c_uchar, len: size_t) -> &'a[u8] {
    slice::from_raw_parts(bytes, len)
}

pub unsafe fn c_deserialize_str<'a>(chars: *const c_char) -> &'a str {
    CStr::from_ptr(chars).to_str().unwrap()
}

pub fn c_reference<T>(object: T) -> *mut T {
    let boxed = Box::new(object);

    Box::into_raw(boxed)
}

pub unsafe fn c_dereference<'a, T>(pointer: *mut T) -> &'a mut T {
    &mut *pointer
}

pub unsafe fn c_drop_reference<T>(pointer: *mut T) {
    drop(Box::from_raw(pointer));
}

fn catch_result<F, R, E>(f: F) -> Result<R, String>
    where F: FnOnce() -> Result<R, E> + UnwindSafe,
          E: Display {

    catch_unwind(f)
        .map_err(panic_to_string)
        .and_then(|res| res.map_err(|err| err.to_string()))
        .tap_err(|err| error!("{}", err))
}

fn catch<F, R>(f: F) -> Result<R, String>
    where F: FnOnce() -> R + UnwindSafe {

    catch_unwind(f)
        .map_err(panic_to_string)
        .tap_err(|err| error!("{}", err))
}

fn panic_to_string(panic: Box<dyn Any + Send>) -> String {
    match panic.downcast::<String>() {
        Ok(panic_msg) => {
            panic_msg.to_string()
        }
        Err(_) => {
            String::from("panicked: unknown error")
        }
    }
}