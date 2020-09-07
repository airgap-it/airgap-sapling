use super::{
    index::{
        Bip32Index,
        create_bip32_index,
    },
    errors::Bip32Error,
};

pub fn split_bip32_path(path: &str) -> Result<Vec<Bip32Index>, Bip32Error> {
    assert_path_non_empty(path)?;
    assert_path_prefixed(path)?;

    let indices_iterator = path
        .trim_end_matches('/') // remove leading and trailing `/`
        .split('/') // split indices
        .skip(1) // skip the `m` prefix
        .map(|s| create_bip32_index(s));

    let indices = unwrap_valid_indices(indices_iterator)?;

    Ok(indices)
}

fn assert_path_non_empty(path: &str) -> Result<(), Bip32Error> {
    if path.is_empty() {
        Err(Bip32Error::EmptyPath)
    } else {
        Ok(())
    }
}

fn assert_path_prefixed(path: &str) -> Result<(), Bip32Error> {
    if &path[..1] != "m" {
        Err(Bip32Error::MissingPrefix)
    } else {
        Ok(())
    }
}

fn unwrap_valid_indices<I>(indices: I) -> Result<Vec<Bip32Index>, Bip32Error>
    where I: Iterator<Item = Result<Bip32Index, Bip32Error>>
{
    let mut valid_indices: Vec<Bip32Index> = vec![];
    for result in indices {
        match result {
            Ok(index) => valid_indices.push(index),
            Err(error) => return Err(error)
        }
    }

    Ok(valid_indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_valid_bip32_path() {
        let test_data = vec![
            ("m/", vec![]),
            ("m/44'/123'/0'/0/0", vec![
                Bip32Index::Hardened(44),
                Bip32Index::Hardened(123),
                Bip32Index::Hardened(0),
                Bip32Index::NonHardened(0),
                Bip32Index::NonHardened(0),
            ]),
            ("m/44h/123h/0h/0/0", vec![
                Bip32Index::Hardened(44),
                Bip32Index::Hardened(123),
                Bip32Index::Hardened(0),
                Bip32Index::NonHardened(0),
                Bip32Index::NonHardened(0),
            ]),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let actual = split_bip32_path(path).unwrap();

                (actual, v)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn fails_to_split_invalid_path_with_error() {
        let test_data = vec![
            ("", Bip32Error::EmptyPath),
            ("44'/123'/0'/0/0", Bip32Error::MissingPrefix),
            ("m/44'//0'/0/0", Bip32Error::EmptyIndex),
            ("m/44'/123a/0'/0/0", Bip32Error::InvalidCharacter(vec!["a".to_owned()])),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let actual = split_bip32_path(path).unwrap_err();

                (actual, v)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }
}