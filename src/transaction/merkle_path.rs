use crate::common::traits::Serializable;
use crate::common::errors::{SaplingError, CausedBy};
use zcash_primitives::merkle_tree::{MerklePath, Hashable};
use crate::transaction::errors::MerklePathError;

impl <Node: Hashable> Serializable<Vec<u8>, SaplingError> for MerklePath<Node> {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        MerklePath::from_slice(&serialized[..])
            .map_err(|_| MerklePathError::ReadFailed)
            .map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Err(SaplingError::caused_by(MerklePathError::CannotWrite))
    }
}