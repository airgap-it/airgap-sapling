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
pub fn get_ext_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, JsValue> {
  generate_ext_spending_key(seed, derivation_path).or_else(|err| Err(JsValue::from(err.to_string())))
}

fn generate_ext_spending_key(seed: &[u8], derivation_path: &str) -> Result<Vec<u8>, SpendingKeyError> {
  let master = ExtendedSpendingKey::master(seed);
  
  let ext_spending_key = derive_ext_spending_key(&master, derivation_path)
      .or_else(|err| Err(SpendingKeyError::caused_by(err.to_string())))?;

  let mut bytes: Vec<u8> = vec![];
  ext_spending_key.write(&mut bytes).or_else(|err| Err(SpendingKeyError::caused_by(err.to_string())))?;

  Ok(bytes)
}

fn derive_ext_spending_key(key: &ExtendedSpendingKey, derivation_path: &str) -> Result<ExtendedSpendingKey, DerivationPathError> {
  let derivation_indices = split_derivation_path(derivation_path)?;
  let ext_spending_key = ExtendedSpendingKey::from_path(key, &derivation_indices);

  Ok(ext_spending_key)
}

#[cfg(test)]
mod test {
  use super::*;
  use bip39::{Language, Mnemonic, Seed};

  const MNEMONIC: &str = "sweet banner danger symptom ring skill benefit kitten appear much comic grab oak number coach";
  const PASSWORD: &str = "";
  const DERIVATION_PATH: &str = "m/";

  #[test]
  fn generates_ext_spending_key_from_mnemonic() {
    let mnemonic = Mnemonic::from_phrase(MNEMONIC, Language::English).unwrap();
    let seed = Seed::new(&mnemonic, PASSWORD);

    let bytes = get_ext_spending_key(seed.as_bytes(), DERIVATION_PATH);

    // TODO
  }
}