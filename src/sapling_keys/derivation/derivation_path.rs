use super::{
  derivation_junction::DerivationJunction,
  errors::DerivationPathError,
};

pub fn split_derivation_path(derivation_path: &str) -> Result<Vec<DerivationJunction>, DerivationPathError> {
  if derivation_path.len() == 0 {
    return Err(DerivationPathError::Empty)
  }

  if &derivation_path[..1] != "m" {
    return Err(DerivationPathError::MissingPrefix)
  }

  let junctions_iterator = derivation_path.split('/')
    .skip(1)
    .map(|s| DerivationJunction::from(s));

  let mut junctions: Vec<DerivationJunction> = vec![];
  for result in junctions_iterator {
    match result {
      Ok(junction) => junctions.push(junction),
      Err(error) => return Err(error)
    }
  }

  return Ok(junctions)
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
        DerivationJunction::new(44, true),
        DerivationJunction::new(123, true),
        DerivationJunction::new(0, true),
        DerivationJunction::new(0, false),
        DerivationJunction::new(0, false),
      ])
    );
  }

  #[test]
  fn fails_to_split_empty_path_with_empty_error() {
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