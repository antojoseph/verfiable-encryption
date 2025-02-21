use methods::{GUEST_CODE_ELF, GUEST_CODE_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rsa::traits::PublicKeyParts;
use rand::rngs::OsRng;
use anyhow::Context;
use risc0_ethereum_contracts::encode_seal;
use bincode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- RSA Key Generation ---
    // Generate a new RSA key pair (2048 bits)
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .expect("Failed to generate RSA key pair");
    let public_key: RsaPublicKey = private_key.to_public_key();

    // Convert the modulus and exponent to hexadecimal strings.
    let n_hex = public_key.n().to_str_radix(16);
    let e_hex = public_key.e().to_str_radix(16);
    let encoded_public_key = format!("{},{}", n_hex, e_hex);
    println!("Encoded public key (to be sent to guest): {}", encoded_public_key);

    // --- Environment Setup and Local Proving ---
    // Build an ExecutorEnv that supplies the encoded public key as input to the guest.
    let env = ExecutorEnv::builder()
        .write(&encoded_public_key)
        .unwrap()
        .build()
        .unwrap();

    // Run the guest program using prove_with_ctx so that the receipt is generated with Groth16 options.
    let prove_info = default_prover()
        .prove_with_ctx(env, &VerifierContext::default(), GUEST_CODE_ELF, &ProverOpts::groth16())
        .unwrap();
    let receipt = prove_info.receipt;

    // --- Journal Extraction and Decryption ---
    // Decode the journal as a Vec<u8> (this is the RSA-encrypted ciphertext).
    let enc_data: Vec<u8> = receipt.journal.decode().unwrap();
    println!("Encrypted data (hex): {}", hex::encode(&enc_data));

    // Decrypt the ciphertext using the RSA private key.
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    let plaintext = String::from_utf8(dec_data)
        .expect("Decrypted data is not valid UTF-8");
    println!("Decrypted plaintext: {}", plaintext);

    // --- Seal & Proof Serialization ---
    // Encode the seal from the receipt.
    let seal = encode_seal(&receipt).with_context(|| "Encoding seal failed")?;
    println!("Seal (hex): {}", hex::encode(&seal));

    // Clone the journal bytes (to avoid moving out of receipt).
    let journal_bytes = receipt.journal.bytes.clone();
    println!("Journal (hex): {}", hex::encode(&journal_bytes));

    // Serialize the inner receipt proof using bincode.
    let proof_bytes = bincode::serialize(&receipt.inner)?;
    println!("Proof (hex): {}", hex::encode(&proof_bytes));

    // --- Receipt Verification ---
    receipt.verify(GUEST_CODE_ID).unwrap();

    Ok(())
}