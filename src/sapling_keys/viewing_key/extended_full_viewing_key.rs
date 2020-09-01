use zcash_primitives::zip32::ExtendedFullViewingKey;

use crate::sapling_keys::get_extended_spending_key;
use super::errors::ViewingKeyError;

pub fn get_extended_full_viewing_key(seed: &[u8], derivation_path: &str) -> Result<ExtendedFullViewingKey, ViewingKeyError> {
    let spending_key = get_extended_spending_key(seed, derivation_path)
        .or_else(|err| Err(ViewingKeyError::caused_by(err)))?;

    let viewing_key = ExtendedFullViewingKey::from(&spending_key);

    Ok(viewing_key)
}

pub fn get_extended_full_viewing_key_bytes(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, ViewingKeyError> {
    let viewing_key = get_extended_full_viewing_key(seed, derivation_path)?;

    let mut bytes: Vec<u8> = vec![];
    viewing_key.write(&mut bytes).or_else(|err| Err(ViewingKeyError::caused_by(err)))?;

    Ok(bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sapling_keys::{
        spending_key::SpendingKeyError,
        derivation::DerivationPathError,
    };

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
                let actual = get_extended_full_viewing_key_bytes(&SEED, path).unwrap();
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
            ("", ViewingKeyError::caused_by(
                SpendingKeyError::caused_by(
                    DerivationPathError::Empty.to_string()
                ).to_string()
            )),
            ("/44'/123'/0'/0/0", ViewingKeyError::caused_by(
                SpendingKeyError::caused_by(
                    DerivationPathError::MissingPrefix.to_string()
                ).to_string()
            )),
            ("m/44'/123q/0'/0/0", ViewingKeyError::caused_by(
                SpendingKeyError::caused_by(
                    DerivationPathError::invalid_character(vec!["q"]).to_string()
                ).to_string()
            )),
            ("m/44'//0'/0/0", ViewingKeyError::caused_by(
                SpendingKeyError::caused_by(
                    DerivationPathError::EmptyIndex.to_string()
                ).to_string()
            )),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, err)| {
                let actual = get_extended_full_viewing_key(&SEED, path).unwrap_err();
                let actual_bytes = get_extended_full_viewing_key_bytes(&SEED, path).unwrap_err();

                (actual, actual_bytes, err)
            });

        for (actual, actual_bytes, expected) in actual_expected {
            assert_eq!(actual, *expected);
            assert_eq!(actual_bytes, *expected);
        }
    }
}