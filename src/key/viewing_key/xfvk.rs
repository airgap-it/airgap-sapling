use zcash_primitives::zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::key::sapling_key::SaplingKey;

use super::errors::ViewingKeyError;

impl SaplingKey for ExtendedFullViewingKey {
    fn from_seed(seed: &[u8], derivation_path: &str) -> Result<Self, SaplingError> where Self: Sized {
        let xsk = ExtendedSpendingKey::from_seed(seed, derivation_path)?;
        let xfvk = ExtendedFullViewingKey::from(&xsk);

        Ok(xfvk)
    }
}

impl Serializable<SaplingError> for ExtendedFullViewingKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self, SaplingError> where Self: Sized {
        ExtendedFullViewingKey::read(bytes).map_err(|err| SaplingError::caused_by(ViewingKeyError::ReadFailed(err)))
    }

    fn to_bytes(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(|err| SaplingError::caused_by(ViewingKeyError::WriteFailed(err)))?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::key::bip32::{Bip32IndexError, Bip32PathError};

    use super::*;

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];


    // from https://github.com/zcash/zcash/blob/master/src/gtest/test_zip32.cpp
    #[test]
    fn generates_extended_full_viewing_key_from_seed() {
        let test_data = vec![
            ("m/1/2h", [
                "02", // depth (1 byte)
                "db999e07", // parent_fvk_tag (4 bytes, LE)
                "02000080", // child_index (4 bytes, LE)
                "97ce15f4ed1b9739b0262a463bcb3dc9b3bd2323a9baa441ca42777383a8d435", // chain_code (32 bytes, LE)
                "a6c5925a0f85fa4f1e405e3a4970d0c4a4b4814438f4e9d4520e20f7fdcf3841", // fvk.ak (32 bytes, LE)
                "304e305916216beb7b654d8aae50ecd188fcb384bc36c00c664f307725e2ee11", // fvk.nk (32 bytes, LE)
                "cf81182e96223c028ce3d6eb4794d3113b95069d14c57588e193b65efc2813bc", // fvk.ovk (32 bytes, LE)
                "a3eda19f9eff46ca12dfa1bf10371b48d1b4a40c4d05a0d8dce0e7dc62b07b37", // dk (32 bytes, LE)
            ]),
            ("m/1/2h/3", [
                "03", // depth (1 byte)
                "48c18375", // parent_fvk_tag (4 bytes, LE)
                "03000000", // child_index (4 bytes, LE)
                "8d937bcf81ba430d5b49afc0a403367b1fd99879ecba41be051c5a4aa7d6e7e8", // chain_code (32 bytes, LE)
                "b185c57b509c2536c4f2d326d766c8fab25447de5375a9328d649ddabd97a6a3", // fvk.ak (32 bytes, LE)
                "db88049e02d207568afc42e07db2abed500b2701c01bbff36399764b81c0664f", // fvk.nk (32 bytes, LE)
                "69b9e0fa1c4b3deb91d53beee871156121474b8b62ef24134478dc3499691af6", // fvk.ovk (32 bytes, LE)
                "becb50c363bb2ed9da5c3043ceb0f1a0527bf836b29a35f7c0c9f261123be56e", // dk (32 bytes, LE)
            ]),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let xfvk = ExtendedFullViewingKey::from_seed(&SEED, path).unwrap();
                let actual = xfvk.to_bytes().unwrap();
                let expected: Vec<u8> = v.iter().flat_map(|&s| hex::decode(s).unwrap()).collect();

                (actual, expected)
            });


        for (actual, expected) in actual_expected {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_invalid() {
        let test_data = vec![
            ("", SaplingError::caused_by(Bip32PathError::Empty)),
            ("/44'/123'/0'/0/0", SaplingError::caused_by(Bip32PathError::MissingPrefix)),
            ("m/44'/123q/0'/0/0", SaplingError::caused_by(Bip32IndexError::InvalidCharacter(vec!["q".to_string()]))),
            ("m/44'//0'/0/0", SaplingError::caused_by(Bip32IndexError::Empty)),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, err)| {
                let actual = ExtendedFullViewingKey::from_seed(&SEED, path).unwrap_err();

                (actual, err)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn reads_extended_full_viewing_key_from_bytes() {
        let expected = ExtendedFullViewingKey::from_seed(&SEED, "m/").unwrap();
        let bytes = expected.to_bytes().unwrap();
        let actual = ExtendedFullViewingKey::from_bytes(&bytes).unwrap();

        assert_eq!(actual, expected);
    }
}