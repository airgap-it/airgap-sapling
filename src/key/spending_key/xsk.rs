use zcash_primitives::zip32::ExtendedSpendingKey;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::key::bip32::split_bip32_path;
use crate::key::sapling_key::SaplingKey;

use super::errors::SpendingKeyError;

impl SaplingKey for ExtendedSpendingKey {
    fn from_seed(seed: &[u8], derivation_path: &str) -> Result<Self, SaplingError> where Self: Sized {
        let master_key = ExtendedSpendingKey::master(seed);
        let bip32_path = split_bip32_path(derivation_path)?;
        let xsk = ExtendedSpendingKey::from_path(&master_key, &bip32_path.indices);

        Ok(xsk)
    }
}

impl Serializable<Vec<u8>, SaplingError> for ExtendedSpendingKey {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> where Self: Sized {
        ExtendedSpendingKey::read(&serialized[..]).map_err(|err| SaplingError::caused_by(SpendingKeyError::ReadFailed(err)))
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        let mut bytes: Vec<u8> = vec![];
        self.write(&mut bytes).map_err(|err| SaplingError::caused_by(SpendingKeyError::WriteFailed(err)))?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use hex;

    use crate::key::bip32::{Bip32IndexError, Bip32PathError};

    use super::*;

    const SEED: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    // from https://github.com/zcash/zcash/blob/master/src/gtest/test_zip32.cpp
    #[test]
    fn generates_extended_spending_key_from_seed() {
        let test_data = vec![
            ("m/", [
                "00", // depth (1 byte)
                "00000000", // parent_fvk_tag (4 bytes, LE)
                "00000000", // child_index (4 bytes, LE)
                "d0947c4b03bf72a37ab44f72276d1cf3fdcd7ebf3e73348b7e550d752018668e", // chain_code (32 bytes, LE)
                "b6c00c93d36032b9a268e99e86a860776560bf0e83c1a10b51f607c954742506", // expsk.ask (32 bytes, LE)
                "8204ede83b2f1fbd84f9b45d7f996e2ebd0a030ad243b48ed39f748a8821ea06", // expsk.nsk (32 bytes, LE)
                "395884890323b9d4933c021db89bcf767df21977b2ff0683848321a4df4afb21", // expsk.ovk (32 bytes, LE)
                "77c17cb75b7796afb39f0f3e91c924607da56fa9a20e283509bc8a3ef996a172", // dk (32 bytes, LE)
            ]),
            ("m/1", [
                "01", // depth (1 byte)
                "14c2713a", // parent_fvk_tag (4 bytes, LE)
                "01000000", // child_index (4 bytes, LE)
                "0147110c691a03b9d9f0ba9005c5e790a595b7f04e3329d2fa438a6705dabce6", // chain_code (32 bytes, LE)
                "282bc197a516287c8ea8f68c424abad302b45cdf95407961d7b8b455267a350c", // expsk.ask (32 bytes, LE)
                "e7a32988fdca1efcd6d1c4c562e629c2e96b2c3f7eda04ac4efd1810ff6bba01", // expsk.nsk (32 bytes, LE)
                "5f1381fc8886da6a02dffeefcf503c40fa8f5a36f7a7142fd81b5518c5a47474", // expsk.ovk (32 bytes, LE)
                "e04de832a2d791ec129ab9002b91c9e9cdeed79241a7c4960e5178d870c1b4dc", // dk (32 bytes, LE)
            ]),
            ("m/1/2h", [
                "02", // depth (1 byte)
                "db999e07", // parent_fvk_tag (4 bytes, LE)
                "02000080", // child_index (4 bytes, LE)
                "97ce15f4ed1b9739b0262a463bcb3dc9b3bd2323a9baa441ca42777383a8d435", // chain_code (32 bytes, LE)
                "8be8113cee3413a71f82c41fc8da517be134049832e6825c92da6b84fee4c60d", // expsk.ask (32 bytes, LE)
                "3778059dc569e7d0d32391573f951bbde92fc6b9cf614773661c5c273aa6990c", // expsk.nsk (32 bytes, LE)
                "cf81182e96223c028ce3d6eb4794d3113b95069d14c57588e193b65efc2813bc", // expsk.ovk (32 bytes, LE)
                "a3eda19f9eff46ca12dfa1bf10371b48d1b4a40c4d05a0d8dce0e7dc62b07b37", // dk (32 bytes, LE)
            ]),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, v)| {
                let xsk = ExtendedSpendingKey::from_seed(&SEED, path).unwrap();
                let actual = xsk.serialize().unwrap();
                let expected: Vec<u8> = v.iter().flat_map(|&s| hex::decode(s).unwrap()).collect();

                (actual, expected)
            });


        for (actual, expected) in actual_expected {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn fails_to_generate_extended_spending_key_when_derivation_path_invalid() {
        let test_data = vec![
            ("", SaplingError::caused_by(Bip32PathError::Empty)),
            ("/44'/123'/0'/0/0", SaplingError::caused_by(Bip32PathError::MissingPrefix)),
            ("m/44'/123q/0'/0/0", SaplingError::caused_by(Bip32IndexError::InvalidCharacter(vec!["q".to_string()]))),
            ("m/44'//0'/0/0", SaplingError::caused_by(Bip32IndexError::Empty)),
        ];

        let actual_expected = test_data.iter()
            .map(|(path, err)| {
                let actual = ExtendedSpendingKey::from_seed(&SEED, path).unwrap_err();

                (actual, err)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected);
        }
    }
    
    #[test]
    fn reads_extended_spending_key_from_bytes() {
        let expected = ExtendedSpendingKey::from_seed(&SEED, "m/").unwrap();
        let bytes = expected.serialize().unwrap();
        let actual = ExtendedSpendingKey::deserialize(bytes).unwrap();
        
        assert_eq!(actual, expected);
    }
}