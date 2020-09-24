use crate::common::traits::Serializable;
use crate::common::errors::SaplingError;
use zcash_primitives::merkle_tree::{MerklePath, Hashable};

impl <Node: Hashable> Serializable<Vec<u8>, SaplingError> for MerklePath<Node> {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        MerklePath::from_slice(&serialized[..]).map_err(|_| SaplingError::new())
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Err(SaplingError::new())
    }
}