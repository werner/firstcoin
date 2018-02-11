use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
  hash: Option<&'static str>,
  previous_hash: &'static str,
  data: &'static str,
  time_stamp: u64
}

impl Block {
  pub fn new(data: &'static str, previous_hash: &'static str) -> Block {
    let now_in_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    Block {
      hash: None,
      previous_hash: previous_hash,
      data: data,
      time_stamp: now_in_time.as_secs() * 1_000 + now_in_time.subsec_nanos() as u64 / 1_000_000
    }
  }
}