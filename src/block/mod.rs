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

    pub fn set_prev_hash(&mut self, hash: &str) {
        self.prev_hash = hash.to_owned();
    }

    pub fn get_next_hash(&self) -> &str {
        &self.next_hash
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

use demonstrate::demonstrate;

demonstrate! {
    describe "Block" {
        use crate::Transaction;
        use crate::Block;
        before {
            let transactions = vec![Transaction::new("1234", "5678", 100)];
            let first_transaction = transactions.first().unwrap().clone();
            let mut block = Block::new(transactions);
        }

        it "Instantiates block with the correct properties" {
            let block_transaction = block.get_transactions().first().unwrap().clone();
            assert_eq!(block_transaction.get_sender(), first_transaction.get_sender());
            assert_eq!(block_transaction.get_receiver(), first_transaction.get_receiver());
            assert_eq!(block_transaction.get_amount(), first_transaction.get_amount());
            assert_eq!(block.get_hash().len(), 64);
            assert_eq!(block.nonce, 0);
        }

        it "Mines block" {
            let difficulty = 3;
            block.mine(difficulty);
            let hash = block.get_hash();
            assert_eq!(&hash[..difficulty], str::repeat("0", difficulty));
            assert_eq!(hash.len(), 64);
        }

        it "Updates prev block hash" {
            block.set_prev_hash("prev hash");
            assert_eq!(block.get_prev_hash(), "prev hash".to_string());
        }

        it "Updates next block hash" {
            block.set_next_hash("next hash");
            assert_eq!(block.get_next_hash(), "next hash".to_string());
        }
    }
}