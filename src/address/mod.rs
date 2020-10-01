pub use errors::SaplingAddressError;
pub use xfvk_address::{
    get_next_xfvk_address,
    get_xfvk_address,
};

mod payment_address;
mod indexed_address;
mod xfvk_address;
mod errors;

