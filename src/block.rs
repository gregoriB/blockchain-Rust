use chrono::Utc;
use crate::transaction::Transaction;
use crate::common::Hashing;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    hash: String,
    prev_hash: String,
    next_hash: String,
    nonce: u32
}

impl Hashing for Block {
    fn get_hashing_strings(&self) -> Vec<String> {
       vec![
           self.timestamp.to_string(),
           self.nonce.to_string(),
           self.hash.to_owned(),
           self.prev_hash.to_owned(),
           self.next_hash.to_owned(),
       ] 
    }
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            hash: String::new(),
            prev_hash: String::new(),
            next_hash: String::new(),
            nonce: 0,
            transactions
        };
        block.hash = block.generate_hash();
        block
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_prev_hash(&self) -> &str {
        &self.prev_hash
    }

    pub fn get_next_hash(&self) -> &str {
        &self.next_hash
    }

    pub fn set_prev_hash(&mut self, hash: &str) {
        self.prev_hash = hash.to_owned();
    }

    pub fn set_next_hash(&mut self, hash: &str) {
        self.next_hash = hash.to_owned();
    }

    pub fn mine(&mut self, difficulty: usize) {
        let zeros = str::repeat("0", difficulty);

        while &self.hash[..difficulty] != zeros {
            self.hash = self.generate_hash();
            self.nonce += 1;
        }
    }
}