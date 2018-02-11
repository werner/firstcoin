use std::time::{SystemTime, UNIX_EPOCH};
use string_utils;
use std::char;
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng};

#[derive(Clone)]
pub struct Block {
  pub hash: String,
  pub previous_hash: String,
  data: String,
  time_stamp: u64,
  nonce: u32
}

impl Block {
  pub fn new(data: String, previous_hash: String) -> Block {
    let now_in_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_stamp = now_in_time.as_secs() * 1_000 + now_in_time.subsec_nanos() as u64 / 1_000_000;
    Block {
      hash: Block::calculate_hash(&previous_hash, time_stamp, &data, &1.to_string()),
      previous_hash: previous_hash,
      data: data,
      nonce: 1,
      time_stamp: time_stamp
    }
  }

  pub fn calculate_hash(previous_hash: &str, time_stamp: u64, data: &str, nonce: &str) -> String {
    string_utils::apply_sha_256(&[previous_hash,
                                  time_stamp.to_string().as_str(),
                                  nonce,
                                  data].join(""))
  }

  pub fn mine_block(&self, difficulty: u32) {
    let between = Range::new(0, 255);
    let mut rng = thread_rng();
    let target = char::from_digit(between.ind_sample(&mut rng), difficulty).unwrap_or('0');
    let mut nonce = 0;
    let mut hash = Block::calculate_hash(&self.previous_hash, self.time_stamp, &self.data, &nonce.to_string());
    while !hash.contains(&target.to_string()) {
      nonce += 1;
      hash = Block::calculate_hash(&self.previous_hash, self.time_stamp, &self.data, &nonce.to_string());
    }
  }
}

#[test]
fn block_basics() {
  let block_genesis = Block::new(String::from("First block"), String::from("0"));
  assert_eq!(block_genesis.hash.len(), 64);
}