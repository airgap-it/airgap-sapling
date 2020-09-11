use zcash_proofs::sapling::SaplingProvingContext;

pub fn init_context() -> *mut SaplingProvingContext {
    let ctx = Box::new(SaplingProvingContext::new());

    Box::into_raw(ctx)
}

pub fn drop_context(ctx: *mut SaplingProvingContext) {
    let ctx = unsafe { Box::from_raw(ctx) };
    drop(ctx);
}