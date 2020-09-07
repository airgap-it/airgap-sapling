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
        let test_data = vec![
            ("m/", vec![]),
            ("m/44'/123'/0'/0/0", vec![
                DerivationIndex::Hardened(44),
                DerivationIndex::Hardened(123),
                DerivationIndex::Hardened(0),
                DerivationIndex::NonHardened(0),
                DerivationIndex::NonHardened(0),
            ]),
            ("m/44h/123h/0h/0/0", vec![
                DerivationIndex::Hardened(44),
                DerivationIndex::Hardened(123),
                DerivationIndex::Hardened(0),
                DerivationIndex::NonHardened(0),
                DerivationIndex::NonHardened(0),
            ]),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let actual = split_derivation_path(path).unwrap();

                (actual, v)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn fails_to_split_invalid_path_with_error() {
        let test_data = vec![
            ("", DerivationPathError::Empty),
            ("44'/123'/0'/0/0", DerivationPathError::MissingPrefix),
            ("m/44'//0'/0/0", DerivationPathError::EmptyIndex),
            ("m/44'/123a/0'/0/0", DerivationPathError::InvalidCharacter(vec!["a".to_owned()])),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let actual = split_derivation_path(path).unwrap_err();

                (actual, v)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }
}