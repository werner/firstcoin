extern crate sha2;
extern crate rand;
extern crate parity_wordlist;

mod block;
mod string_utils;
mod chain;
mod wallet;
mod transaction;

fn main() {
    println!("Words: {}", parity_wordlist::random_phrase(12));
}
