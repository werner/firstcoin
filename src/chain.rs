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
    let previous_hash = previous_block.hash;
    self.chain.push(Block::new(data, String::from(previous_hash.clone())));
    self
  }
}