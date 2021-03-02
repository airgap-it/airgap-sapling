use crate::common::errors::SaplingError;

pub trait SaplingKey {
    fn from_seed(seed: &[u8], derivation_path: &str) -> Result<Self, SaplingError> where Self: Sized;
}