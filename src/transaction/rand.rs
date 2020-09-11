use rand_core::{CryptoRng, OsRng, RngCore};

pub fn generate_rand_bytes(len: usize) -> Vec<u8> {
    let mut rng = OsRng;
    let mut buffer = vec![0u8; len];
    rng.fill_bytes(&mut buffer);
    
    buffer
}

pub fn generate_rand_scalar<R: RngCore + CryptoRng>(rng: Option<&mut R>) -> jubjub::Scalar {
    let mut buffer = [0u8; 64];

    match rng {
        Some(rng) => rng.fill_bytes(&mut buffer),
        None => {
            let mut rng = OsRng;
            rng.fill_bytes(&mut buffer);
        },
    }

    jubjub::Scalar::from_bytes_wide(&buffer)
}