use methods::{GUEST_CODE_ELF, GUEST_CODE_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use rsa::{RsaPrivateKey, RsaPublicKey}; 
use rsa::traits::PublicKeyParts;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new RSA key pair (2048 bits)
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .expect("Failed to generate RSA key pair");
    
    // Extract the public key.
    let public_key: RsaPublicKey = private_key.to_public_key();

    // Convert the modulus (n) and exponent (e) to hexadecimal strings.
    let n_hex = public_key.n().to_str_radix(16);
    let e_hex = public_key.e().to_str_radix(16);
    let encoded_public_key = format!("{},{}", n_hex, e_hex);

    // For debugging, print the encoded public key.
    println!("Encoded public key (to be sent to guest): {}", encoded_public_key);

    // Create an ExecutorEnv that supplies the encoded public key as input to the guest.
    // Note: Since `encoded_public_key` is a String (which is Sized), we pass it directly.
    let env = ExecutorEnv::builder()
        .write(&encoded_public_key)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Run the guest program.
    let prove_info = prover.prove(env, GUEST_CODE_ELF).unwrap();
    let receipt = prove_info.receipt;

    // Decode the journal as a Vec<u8> (this is the encrypted data).
    let enc_data: Vec<u8> = receipt.journal.decode().unwrap();

    // Print the encrypted data as a hexadecimal string.
    println!("Encrypted data (hex): {}", hex::encode(&enc_data));

    // Verify the receipt.
    receipt.verify(GUEST_CODE_ID).unwrap();

    Ok(())
}