use std::time::{SystemTime, UNIX_EPOCH};
use string_utils;
use std::char;
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Block {
  pub hash: Option<String>,
  pub previous_hash: String,
  data: String,
  time_stamp: u64
}

impl Block {
  pub fn new(data: String, previous_hash: String) -> Block {
    let now_in_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_stamp = now_in_time.as_secs() * 1_000 + now_in_time.subsec_nanos() as u64 / 1_000_000;
    let mut block = 
      Block {
        hash: None,
        previous_hash: previous_hash,
        data: data,
        time_stamp: time_stamp
      };

    block.hash = Some(block.calculate_hash());
    block
  }

  pub fn calculate_hash(&self) -> String {
    let mut nonce = 0;
    let mut rng = thread_rng();
    if rng.gen() {
      nonce = rng.gen::<i32>();
    }
    string_utils::apply_sha_256(&[self.previous_hash.clone(),
                                  self.time_stamp.to_string(),
                                  nonce.to_string(),
                                  self.data.clone()].join(""))
  }

  pub fn generate_target(&self, difficulty: u32) -> char {
    let between = Range::new(0, 255);
    let mut rng = thread_rng();
    char::from_digit(between.ind_sample(&mut rng), difficulty).unwrap_or('0')
  }

  pub fn mine_block(&self, difficulty: u32) {
    let target = self.generate_target(difficulty);
    let mut hash = self.hash.clone().unwrap_or(String::new());
    while !hash.contains(&target.to_string()) {
      hash = self.calculate_hash();
    }
  }
}

#[test]
fn block_basics() {
  let block_genesis = Block::new(String::from("First block"), String::from("0"));
  assert_eq!(block_genesis.hash.unwrap().len(), 64);
}