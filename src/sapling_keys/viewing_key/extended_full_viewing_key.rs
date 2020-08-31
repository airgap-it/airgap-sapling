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

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];


    #[test]
    fn generates_extended_full_viewing_key_from_seed() {
        let _bytes = get_extended_full_viewing_key_bytes(&SEED, "m/").unwrap();

        // TODO
    }

    #[test]
    fn fails_to_generate_extended_full_viewing_key_when_derivation_path_empty() {
        let error = get_extended_full_viewing_key(&SEED, "").unwrap_err();
        let error_bytes = get_extended_full_viewing_key_bytes(&SEED, "").unwrap_err();

        let expected = ViewingKeyError::caused_by(
            SpendingKeyError::caused_by(
                DerivationPathError::Empty.to_string()
            ).to_string()
        );
        assert_eq!(error, expected);
        assert_eq!(error_bytes, expected);
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_missing_prefix() {
        let path = "/44'/123'/0'/0/0";
        let error = get_extended_full_viewing_key(&SEED, path).unwrap_err();
        let error_bytes = get_extended_full_viewing_key_bytes(&SEED, path).unwrap_err();

        let expected = ViewingKeyError::caused_by(
            SpendingKeyError::caused_by(
                DerivationPathError::MissingPrefix.to_string()
            ).to_string()
        );

        assert_eq!(error, expected);
        assert_eq!(error_bytes, expected);
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_invalid() {
        let invalid_character_path = "m/44'/123q/0'/0/0";
        let invalid_character = get_extended_full_viewing_key(&SEED, invalid_character_path).unwrap_err();
        let invalid_character_bytes = get_extended_full_viewing_key_bytes(&SEED, invalid_character_path).unwrap_err();

        let empty_index_path = "m/44'//0'/0/0";
        let empty_index = get_extended_full_viewing_key(&SEED, empty_index_path).unwrap_err();
        let empty_index_bytes = get_extended_full_viewing_key_bytes(&SEED, empty_index_path).unwrap_err();

        let invalid_character_expected = ViewingKeyError::caused_by(
            SpendingKeyError::caused_by(
                DerivationPathError::invalid_character(vec!["q"]).to_string()
            ).to_string()
        );
        assert_eq!(invalid_character, invalid_character_expected);
        assert_eq!(invalid_character_bytes, invalid_character_expected);

        let empty_index_expected = ViewingKeyError::caused_by(
            SpendingKeyError::caused_by(
                DerivationPathError::EmptyIndex.to_string()
            ).to_string()
        );
        assert_eq!(empty_index, empty_index_expected);
        assert_eq!(empty_index_bytes, empty_index_expected);
    }
}