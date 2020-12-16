pub use merkle_tree::hash as merkle_hash;
pub use output::prepare_output_description;
pub use proof::{parse_params, ProofParams};
pub use rand::rand_scalar;
pub use signature::create_binding_sig;
pub use spend::{prepare_spend_description, sign_spend_description};

mod output;
mod spend;
mod signature;

mod merkle_tree;
mod note;
mod proof;
mod rand;
mod scalar;

mod errors;
