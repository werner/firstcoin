use string_utils;

type PublicKey = String;
pub struct TransactionInput {}
pub struct TransactionOutput {}

pub struct Transaction {
  id: Option<String>,
  sender: PublicKey,
  recipient: PublicKey,
  value: f64,
  signature: Option<Vec<u8>>,
  inputs: Vec<TransactionInput>,
  outputs: Option<Vec<TransactionOutput>>,
  sequence: Option<i32>
}

impl Transaction {
  pub fn new(from: PublicKey, to: PublicKey, value: f64, inputs: Vec<TransactionInput>, sequence: i32) -> Transaction {
    let mut transaction = 
      Transaction {
        id: None,
        sender: from.clone(),
        recipient: to.clone(),
        value: value,
        inputs: inputs,
        outputs: None,
        sequence: None,
        signature: None
      };
    
    transaction.id = Some(string_utils::apply_sha_256(&[from,
                                                        to,
                                                        value.to_string(),
                                                        sequence.to_string()].join("")));
    transaction.sequence = Some(sequence + 1);
    transaction
  }
}