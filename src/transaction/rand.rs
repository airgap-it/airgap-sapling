use rand_core::{CryptoRng, OsRng, RngCore};

pub fn generate_rand_bytes(len: usize) -> Vec<u8> {
    let mut rng = OsRng;
    let mut buffer = vec![0u8; len];
    rng.fill_bytes(&mut buffer);
    
    buffer
}

pub fn generate_rand_scalar() -> jubjub::Scalar {
    let mut buffer = [0u8; 64];

    let mut rng = OsRng;
    rng.fill_bytes(&mut buffer);

    jubjub::Scalar::from_bytes_wide(&buffer)
}

pub fn generate_rand_scalar_bytes() -> [u8; 32] {
    generate_rand_scalar().to_bytes()
}