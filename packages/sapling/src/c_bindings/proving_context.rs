use zcash_proofs::sapling::SaplingProvingContext;

use crate::common::utils::c_utils::{c_drop_reference, c_reference};

#[no_mangle]
pub extern "C" fn c_init_proving_context() -> *mut SaplingProvingContext {
    c_reference(SaplingProvingContext::new())
}

#[no_mangle]
pub extern "C" fn c_drop_proving_context(ctx: *mut SaplingProvingContext) {
    unsafe { c_drop_reference::<SaplingProvingContext>(ctx) }
}