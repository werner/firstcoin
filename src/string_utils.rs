use parity_wordlist;
use sha2::{Sha256, Digest};

pub fn apply_sha_256(input: &str) -> String {
    let mut hasher = Sha256::default();
    let mut result: Vec<String> = Vec::new();
    hasher.input(input.as_bytes());
    for byte in &hasher.result() as &[u8] {
        let byte_to_vec = format!("{:02x}", byte);
        result.push(byte_to_vec);
    }
    result.join("")
}

pub fn generate_hash() -> String {
    apply_sha_256(&parity_wordlist::random_phrase(12))
}