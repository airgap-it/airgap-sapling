use std::convert::TryInto;
use zcash_primitives::{
    jubjub::JubjubEngine,
    primitives::PaymentAddress,
    zip32::{ExtendedFullViewingKey, DiversifierIndex},
};

use super::errors::ViewingKeyError;

#[derive(Debug)]
pub struct SaplingAddress {
    pub index: [u8; 11],
    pub diversifier: [u8; 11],
    pub pkd: [u8; 32],
}

impl SaplingAddress {
    pub fn new<T: JubjubEngine>(
        index: DiversifierIndex,
        payment_address: PaymentAddress<T>
    ) -> SaplingAddress {
        let bytes = payment_address.to_bytes();

        let diversifier: [u8; 11] = bytes[..11].try_into().unwrap();
        let pkd: [u8; 32] = bytes[11..].try_into().unwrap();

        SaplingAddress {
            index: index.0,
            diversifier,
            pkd,
        }
    }
}

pub fn get_xfvk_address(
    xfvk: &ExtendedFullViewingKey,
    index: Option<[u8; 11]>
) -> Result<SaplingAddress, ViewingKeyError> {
    let index = index.map_or(DiversifierIndex::new(), DiversifierIndex);
    let (index, payment_address) = xfvk.address(index).or_else(|_| Err(ViewingKeyError::new()))?;

    Ok(SaplingAddress::new(index, payment_address))
}

pub fn get_next_xfvk_address(
    xfvk: &ExtendedFullViewingKey,
    index: [u8; 11]
) -> Result<SaplingAddress, ViewingKeyError> {
    let mut index = DiversifierIndex(index);
    index.increment().or_else(|_| Err(ViewingKeyError::caused_by("diversifier index overflow")))?;

    get_xfvk_address(xfvk, Some(index.0))
}

#[cfg(test)]
mod test {
    use crate::sapling_keys::get_xfvk;
    use super::*;

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    // from https://github.com/zcash/zcash/blob/master/src/gtest/test_zip32.cpp
    #[test]
    fn gets_address_from_extended_full_viewing_key() {
        // TODO: add cases with specified diversifier indices
        let test_data = vec![
            ("m/", None, SaplingAddress {
                index: [0; 11],
                diversifier: [0xd8, 0x62, 0x1b, 0x98, 0x1c, 0xf3, 0x00, 0xe9, 0xd4, 0xcc, 0x89],
                pkd: [0; 32] // TODO: set actual expected value
            }),
            ("m/1", None, SaplingAddress {
                index: [0; 11],
                diversifier: [0x8b, 0x41, 0x38, 0x32, 0x0d, 0xfa, 0xfd, 0x7b, 0x39, 0x97, 0x81],
                pkd: [0; 32] // TODO: set actual expected value
            }),
            ("m/1/2h", None,
             SaplingAddress {
                index: [0; 11],
                diversifier: [0xe8, 0xd0, 0x37, 0x93, 0xcd, 0xd2, 0xba, 0xcc, 0x9c, 0x70, 0x41],
                pkd: [0; 32] // TODO: set actual expected value
            }),
            ("m/1/2h/3", None, SaplingAddress {
                index: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                diversifier: [0x03, 0x0f, 0xfb, 0x26, 0x3a, 0x93, 0x9e, 0x23, 0x0e, 0x96, 0xdd],
                pkd: [0; 32] // TODO: set actual expected value
            }),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, i, addr)| {
                let viewing_key = get_xfvk(&SEED, path).unwrap();
                let actual = get_xfvk_address(&viewing_key, *i).unwrap();

                (actual, addr)
            });


        for (actual, expected) in actual_expected {
            assert_eq!(actual.index, expected.index);
            assert_eq!(actual.diversifier, expected.diversifier);
        }
    }

    #[test]
    fn gets_next_address_from_extended_full_viewing_key() {
        // TODO: add more test cases
        let test_data: Vec<([u8; 11], &str, SaplingAddress)> = vec![
            ([0; 11], "m/", SaplingAddress {
                index: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                diversifier: [0; 11], // TODO: set actual expected value,
                pkd: [0; 32] // TODO: set actual expected value
            }),
        ];

        let actual_expected = test_data.iter()
            .map(|(i, path, addr)| {
                let viewing_key = get_xfvk(&SEED, path).unwrap();
                let actual = get_next_xfvk_address(&viewing_key, *i).unwrap();

                (actual, addr)
            });


        for (actual, expected) in actual_expected {
            assert_eq!(actual.index, expected.index);
        }
    }

    #[test]
    fn fails_to_get_next_address_from_extended_full_viewing_key_on_diversifier_index_overflow() {
        let max_index: [u8; 11] = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
        let xfvk = get_xfvk(&SEED, "m/").unwrap();

        let error = get_next_xfvk_address(&xfvk, max_index).unwrap_err();

        assert_eq!(error, ViewingKeyError::caused_by("diversifier index overflow"))
    }
}