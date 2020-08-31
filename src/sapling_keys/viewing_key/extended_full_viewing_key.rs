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
    use bip39::{Language, Mnemonic, Seed};

    const MNEMONIC: &str = "sweet banner danger symptom ring skill benefit kitten appear much comic grab oak number coach";
    const PASSWORD: &str = "";
    const DERIVATION_PATH: &str = "m/";

    #[test]
    fn generates_extended_full_viewing_key_from_mnemonic() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let _bytes = get_extended_full_viewing_key_bytes(seed.as_bytes(), DERIVATION_PATH).unwrap();

        // TODO
    }

    #[test]
    fn fails_to_generate_extended_full_viewing_key_when_derivation_path_empty() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let error = get_extended_full_viewing_key_bytes(seed.as_bytes(), "");

        assert_eq!(
            error,
            Err(
                ViewingKeyError::caused_by(
                    SpendingKeyError::caused_by(
                        DerivationPathError::Empty.to_string()
                    ).to_string()
                )
            )
        );
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_missing_prefix() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let error = get_extended_full_viewing_key_bytes(seed.as_bytes(), "/44'/123'/0'/0/0");

        assert_eq!(
            error,
            Err(
                ViewingKeyError::caused_by(
                    SpendingKeyError::caused_by(
                        DerivationPathError::MissingPrefix.to_string()
                    ).to_string()
                )
            )
        );
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_invalid() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let invalid_character = get_extended_full_viewing_key_bytes(seed.as_bytes(), "m/44'/123q/0'/0/0");
        let empty_index = get_extended_full_viewing_key_bytes(seed.as_bytes(), "m/44'//0'/0/0");

        assert_eq!(
            invalid_character,
            Err(
                ViewingKeyError::caused_by(
                    SpendingKeyError::caused_by(
                        DerivationPathError::invalid_character(vec!["q"]).to_string()
                    ).to_string()
                )
            )
        );

        assert_eq!(
            empty_index,
            Err(
                ViewingKeyError::caused_by(
                    SpendingKeyError::caused_by(
                        DerivationPathError::EmptyIndex.to_string()
                    ).to_string()
                )
            )
        );
    }
}