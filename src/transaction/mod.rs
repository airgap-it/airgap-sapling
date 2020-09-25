pub use signature::create_binding_sig;
pub use context::{deref_context, drop_context, init_context};
pub use output::prepare_output_description;
pub use spend::{prepare_spend_description, sign_spend_description};
pub use rand::rand_scalar;

mod output;
mod spend;
mod signature;
mod context;

mod merkle_path;
mod note;
mod proof;
mod rand;
mod scalar;

mod errors;
