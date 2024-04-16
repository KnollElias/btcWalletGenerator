use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use rand::Rng;
use secp256k1::{Secp256k1, SecretKey};
use std::str::FromStr;

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

fn main() {
    let private_key = generate_private_key();
    let xpub = generate_xpub(&private_key);

    println!("Private Key: {:?}", private_key);
    println!("xPub: {}", xpub);
}
