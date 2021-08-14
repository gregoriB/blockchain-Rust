use crypto_hash::{Algorithm, hex_digest};
use chrono::Utc;

#[derive(Debug, Clone)]
struct Transaction {
    timestamp: i64,
    sender: &'static str,
    receiver: &'static str,
    amount: u32,
    hash: String
}

trait Hashing {
    fn get_hashing_strings(&self) -> Vec<String>;

    fn generate_hash(&mut self) -> String {
        let strings = self.get_hashing_strings();
        let mut hash_data = String::new();
        for string  in &strings {
            hash_data.push_str(&string);
        }
        hex_digest(Algorithm::SHA256, hash_data.as_bytes())
    }
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
    fn new(sender: &'static str, receiver: &'static str, amount: u32) -> Transaction {
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

    fn get_hash(&self) -> &str {
        &self.hash
    }

    fn get_sender(&self) -> &'static str {
        self.sender
    }

    fn get_receiver(&self) -> &'static str {
        self.receiver
    }

    fn get_amount(&self) -> u32 {
        self.amount
    }
}

struct Block {
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
    fn new(transactions: Vec<Transaction>) -> Block {
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

    fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    fn get_hash(&self) -> &str {
        &self.hash
    }

    fn mine(&mut self, difficulty: usize) {
        let zeros = str::repeat("0", difficulty);

        while &self.hash[..difficulty] != zeros {
            self.hash = self.generate_hash();
            self.nonce += 1;
        }
    }
}

fn main() {
    let transactions = vec![Transaction::new("1234", "5678", 100)];
    let mut block = Block::new(transactions);
    println!("Transactions: {:?}", block.get_transactions());
    block.mine(4);
    println!("HASH: {:?}", block.get_hash());
}

#[cfg(test)]
mod tests {
    use crate::Block;
    use crate::Transaction;
        
    #[test]
    fn transaction_instantiated_with_correct_properties() {
        let sender = "1234";
        let receiver = "5678";
        let amount = 100;
        let transaction = Transaction::new(sender, receiver, amount);
        assert_eq!(transaction.get_sender(), sender);
        assert_eq!(transaction.get_receiver(), receiver);
        assert_eq!(transaction.get_amount(), amount);
        assert_eq!(transaction.get_hash().len(), 64);
    }

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