use std::time::{SystemTime, UNIX_EPOCH};
use string_utils;

#[derive(Clone)]
pub struct Block {
  pub hash: String,
  pub previous_hash: String,
  data: String,
  time_stamp: u64
}

impl Block {
  pub fn new(data: String, previous_hash: String) -> Block {
    let now_in_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_stamp = now_in_time.as_secs() * 1_000 + now_in_time.subsec_nanos() as u64 / 1_000_000;
    Block {
      hash: Block::calculate_hash(&previous_hash, time_stamp, &data),
      previous_hash: previous_hash,
      data: data,
      time_stamp: time_stamp
    }
  }

  pub fn calculate_hash(previous_hash: &str, time_stamp: u64, data: &str) -> String {
    string_utils::apply_sha_256(&[previous_hash,
                                  time_stamp.to_string().as_str(),
                                  data].join(""))
  }
}

#[test]
fn block_basics() {
  let block_genesis = Block::new(String::from("First block"), String::from("0"));
  assert_eq!(block_genesis.hash.len(), 64);
}