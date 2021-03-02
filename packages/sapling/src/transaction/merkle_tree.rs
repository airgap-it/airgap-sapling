use zcash_primitives::merkle_tree::{Hashable, MerklePath};
use zcash_primitives::sapling::merkle_hash;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
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

pub fn hash(depth: usize, lhs: [u8; 32], rhs: [u8; 32]) -> [u8; 32] {
    merkle_hash(depth, &lhs, &rhs)
}