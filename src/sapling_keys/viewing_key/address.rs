use zcash_primitives::{
    zip32::{ExtendedFullViewingKey, DiversifierIndex},
};

use super::errors::ViewingKeyError;
use zcash_primitives::primitives::PaymentAddress;
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
    let diversifier_index = index.map_or(DiversifierIndex::new(), |index| DiversifierIndex(index));
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