pub use output::create_output_description;
pub use context::{drop_context, init_context};

mod output;
mod spend;
mod context;

mod rand;