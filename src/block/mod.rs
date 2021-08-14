use chrono::Utc;
use crate::transaction::Transaction;
use crate::common::Hashing;

pub struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    hash: String,
    prev: String,
    next: String,
    nonce: u32
}

impl Hashing for Block {
    fn get_hashing_strings(&self) -> Vec<String> {
       vec![
           self.timestamp.to_string(),
           self.nonce.to_string(),
           self.hash.to_owned(),
           self.prev.to_owned(),
           self.next.to_owned(),
       ] 
    }
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            hash: String::new(),
            prev: String::new(),
            next: String::new(),
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

    pub fn mine(&mut self, difficulty: usize) {
        let zeros = str::repeat("0", difficulty);

        while &self.hash[..difficulty] != zeros {
            self.hash = self.generate_hash();
            self.nonce += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Block;
    use crate::Transaction;
        
    #[test]
    fn block_instantiated_with_correct_properties() {
        let transactions = vec![Transaction::new("1234", "5678", 100)];
        let first_transaction = transactions.first().unwrap().clone();
        let block = Block::new(transactions);
        let block_transaction = block.get_transactions().first().unwrap().clone();
        assert_eq!(block_transaction.get_sender(), first_transaction.get_sender());
        assert_eq!(block_transaction.get_receiver(), first_transaction.get_receiver());
        assert_eq!(block_transaction.get_amount(), first_transaction.get_amount());
        assert_eq!(block.get_hash().len(), 64);
        assert_eq!(block.nonce, 0);
    }

    #[test]
    fn block_is_mined() {
        let transactions = vec![Transaction::new("1234", "5678", 100)];
        let mut block = Block::new(transactions);
        let difficulty = 3;
        block.mine(difficulty);
        let hash = block.get_hash();
        assert_eq!(&hash[..difficulty], str::repeat("0", difficulty));
        assert_eq!(hash.len(), 64);
    }
}