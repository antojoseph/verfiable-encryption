# Verifiable Encryption Template Based on RISC0

Welcome to the Verifiable Encryption Template based on RISC0! This template demonstrates a complete host–guest interaction where the host generates an RSA key pair and passes the public key (encoded as a comma‑separated hexadecimal string) to the guest. The guest then decodes this key, uses it to encrypt a test message with PKCS#1 v1.5 encryption, and commits the resulting ciphertext to the journal. The host retrieves and prints this ciphertext.

For an overview of the concepts behind the zkVM, see the [zkVM Overview][zkvm-overview].

## Quick Start

First, make sure [rustup] is installed. The [`rust-toolchain.toml`][rust-toolchain] file will automatically install the correct version.

To build and run both the host and guest programs, simply run:

```bash
cargo run

What This Template Does
	•	Host Program:
	•	Generates a 2048‑bit RSA key pair using a secure RNG.
	•	Extracts the public key and converts its modulus (n) and exponent (e) to hexadecimal strings.
	•	Encodes the public key as a comma‑separated string in the format "n_hex,e_hex".
	•	Supplies this encoded public key as input to the guest via the Executor Environment.
	•	After execution, reads the guest’s journal, decodes the committed ciphertext (which is 256 bytes long for a 2048‑bit key), and prints it as a hex string.
	•	Guest Program:
	•	Reads the RSA public key string from the host.
	•	Decodes the key by splitting the string into its hexadecimal components and converting them into BigUint values.
	•	Constructs an RsaPublicKey and uses it to encrypt a test message (“hello world”) with PKCS#1 v1.5 encryption.
	•	Commits the resulting ciphertext to the journal.

Executing the Project Locally in Development Mode

For faster iteration during development, run your project in dev-mode (which enables debugging and faster builds) with execution statistics. For example:

RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
Running Proofs Remotely on Bonsai

Note: The Bonsai proving service is still in early Alpha; an API key is required for access. Click here to request access.

If you have access to Bonsai, run your proofs remotely by providing the necessary environment variables:
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
How the RSA Demo Works
	1.	Host Side:
	•	A new RSA key pair is generated using a secure RNG.
	•	The public key’s modulus (n) and exponent (e) are converted to hexadecimal strings and concatenated (e.g. "n_hex,e_hex").
	•	This encoded public key is sent as input to the guest via the Executor Environment.
	•	After the guest finishes execution, the host decodes the guest’s journal (which contains the RSA-encrypted ciphertext) and prints the ciphertext as a hexadecimal string.
	2.	Guest Side:
	•	The guest reads the public key string from the host.
	•	It parses the string by splitting it on the comma and converting each part from hexadecimal into BigUint values.
	•	Using the reconstructed public key, it encrypts a test message (“hello world”) using PKCS#1 v1.5 padding.
	•	Finally, it commits the ciphertext (which will be 256 bytes for a 2048‑bit RSA key) to the journal.

Directory Structure

A common directory structure for zkVM applications is used:
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs       <-- [Host code: RSA key generation & guest execution]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs   <-- [Guest code: public key decoding & RSA encryption]
    └── src
        └── lib.rs

