use super::{
    derivation_index::{
        DerivationIndex,
        create_derivation_index,
    },
    errors::DerivationPathError,
};

pub fn split_derivation_path(path: &str) -> Result<Vec<DerivationIndex>, DerivationPathError> {
    assert_path_non_empty(path)?;
    assert_path_prefixed(path)?;

    let indices_iterator = path
        .trim_end_matches('/') // remove leading and trailing `/`
        .split('/') // split indices
        .skip(1) // skip the `m` prefix
        .map(|s| create_derivation_index(s));

    let indices = unwrap_valid_indices(indices_iterator)?;

    Ok(indices)
}

fn assert_path_non_empty(path: &str) -> Result<(), DerivationPathError> {
    if path.is_empty() {
        Err(DerivationPathError::Empty)
    } else {
        Ok(())
    }
}

fn assert_path_prefixed(path: &str) -> Result<(), DerivationPathError> {
    if &path[..1] != "m" {
        Err(DerivationPathError::MissingPrefix)
    } else {
        Ok(())
    }
}

fn unwrap_valid_indices<I>(indices: I) -> Result<Vec<DerivationIndex>, DerivationPathError>
    where I: Iterator<Item = Result<DerivationIndex, DerivationPathError>>
{
    let mut valid_indices: Vec<DerivationIndex> = vec![];
    for result in indices {
        match result {
            Ok(index) => valid_indices.push(index),
            Err(error) => return Err(error)
        }
    }

    Ok(valid_indices)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn splits_valid_derivation_path() {
        let indices_apostrophes = split_derivation_path("m/44'/123'/0'/0/0");
        let indices_hs = split_derivation_path("m/44h/123h/0h/0/0");

        let expected = Ok(vec![
            DerivationIndex::Hardened(44),
            DerivationIndex::Hardened(123),
            DerivationIndex::Hardened(0),
            DerivationIndex::NonHardened(0),
            DerivationIndex::NonHardened(0),
        ]);

        assert_eq!(indices_apostrophes, expected);
        assert_eq!(indices_hs, expected);
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
        let empty = split_derivation_path("m/44'//0'/0/0");
        let invalid = split_derivation_path("m/44'/123a/0'/0/0");

        assert_eq!(empty, Err(DerivationPathError::EmptyIndex));
        assert_eq!(invalid, Err(DerivationPathError::InvalidCharacter(vec!["a".to_owned()])));
    }
}