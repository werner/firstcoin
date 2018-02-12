use block::{Block};

pub struct Chain {
  chain: Vec<Block>
}

impl Chain {

  pub fn new() -> Chain {
    Chain {
      chain: vec![Block::new(String::from("Genesis block"), String::from("0"))]
    }
  }

  pub fn add_block(&mut self, data : String) -> &Self {
    let previous_block = self.chain[self.chain.len() - 1].clone();
    let previous_hash = previous_block.hash.unwrap_or(String::new());
    self.chain.push(Block::new(data, String::from(previous_hash.clone())));
    self
  }

  pub fn is_valid(&self) -> bool {
    for block in self.chain.clone() {
      let current_block = block;
      let previous_block = self.chain[self.chain.len() - 1].clone();

      if previous_block.hash.unwrap_or(String::new()) != current_block.previous_hash {
        return false
      }
    }
    true
  }
}