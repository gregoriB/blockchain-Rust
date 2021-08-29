use chrono::Utc;
use crate::common::Hashing;

#[derive(Debug, Clone)]
pub struct Transaction {
    timestamp: i64,
    sender: &'static str,
    receiver: &'static str,
    amount: u32,
    hash: String
}

impl Hashing for Transaction {
    fn get_hashing_strings(&self) -> Vec<String> {
       vec![
           self.timestamp.to_string(),
           self.sender.to_string(),
           self.receiver.to_string(),
           self.amount.to_string()
       ] 
    }
}

impl Transaction {
    pub fn new(sender: &'static str, receiver: &'static str, amount: u32) -> Transaction {
        let mut transaction = Transaction {
            timestamp: Utc::now().timestamp(),
            sender,
            receiver,
            amount,
            hash: String::new()
        };
        transaction.hash = transaction.generate_hash();
        transaction
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_sender(&self) -> &'static str {
        self.sender
    }

    pub fn get_receiver(&self) -> &'static str {
        self.receiver
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }
}