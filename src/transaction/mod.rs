pub use context::{dereference_context, drop_context, init_context};
pub use output::create_output_description;
pub use spend::create_spend_description;
pub use rand::generate_rand_scalar;

mod output;
mod spend;
mod context;

mod merkle_path;
mod rand;
mod scalar;