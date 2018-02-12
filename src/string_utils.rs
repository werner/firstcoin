use parity_wordlist;
use ring::{digest};

pub fn apply_sha_256(input: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    let hash = digest::digest(&digest::SHA256, input.as_bytes());
    for byte in hash.as_ref() {
        let byte_to_vec = format!("{:02x}", byte);
        result.push(byte_to_vec);
    }
    result.join("")
}

pub fn generate_hash() -> String {
    apply_sha_256(&parity_wordlist::random_phrase(12))
}

pub fn from_bytes_to_string(input: &[u8]) -> String {
    let mut result: Vec<String> = Vec::new();
    for byte in input.as_ref() {
        let byte_to_vec = format!("{:02x}", byte);
        result.push(byte_to_vec);
    }
    result.join("")
}