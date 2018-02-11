use std::time::{SystemTime, UNIX_EPOCH};
use string_utils;

pub struct Block {
  hash: String,
  previous_hash: &'static str,
  data: &'static str,
  time_stamp: u64
}

impl Block {
  pub fn new(data: &'static str, previous_hash: &'static str) -> Block {
    let now_in_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time_stamp = now_in_time.as_secs() * 1_000 + now_in_time.subsec_nanos() as u64 / 1_000_000;
    Block {
      hash: Block::calculate_hash(previous_hash, time_stamp, data),
      previous_hash: previous_hash,
      data: data,
      time_stamp: time_stamp
    }
  }

  fn calculate_hash(previous_hash: &str, time_stamp: u64, data: &str) -> String {
    string_utils::apply_sha_256(&[previous_hash,
                                  time_stamp.to_string().as_str(),
                                  data].join(""))
  }
}

#[test]
fn block_basics() {
  let block_genesis = Block::new("First block", "0");
  assert_eq!(block_genesis.hash, String::from("123456"));
}