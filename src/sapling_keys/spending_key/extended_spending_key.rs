use zcash_primitives::zip32::ExtendedSpendingKey;

use crate::sapling_keys::derivation::split_derivation_path;
use super::errors::SpendingKeyError;

pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<ExtendedSpendingKey, SpendingKeyError> {
    let master_key = ExtendedSpendingKey::master(seed);
    let derivation_indices = split_derivation_path(derivation_path).or_else(|err| Err(SpendingKeyError::caused_by(err)))?;

    let spending_key = ExtendedSpendingKey::from_path(&master_key, &derivation_indices);

    Ok(spending_key)
}

pub fn get_extended_spending_key_bytes(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, SpendingKeyError> {
    let spending_key = get_extended_spending_key(seed, derivation_path)?;

    let mut bytes: Vec<u8> = vec![];
    spending_key.write(&mut bytes).or_else(|err| Err(SpendingKeyError::caused_by(err)))?;

    Ok(bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sapling_keys::derivation::DerivationPathError;
    use hex;

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    // from https://github.com/zcash/zcash/blob/master/src/gtest/test_zip32.cpp
    #[test]
    fn generates_extended_spending_key_from_seed() {
        let bytes = get_extended_spending_key_bytes(&SEED, "m/").unwrap();
        let expected: Vec<u8> = [
            "00", // depth (1 byte)
            "00000000", // parent_fvk_tag (4 bytes)
            "00000000", // child_index (4 bytes)
            "d0947c4b03bf72a37ab44f72276d1cf3fdcd7ebf3e73348b7e550d752018668e", // chain_code (32 bytes)
            "b6c00c93d36032b9a268e99e86a860776560bf0e83c1a10b51f607c954742506", // expsk.ask (32 bytes)
            "8204ede83b2f1fbd84f9b45d7f996e2ebd0a030ad243b48ed39f748a8821ea06", // expsk.nsk (32 bytes)
            "395884890323b9d4933c021db89bcf767df21977b2ff0683848321a4df4afb21", // expsk.ovk (32 bytes)
            "77c17cb75b7796afb39f0f3e91c924607da56fa9a20e283509bc8a3ef996a172", // dk (32 bytes)
        ].iter().flat_map(|&s| hex::decode(s).unwrap()).collect();

        assert_eq!(bytes, expected);
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_empty() {
        let error = get_extended_spending_key(&SEED, "").unwrap_err();
        let error_bytes = get_extended_spending_key_bytes(&SEED, "").unwrap_err();

        let expected = SpendingKeyError::caused_by(
            DerivationPathError::Empty.to_string()
        );
        assert_eq!(error, expected);
        assert_eq!(error_bytes, expected);
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_missing_prefix() {
        let path = "/44'/123'/0'/0/0";
        let error = get_extended_spending_key(&SEED, path).unwrap_err();
        let error_bytes = get_extended_spending_key_bytes(&SEED, path).unwrap_err();

        let expected = SpendingKeyError::caused_by(
            DerivationPathError::MissingPrefix.to_string()
        );
        assert_eq!(error, expected);
        assert_eq!(error_bytes, expected);
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_invalid() {
        let invalid_character_path = "m/44'/123q/0'/0/0";
        let invalid_character = get_extended_spending_key(&SEED, invalid_character_path).unwrap_err();
        let invalid_character_bytes = get_extended_spending_key_bytes(&SEED, invalid_character_path).unwrap_err();

        let empty_index_path = "m/44'//0'/0/0";
        let empty_index = get_extended_spending_key(&SEED, empty_index_path).unwrap_err();
        let empty_index_bytes = get_extended_spending_key_bytes(&SEED, empty_index_path).unwrap_err();

        let invalid_character_expected = SpendingKeyError::caused_by(
            DerivationPathError::invalid_character(vec!["q"]).to_string()
        );
        assert_eq!(invalid_character, invalid_character_expected);
        assert_eq!(invalid_character_bytes, invalid_character_expected);

        let empty_index_expected = SpendingKeyError::caused_by(
            DerivationPathError::EmptyIndex.to_string()
        );
        assert_eq!(empty_index, empty_index_expected);
        assert_eq!(empty_index_bytes, empty_index_expected);
    }
}