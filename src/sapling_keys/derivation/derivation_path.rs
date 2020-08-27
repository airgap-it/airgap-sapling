
use super::{
  derivation_index::{
    DerivationIndex,
    create_derivation_index,
  },
  errors::DerivationPathError,
};

pub fn split_derivation_path(derivation_path: &str) -> Result<Vec<DerivationIndex>, DerivationPathError> {
  if derivation_path.len() == 0 {
    return Err(DerivationPathError::Empty)
  }

  if &derivation_path[..1] != "m" {
    return Err(DerivationPathError::MissingPrefix)
  }

  let junctions_iterator = derivation_path
      .trim_end_matches('/')
      .split('/')
      .skip(1)
      .map(|s| create_derivation_index(s));

  let mut derivation_indices: Vec<DerivationIndex> = vec![];
  for result in junctions_iterator {
    match result {
      Ok(junction) => derivation_indices.push(junction),
      Err(error) => return Err(error)
    }
  }

  return Ok(derivation_indices)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn splits_valid_derivation_path() {
    let derivation_junctions = split_derivation_path("m/44'/123'/0'/0/0");

    assert_eq!(
      derivation_junctions, 
      Ok(vec![
        DerivationIndex::Hardened(44),
        DerivationIndex::Hardened(123),
        DerivationIndex::Hardened(0),
        DerivationIndex::NonHardened(0),
        DerivationIndex::NonHardened(0),
      ])
    );
  }

  #[test]
  fn splits_empty_derivation_path() {
    let empty = split_derivation_path("m/");

    assert_eq!(empty, Ok(vec![]));
  }

  #[test]
  fn fails_to_split_empty_string_with_empty_error() {
    let empty = split_derivation_path("");

    assert_eq!(empty, Err(DerivationPathError::Empty));
  }

  #[test]
  fn fails_to_split_missing_prefix_path_with_missing_prefix_error() {
    let missing_prefix = split_derivation_path("44'/123'/0'/0/0");

    assert_eq!(missing_prefix, Err(DerivationPathError::MissingPrefix));
  }

  #[test]
  fn fails_to_split_invalid_path_with_error() {
    let empty_junction = split_derivation_path("m/44'//0'/0/0");
    let invalid_junction = split_derivation_path("m/44'/123a/0'/0/0");

    assert_eq!(empty_junction, Err(DerivationPathError::EmptyJunction));
    assert_eq!(invalid_junction, Err(DerivationPathError::InvalidCharacter(vec!["a".to_owned()])));
  }
}