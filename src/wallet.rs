use string_utils;

pub struct Wallet {
  private_key: String,
  pub public_key: String
}

impl Wallet {
  pub fn new() -> Wallet {
    Wallet {
      private_key: string_utils::generate_hash(),
      public_key: string_utils::generate_hash()
    }
  }
}