use zcash_proofs::ZcashParameters;

use crate::common::errors::{CausedBy, SaplingError};

pub struct State {
    is_initialized: bool,
    proof_params: Option<ZcashParameters>,
}

impl State {
    pub fn is_initialized() -> bool {
        unsafe { STATE.is_initialized }
    }

    pub fn set_initialized() {
        unsafe { STATE.is_initialized = true; }
    }

    pub fn proof_params() -> Result<&'static ZcashParameters, SaplingError> {
        unsafe { STATE.proof_params.as_ref().ok_or_else(|| SaplingError::caused_by("sapling parameters have not been initialized")) }
    }

    pub fn set_proof_params(params: ZcashParameters) {
        unsafe { STATE.proof_params = Some(params); }
    }
}

static mut STATE: State = State {
    is_initialized: false,
    proof_params: None,
};