use crate::transaction::ProofParams;
use crate::common::errors::{SaplingError, CausedBy};

pub struct State {
    proof_params: Option<ProofParams>,
}

impl State {
    pub fn proof_params() -> Result<&'static ProofParams, SaplingError> {
        unsafe { STATE.proof_params.as_ref().ok_or_else(|| SaplingError::caused_by("sapling parameters have not been initialized")) }
    }

    pub fn set_proof_params(params: ProofParams) {
        unsafe { STATE.proof_params = Some(params) }
    }
}

static mut STATE: State = State {
    proof_params: None
};