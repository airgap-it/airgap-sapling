use zcash_primitives::zip32::{DiversifierIndex, ExtendedFullViewingKey};

use crate::address::indexed_address::IndexedAddress;
use crate::common::errors::{CausedBy, SaplingError};

use super::errors::SaplingAddressError;

pub fn get_xfvk_address(
    xfvk: &ExtendedFullViewingKey,
    index: Option<[u8; 11]>
) -> Result<IndexedAddress, SaplingError> {
    let (index, payment_address) = match index {
        Some(index) => xfvk.address(DiversifierIndex(index)),
        None => xfvk.default_address()
    }.map_err(|_| SaplingAddressError::DiversifierSpaceExhausted).map_err(SaplingError::caused_by)?;

    Ok(IndexedAddress::new(index, payment_address))
}

pub fn get_next_xfvk_address(
    xfvk: &ExtendedFullViewingKey,
    index: [u8; 11]
) -> Result<IndexedAddress, SaplingError> {
    let mut index = DiversifierIndex(index);
    index.increment().map_err(|_| SaplingAddressError::DiversifierSpaceExhausted).map_err(SaplingError::caused_by)?;

    get_xfvk_address(xfvk, Some(index.0))
}

#[cfg(test)]
mod tests {
    use group::{Group, GroupEncoding};
    use zcash_primitives::primitives::{Diversifier, PaymentAddress};

    use crate::key::SaplingKey;

    use super::*;
    use rand_core::OsRng;

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    // from https://github.com/zcash/zcash/blob/master/src/gtest/test_zip32.cpp
    #[test]
    fn gets_address_from_extended_full_viewing_key() {
        let mut rng = OsRng;

        // TODO: add cases with specified diversifier indices
        let test_data: Vec<(&str, Option<[u8; 11]>, IndexedAddress)> = vec![
            ("m/", None, IndexedAddress(
                [0; 11],
                PaymentAddress::from_parts(
                    Diversifier([0xd8, 0x62, 0x1b, 0x98, 0x1c, 0xf3, 0x00, 0xe9, 0xd4, 0xcc, 0x89]),
                    jubjub::SubgroupPoint::random(&mut rng) // TODO: set actual expected value
                ).unwrap(),
            )),
            ("m/1", None, IndexedAddress(
                [0; 11],
                PaymentAddress::from_parts(
                    Diversifier([0x8b, 0x41, 0x38, 0x32, 0x0d, 0xfa, 0xfd, 0x7b, 0x39, 0x97, 0x81]),
                    jubjub::SubgroupPoint::random(&mut rng) // TODO: set actual expected value
                ).unwrap(),
            )),
            ("m/1/2h", None, IndexedAddress(
                 [0; 11],
                 PaymentAddress::from_parts(
                     Diversifier([0xe8, 0xd0, 0x37, 0x93, 0xcd, 0xd2, 0xba, 0xcc, 0x9c, 0x70, 0x41]),
                     jubjub::SubgroupPoint::random(&mut rng) // TODO: set actual expected value
                 ).unwrap(),
             )),
            ("m/1/2h/3", None, IndexedAddress(
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                PaymentAddress::from_parts(
                    Diversifier([0x03, 0x0f, 0xfb, 0x26, 0x3a, 0x93, 0x9e, 0x23, 0x0e, 0x96, 0xdd]),
                    jubjub::SubgroupPoint::random(&mut rng) // TODO: set actual expected value
                ).unwrap()
            )),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, i, addr)| {
                let viewing_key = ExtendedFullViewingKey::from_seed(&SEED, path).unwrap();
                let actual = get_xfvk_address(&viewing_key, *i).unwrap();

                (actual, addr)
            });


        for (actual, expected) in actual_expected {
            assert_eq!(actual.0, expected.0);
            assert_eq!(actual.1.diversifier(), expected.1.diversifier());
        }
    }

    #[test]
    fn gets_next_address_from_extended_full_viewing_key() {
        let mut rng = OsRng;

        // TODO: add more test cases
        let test_data: Vec<([u8; 11], &str, IndexedAddress)> = vec![
            ([0; 11], "m/", IndexedAddress(
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                PaymentAddress::from_parts(
                    Diversifier([0; 11]), // TODO: set actual expected value,
                    jubjub::SubgroupPoint::random(&mut rng) // TODO: set actual expected value
                ).unwrap(),
            )),
        ];

        let actual_expected = test_data.iter()
            .map(|(i, path, addr)| {
                let viewing_key = ExtendedFullViewingKey::from_seed(&SEED, path).unwrap();
                let actual = get_next_xfvk_address(&viewing_key, *i).unwrap();

                (actual, addr)
            });


        for (actual, expected) in actual_expected {
            assert_eq!(actual.0, expected.0);
        }
    }

    #[test]
    fn fails_to_get_next_address_from_extended_full_viewing_key_on_diversifier_index_overflow() {
        let max_index: [u8; 11] = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
        let xfvk = ExtendedFullViewingKey::from_seed(&SEED, "m/").unwrap();

        let error = get_next_xfvk_address(&xfvk, max_index).unwrap_err();

        assert_eq!(error, SaplingError::caused_by(SaplingAddressError::DiversifierSpaceExhausted))
    }
}