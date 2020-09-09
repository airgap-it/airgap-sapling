pub use errors::SaplingAddressError;
pub use sapling_address::SaplingAddress;
pub use xfvk_address::{
    get_next_xfvk_address,
    get_xfvk_address,
};

mod sapling_address;
mod indexed_address;
mod xfvk_address;
mod errors;

