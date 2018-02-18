extern crate ring;
extern crate rand;
extern crate parity_wordlist;

mod block;
mod string_utils;
mod chain;
mod wallet;
mod transaction;

fn main() {
    let mut chain = chain::Chain::new();
	chain.add_block(String::from("Second Block"));
	chain.mining();
	println!("{}", chain.is_valid());
}
