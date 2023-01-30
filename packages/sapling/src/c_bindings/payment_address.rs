use std::convert::TryInto;

use libc::{c_uchar, size_t};
use zcash_primitives::primitives::{Diversifier, PaymentAddress};
use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::address::{get_ivk_address, get_next_xfvk_address, get_xfvk_address};
use crate::c_init_lib;
use crate::common::errors::{CausedBy, SaplingError};
use crate::common::utils::c_utils::{c_get_result_res, c_deserialize, c_deserialize_slice, c_serialize, c_serialize_res, c_ptr_catch_result};

#[no_mangle]
pub extern "C" fn c_default_payment_address_from_xfvk(
    xfvk: *const c_uchar,
    xfvk_len: size_t,
    address_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let xfvk_address = get_xfvk_address(&xfvk, None);

        unsafe { c_serialize_res(xfvk_address, address_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_next_payment_address_from_xfvk(
    xfvk: *const c_uchar,
    xfvk_len: size_t,
    index: *const c_uchar,
    index_len: size_t,
    address_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let index: [u8; 11] = unsafe { c_deserialize_slice(index, index_len) }.try_into()
            .map_err(|_| SaplingError::caused_by("nextPaymentAddressFromXfvk: index must be an array of 11 bytes"))?;

        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let xfvk_address = get_next_xfvk_address(&xfvk, index);

        unsafe { c_serialize_res(xfvk_address, address_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_payment_address_from_xfvk(
    xfvk: *const c_uchar,
    xfvk_len: size_t,
    index: *const c_uchar,
    index_len: size_t,
    address_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let index: [u8; 11] = unsafe { c_deserialize_slice(index, index_len) }.try_into()
            .map_err(|_| SaplingError::caused_by("paymentAddressFromXfvk: index must be an array of 11 bytes"))?;

        let xfvk: ExtendedFullViewingKey = unsafe { c_deserialize(xfvk, xfvk_len) }?;
        let xfvk_address = get_xfvk_address(&xfvk, Some(index));

        unsafe { c_serialize_res(xfvk_address, address_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_payment_address_from_ivk(
    ivk: *const c_uchar,
    ivk_len: size_t,
    diversifier: *const c_uchar,
    diversifier_len: size_t,
    address_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let ivk: jubjub::Scalar = unsafe { c_deserialize(ivk, ivk_len) }?;

        let diversifier: [u8; 11] = unsafe { c_deserialize_slice(diversifier, diversifier_len) }.try_into()
            .map_err(|_| SaplingError::caused_by("paymentAddressfromIvk: index must be an array of 11 bytes"))?;
        let diversifier = Diversifier(diversifier);

        let address = get_ivk_address(ivk, diversifier);

        unsafe { c_serialize_res(address, address_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_diversifier_from_payment_address(
    address: *const c_uchar,
    address_len: size_t,
    diversifier_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let address: PaymentAddress = unsafe { c_deserialize(address, address_len) }?;
        let diversifier = address.diversifier().0.to_vec();

        unsafe { c_get_result_res::<SaplingError>(diversifier, diversifier_len) }
    })
}

#[no_mangle]
pub extern "C" fn c_pkd_from_payment_address(
    address: *const c_uchar,
    address_len: size_t,
    pkd_len: *mut size_t,
) -> *mut c_uchar {
    c_init_lib();

    c_ptr_catch_result(|| {
        let address: PaymentAddress = unsafe { c_deserialize(address, address_len) }?;

        unsafe { c_serialize(*address.pk_d(), pkd_len) }
    })
}