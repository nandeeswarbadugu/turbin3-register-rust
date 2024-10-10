// Contents: keygen function to generate a new Solana wallet
use solana_sdk::signature::{Keypair, Signer};

/// Generates a new Solana wallet and prints the public and private keys.
pub fn keygen() -> Keypair {
    let kp = Keypair::new();
    println!("New Solana wallet created: {}", kp.pubkey());
    println!("Save the following private key into a JSON file:");
    println!("{:?}", kp.to_bytes());
    kp
}

