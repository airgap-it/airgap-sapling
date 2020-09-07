use std::convert::TryInto;
use zcash_primitives::{
    primitives::PaymentAddress,
    jubjub::JubjubEngine,
    zip32::DiversifierIndex,
};

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