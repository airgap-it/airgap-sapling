use zcash_primitives::{
    zip32::{ExtendedFullViewingKey, DiversifierIndex},
};

use super::errors::ViewingKeyError;
use zcash_primitives::primitives::{PaymentAddress};
use zcash_primitives::jubjub::JubjubEngine;

pub struct SaplingAddress {
    pub index: [u8; 11],
    pub payload: [u8; 43],
}

impl SaplingAddress {
    pub fn new<T: JubjubEngine>(
        diversifier_index: DiversifierIndex,
        payment_address: PaymentAddress<T>
    ) -> SaplingAddress {
        SaplingAddress {
            index: diversifier_index.0,
            payload: payment_address.to_bytes(),
        }
    }
}

pub fn get_address_from_viewing_key(
    viewing_key: &ExtendedFullViewingKey,
    index: Option<[u8; 11]>
) -> Result<SaplingAddress, ViewingKeyError> {
    let diversifier_index = index.map_or(DiversifierIndex::new(), DiversifierIndex);
    let (diversifier_index, payment_address) = viewing_key.address(diversifier_index)
        .or_else(|_| Err(ViewingKeyError::new()))?;

    Ok(SaplingAddress::new(diversifier_index, payment_address))
}

pub fn get_address_from_viewing_key_bytes(
    bytes: &[u8],
    index: Option<[u8; 11]>
) -> Result<SaplingAddress, ViewingKeyError> {
    let viewing_key = ExtendedFullViewingKey::read(bytes)
        .or_else(|err| Err(ViewingKeyError::caused_by(err)))?;

    get_address_from_viewing_key(&viewing_key, index)
}

#[cfg(test)]
mod test {
    use crate::sapling_keys::{
        get_extended_full_viewing_key,
        get_extended_full_viewing_key_bytes,
    };
    use super::*;

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    // from https://github.com/zcash/zcash/blob/master/src/gtest/test_zip32.cpp
    #[test]
    fn gets_address_from_viewing_key() {
        let test_data = vec![
            ("m/", None, [0xd8, 0x62, 0x1b, 0x98, 0x1c, 0xf3, 0x00, 0xe9, 0xd4, 0xcc, 0x89]),
            ("m/1", None, [0x8b, 0x41, 0x38, 0x32, 0x0d, 0xfa, 0xfd, 0x7b, 0x39, 0x97, 0x81]),
            ("m/1/2h", None, [0xe8, 0xd0, 0x37, 0x93, 0xcd, 0xd2, 0xba, 0xcc, 0x9c, 0x70, 0x41]),
            ("m/1/2h/3", None, [0x03, 0x0f, 0xfb, 0x26, 0x3a, 0x93, 0x9e, 0x23, 0x0e, 0x96, 0xdd]),
        ];

        let actual_expected = test_data.iter()
            .map(|&(path, i, v)| {
                let viewing_key = get_extended_full_viewing_key(&SEED, path).unwrap();
                let viewing_key_bytes = get_extended_full_viewing_key_bytes(&SEED, path).unwrap();

                let actual = get_address_from_viewing_key(&viewing_key, i).unwrap();
                let actual_from_bytes = get_address_from_viewing_key_bytes(&viewing_key_bytes, i).unwrap();

                (actual, actual_from_bytes, v)
            });


        for (actual, actual_from_bytes, expected) in actual_expected {
            assert_eq!(actual.payload[..11], expected);
            assert_eq!(actual_from_bytes.payload[..11], expected);
        }
    }
}