#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use risc0_zkvm::guest::env;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey, BigUint};
use rsa::traits::PublicKeyParts;
use num_traits::Num;
use rand::rngs::OsRng; // Or use a deterministic RNG if needed

/// Parses an encoded RSA public key in the format "n_hex,e_hex"
/// and returns a tuple (n, e) as BigUint values.
fn parse_public_key(encoded: &str) -> Option<(BigUint, BigUint)> {
    let parts: Vec<&str> = encoded.trim().split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let n = BigUint::from_str_radix(parts[0], 16).ok()?;
    let e = BigUint::from_str_radix(parts[1], 16).ok()?;
    Some((n, e))
}

pub fn main() {
    // Read the encoded public key as a String from the host.
    let encoded: String = env::read();

    // Parse the encoded public key.
    let (n, e) = parse_public_key(&encoded).expect("Failed to parse public key");
    
    // Build an RsaPublicKey using the unchecked constructor.
    let public_key = RsaPublicKey::new_unchecked(n, e);
    
    // Encrypt a test message.
    let mut rng = OsRng;
    let data = b"hello world";
    let enc_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .expect("failed to encrypt");
    
    // Commit ONLY the encrypted data to the journal.
    env::commit(&enc_data);
}