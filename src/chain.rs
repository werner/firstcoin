use block::{Block};

pub struct Chain {
  chain: Vec<Block>,
  difficulty: u32
}

impl Chain {

  pub fn new() -> Chain {
    Chain {
      chain: vec![Block::new(String::from("Genesis block"), String::from("0"))],
      difficulty: 5
    }
  }

  pub fn add_block(&mut self, data : String) -> &Self {
    let previous_block = self.chain[self.chain.len() - 1].clone();
    let previous_hash = previous_block.hash.unwrap_or(String::new());
    self.chain.push(Block::new(data, String::from(previous_hash.clone())));
    self
  }

  pub fn is_valid(&self) -> bool {
    for (i, block) in self.chain.clone().into_iter().enumerate() {
	  if i == 0 { continue };
      let current_block = block;
      let previous_block = self.chain[i - 1].clone();
      let target = current_block.generate_target(self.difficulty);

      if previous_block.hash.unwrap_or(String::new()) != current_block.previous_hash {
        println!("Hashes not equal");
        return false
      }

      if current_block.hash.unwrap_or(String::new()).contains(&target.to_string()) {
        println!("Not mined");
        return false
      }
    }
    true
  }

  pub fn mining(&self) {
  	  for block in self.chain.clone() {
	  	  block.mine_block(self.difficulty);
	  }
  }
}