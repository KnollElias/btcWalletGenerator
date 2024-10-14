use bitcoin::util::bip32::ExtendedPrivKey;
use secp256k1::SecretKey;
use std::fs;
use std::str::FromStr;

pub fn private_key_to_mnemonic(private_key: &SecretKey) -> Vec<String> {
    let entropy = private_key[..].to_vec();
    let word_list = fs::read_to_string("src/word_list/english.txt").expect("Unable to read file");
    let words: Vec<&str> = word_list.split('\n').collect();

    let mut mnemonic = Vec::new();
    for i in (0..entropy.len()).step_by(2) {
        let index = (entropy[i] as usize) << 8 | (entropy[i + 1] as usize);
        mnemonic.push(words[index % 2048].to_string());
    }
    mnemonic
}

fn main() {
    let private_key_wif = "L3TEcrgashjdghkjdgafslkjasdf";
    let secp = secp256k1::Secp256k1::new();
    let private_key = SecretKey::from_str(private_key_wif).expect("Invalid private key WIF");

    let mnemonic = private_key_to_mnemonic(&private_key);
    println!("Mnemonic: {:?}", mnemonic.join(" "));
}
