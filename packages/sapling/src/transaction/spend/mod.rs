pub use description::{compute_nullifier, prepare_spend_description, sign_spend_description, UnsignedSpendDescription};
pub use proof::{SpendDetails, SpendParameters};

mod description;
mod proof;
mod errors;