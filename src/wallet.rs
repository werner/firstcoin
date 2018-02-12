use string_utils;
use ring::{digest, signature, rand, agreement};

pub struct Wallet {
  private_key: Option<agreement::EphemeralPrivateKey>,
  pub public_key: Option<String>
}

impl Wallet {
  pub fn new() -> Wallet {
    Wallet {
      private_key: None,
      public_key: None
    }
  }

  pub fn generate_keys(&mut self) -> &Wallet {
      let rng = rand::SystemRandom::new();
      let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
      let mut computed_public = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
      if private_key.compute_public_key(&mut computed_public[..private_key.public_key_len()]).is_ok() {
        self.private_key = Some(private_key);
        self.public_key = Some(string_utils::from_bytes_to_string(&computed_public));
      }

      self
  }
}