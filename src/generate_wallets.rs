use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use rand::Rng;
use secp256k1::{Secp256k1, SecretKey};
use std::fs::OpenOptions;
use std::io::{self, Write};
use crate::generate_password::get_random_phrase;

fn generate_private_key() -> SecretKey {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    SecretKey::from_slice(&key).expect("Slice with incorrect length")
}

fn generate_xpub(private_key: &SecretKey) -> ExtendedPubKey {
    let secp = Secp256k1::new();
    let extended_priv_key =
        ExtendedPrivKey::new_master(Network::Bitcoin, &private_key[..]).unwrap();
    ExtendedPubKey::from_private(&secp, &extended_priv_key)
}

pub fn generate_wallets_to_csv(csv_path: &str, number_of_wallets: usize) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(csv_path)?;

    for _ in 0..number_of_wallets {
        let private_key = generate_private_key();
        let xpub = generate_xpub(&private_key);
        let seed_phrase = get_random_phrase().join(" ");

        writeln!(file, "{},{},{}", seed_phrase, private_key.display_secret(), xpub)?;
    }

    Ok(())
}

fn main() {
    let csv_path = "./wallets.csv"; // showld be runtime parameter
    let number_of_wallets = 5; // showld be runtime parameter

    if let Err(e) = generate_wallets_to_csv(csv_path, number_of_wallets) {
        eprintln!("Failed to generate wallets: {}", e);
    } else {
        println!("Wallets successfully generated and written to {}", csv_path);
    }
}
