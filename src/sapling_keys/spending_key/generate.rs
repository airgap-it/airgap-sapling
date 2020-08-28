use wasm_bindgen::{
    JsValue,
    prelude::*,
};

use zcash_primitives::{
    zip32::ExtendedSpendingKey
};

use crate::sapling_keys::derivation::{DerivationPathError, split_derivation_path};
use super::errors::SpendingKeyError;

#[wasm_bindgen(catch)]
pub fn get_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
    generate_extended_spending_key(seed, derivation_path).or_else(|err| Err(JsValue::from(err.to_string())))
}

fn generate_extended_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, SpendingKeyError> {
    let master = ExtendedSpendingKey::master(seed);

    let spending_key = derive_extended_spending_key(&master, derivation_path)
        .or_else(|err| Err(SpendingKeyError::caused_by(err.to_string())))?;

    let mut bytes: Vec<u8> = vec![];
    spending_key.write(&mut bytes).or_else(|err| Err(SpendingKeyError::caused_by(err.to_string())))?;

    Ok(bytes)
}

fn derive_extended_spending_key(key: &ExtendedSpendingKey, derivation_path: &str) -> Result<ExtendedSpendingKey, DerivationPathError> {
    let derivation_indices = split_derivation_path(derivation_path)?;
    let spending_key = ExtendedSpendingKey::from_path(key, &derivation_indices);

    Ok(spending_key)
}

#[cfg(test)]
mod test {
    use super::*;
    use bip39::{Language, Mnemonic, Seed};

    const MNEMONIC: &str = "sweet banner danger symptom ring skill benefit kitten appear much comic grab oak number coach";
    const PASSWORD: &str = "";
    const DERIVATION_PATH: &str = "m/";

    #[test]
    fn generates_extended_spending_key_from_mnemonic() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let _bytes = generate_extended_spending_key(seed.as_bytes(), DERIVATION_PATH).unwrap();

        // TODO: assert the key is correct
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_empty() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let error = generate_extended_spending_key(seed.as_bytes(), "");

        assert_eq!(error, Err(SpendingKeyError::caused_by(DerivationPathError::Empty.to_string())));
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_missing_prefix() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let error = generate_extended_spending_key(seed.as_bytes(), "/44'/123'/0'/0/0");

        assert_eq!(error, Err(SpendingKeyError::caused_by(DerivationPathError::MissingPrefix.to_string())));
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_invalid() {
        let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, PASSWORD);

        let invalid_character = generate_extended_spending_key(seed.as_bytes(), "m/44'/123q/0'/0/0");
        let empty_index = generate_extended_spending_key(seed.as_bytes(), "m/44'//0'/0/0");

        assert_eq!(invalid_character, Err(SpendingKeyError::caused_by(DerivationPathError::InvalidCharacter(vec!["q".to_owned()]).to_string())));
        assert_eq!(empty_index, Err(SpendingKeyError::caused_by(DerivationPathError::EmptyIndex.to_string())));
    }
}