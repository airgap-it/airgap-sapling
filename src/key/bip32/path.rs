use crate::common::errors::{CausedBy, SaplingError};
use crate::key::bip32::errors::Bip32PathError;
use crate::key::bip32::index::Bip32Index;
use crate::key::bip32::index::create_index;

#[derive(Debug, PartialEq)]
pub struct Bip32Path {
    pub indices: Vec<Bip32Index>
}

impl Bip32Path {
    fn empty() -> Bip32Path {
        Bip32Path { indices: vec![] }
    }

    fn new(indices: &[Bip32Index]) -> Bip32Path {
        Bip32Path { indices: indices.to_owned() }
    }
}

pub fn split_path(path: &str) -> Result<Bip32Path, SaplingError> {
    assert_path_non_empty(path).map_err(SaplingError::caused_by)?;
    assert_path_prefixed(path).map_err(SaplingError::caused_by)?;

    let indices_iterator = path
        .trim_end_matches('/') // remove leading and trailing `/`
        .split('/') // split indices
        .skip(1) // skip the `m` prefix
        .map(|s| create_index(s));

    let indices = unwrap_valid_indices(indices_iterator)?;

    Ok(Bip32Path::new(&indices))
}

fn assert_path_non_empty(path: &str) -> Result<(), Bip32PathError> {
    if path.is_empty() {
        Err(Bip32PathError::Empty)
    } else {
        Ok(())
    }
}

fn assert_path_prefixed(path: &str) -> Result<(), Bip32PathError> {
    if &path[..1] != "m" {
        Err(Bip32PathError::MissingPrefix)
    } else {
        Ok(())
    }
}

fn unwrap_valid_indices<I>(indices: I) -> Result<Vec<Bip32Index>, SaplingError>
    where I: Iterator<Item = Result<Bip32Index, SaplingError>>
{
    let mut valid_indices: Vec<Bip32Index> = vec![];
    for result in indices {
        match result {
            Ok(index) => valid_indices.push(index),
            Err(err) => return Err(err)
        }
    }

    Ok(valid_indices)
}

#[cfg(test)]
mod tests {
    use crate::key::bip32::Bip32IndexError;

    use super::*;

    #[test]
    fn splits_valid_bip32_path() {
        let test_data = vec![
            ("m/", Bip32Path::empty()),
            ("m/44'/123'/0'/0/0", Bip32Path {
                indices: vec![
                    Bip32Index::Hardened(44),
                    Bip32Index::Hardened(123),
                    Bip32Index::Hardened(0),
                    Bip32Index::NonHardened(0),
                    Bip32Index::NonHardened(0),
                ]
            }),
            ("m/44h/123h/0h/0/0", Bip32Path {
                indices: vec![
                    Bip32Index::Hardened(44),
                    Bip32Index::Hardened(123),
                    Bip32Index::Hardened(0),
                    Bip32Index::NonHardened(0),
                    Bip32Index::NonHardened(0),
                ]
            }),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let actual = split_path(path).unwrap();

                (actual, v)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn fails_to_split_invalid_path_with_error() {
        let test_data = vec![
            ("", SaplingError::caused_by(Bip32PathError::Empty)),
            ("44'/123'/0'/0/0", SaplingError::caused_by(Bip32PathError::MissingPrefix)),
            ("m/44'//0'/0/0", SaplingError::caused_by(Bip32IndexError::Empty)),
            ("m/44'/123a/0'/0/0", SaplingError::caused_by(Bip32IndexError::InvalidCharacter(vec!["a".to_owned()]))),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let actual = split_path(path).unwrap_err();

                (actual, v)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }
}