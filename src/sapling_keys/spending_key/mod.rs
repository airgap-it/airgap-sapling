mod extended_spending_key;
mod errors;

pub use extended_spending_key::{
    get_extended_spending_key,
    get_extended_spending_key_bytes,
};

pub use errors::SpendingKeyError;
