use string_utils;
use parity_wordlist;

pub struct Wallet {
  private_key: String,
  pub public_key: String
}

impl Wallet {
  pub fn new() -> Wallet {
    Wallet {
      private_key: Wallet::generate_key(),
      public_key: Wallet::generate_key()
    }
  }

  fn generate_key() -> String {
    string_utils::apply_sha_256(&parity_wordlist::random_phrase(12))
  }
}