use crypto_hash::{Algorithm, hex_digest};

#[derive(Debug, Copy, Clone)]
struct Transaction {
    sender: &'static str,
    receiver: &'static str,
    amount: u32
}

impl Transaction {
    fn new(sender: &'static str, receiver: &'static str, amount: u32) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount
        }
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

impl Block {
    fn new(transactions: Vec<Transaction>) -> Block {
        let mut block = Block {
            timestamp: 0,
            hash: String::new(),
            prev: String::new(),
            next: String::new(),
            nonce: 0,
            transactions
        };
        block.generate_hash();
        block
    }

    fn generate_hash(&mut self) {
        let mut hash_data = String::new();
        hash_data.push_str(&self.timestamp.to_string());
        hash_data.push_str(&self.nonce.to_string());
        hash_data.push_str(&self.prev);
        hash_data.push_str(stringify!(self.transactions));
        self.hash = hex_digest(Algorithm::SHA256, hash_data.as_bytes());
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
            self.generate_hash();
            self.nonce += 1;
        }

        println!("Block successfully mined");
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