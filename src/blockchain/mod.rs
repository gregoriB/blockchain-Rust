use chrono::Utc;
use crate::transaction::Transaction;
use crate::block::Block;

pub struct BlockChain {
    timestamp: i64,
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    name: &'static str,
    difficulty: usize,
    payout: u32,
    supply: u32
}

impl BlockChain {
    fn new(name: &'static str, difficulty: usize, payout: u32, supply: u32) -> BlockChain {
        let mut blockchain = BlockChain {
            timestamp: Utc::now().timestamp(),
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            name,
            difficulty,
            payout,
            supply,
        };
        blockchain.generate_genesis_block();
        blockchain
    } 

    fn generate_genesis_block(&mut self) {
        let transaction = Transaction::new("Genesis Block", "Genesis Block", 0);
        let block = Block::new(vec![transaction]);
        self.chain.push(block);
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn add_block(&mut self, payout_address: &'static str) {
        let mut last_block = self.get_last_block();
        let mut block = Block::new(self.pending_transactions.clone());
        block.mine(self.difficulty);
        last_block.set_next_hash(block.get_hash());
        block.set_prev_hash(last_block.get_hash());
        let mut chain_slice = self.chain[0..self.chain.len() - 1].to_owned();
        chain_slice.push(last_block);
        chain_slice.push(block);
        self.chain = chain_slice;
        let transaction = Transaction::new(self.name, payout_address, self.payout);
        self.pending_transactions = vec![];
        self.add_transaction(transaction);
    }

    fn get_last_block(&self) -> Block {
        self.chain.last().unwrap().to_owned()
    }
}


use demonstrate::demonstrate;

demonstrate! {
    describe "Blockchain" {
        use crate::BlockChain;
        use crate::Transaction;

        before {
            let payout_address = "test_payout";

            let name = "test coin";
            let difficulty = 2;
            let payout = 1;
            let supply = 100;
            let mut blockchain = BlockChain::new(name, difficulty, payout, supply);

            let sender = "test_sender";
            let receiver = "test_receiver";
            let amount = 1;
            let transaction = Transaction::new(sender, receiver, amount);
        }

        it "Is instantiated with the correct properties" {
            assert_eq!(blockchain.name, "test coin");
            assert_eq!(blockchain.difficulty, 2);
            assert_eq!(blockchain.payout, 1);
            assert_eq!(blockchain.supply, 100);
        }

        it "Creates a genesis block when instantiated" {
            assert_eq!(blockchain.chain.len(),  1);
            let gen_block = &blockchain.chain[0];
            let gen_block_transaction = &gen_block.get_transactions()[0];
            assert_eq!(gen_block_transaction.get_sender(), "Genesis Block");
        }

        it "Adds a new transaction to pending transactions" {
            blockchain.add_transaction(transaction.to_owned());
            let pending = &blockchain.pending_transactions;
            assert!(pending.len() == 1);
            assert_eq!(pending[0].get_sender(), transaction.get_sender());
        }

        it "Adds a new block to the chain" {
            blockchain.add_transaction(transaction);
            blockchain.add_block(payout_address);
            let prev_block = &blockchain.chain[0];
            let new_block = &blockchain.chain[1];
            assert_eq!(prev_block.get_next_hash(), new_block.get_hash());
            assert_eq!(new_block.get_prev_hash(), prev_block.get_hash());
        }

        it "Pays miner after adding block to chain" {
            blockchain.add_transaction(transaction);
            blockchain.add_block(payout_address);
            let first_pending = blockchain.pending_transactions[0].to_owned();
            assert_eq!(first_pending.get_receiver(), payout_address);
        }

        it "Updates the previous block\'s next hash with the new block hash" {
            blockchain.add_transaction(transaction);
            blockchain.add_block(payout_address);
            let first_block = blockchain.chain.first().unwrap().to_owned();
            let last_block = blockchain.get_last_block();
            assert_eq!(first_block.get_next_hash(), last_block.get_hash());
        }
    }
}