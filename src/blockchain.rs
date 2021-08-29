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

#[derive(Debug)]
pub struct BlockChainData {
    pub timestamp: i64,
    pub name: &'static str,
    pub difficulty: usize,
    pub payout: u32,
    pub supply: u32
}

impl BlockChain {
    pub fn new(name: &'static str, difficulty: usize, payout: u32, supply: u32) -> BlockChain {
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

    pub fn get_data(&self) -> BlockChainData {
        BlockChainData {
            timestamp: self.timestamp,
            name: self.name,
            supply: self.supply,
            difficulty: self.difficulty,
            payout: self.payout
        }
    }

    pub fn get_chain(&self) -> Vec<Block> {
        self.chain.clone()
    }

    pub fn get_pending_transactions(&self) -> Vec<Transaction> {
        self.pending_transactions.clone()
    }

    pub fn get_last_block(&self) -> Block {
        self.chain.last().unwrap().to_owned()
    }

    fn generate_genesis_block(&mut self) {
        let transaction = Transaction::new("Genesis Block", "Genesis Block", 0);
        let block = Block::new(vec![transaction]);
        self.chain.push(block);
    }

    fn create_block(&self) -> Block {
        Block::new(self.pending_transactions.clone())
    }

    pub fn add_block(&mut self, payout_address: &'static str) {
        let mut last_block = self.get_last_block();
        let mut new_block = self.create_block();
        new_block.mine(self.difficulty);
        last_block.set_next_hash(new_block.get_hash());
        new_block.set_prev_hash(last_block.get_hash());
        self.update_chain(last_block, new_block);
        self.reset_pending_transactions();
        self.pay_miner(payout_address);
    }

    fn update_chain(&mut self, last_block: Block, new_block: Block) {
        let mut chain_slice = self.chain[0..self.chain.len() - 1].to_owned();
        chain_slice.push(last_block);
        chain_slice.push(new_block);
        self.chain = chain_slice;
    }

    fn pay_miner(&mut self, payout_address: &'static str) {
        let payout_transaction = self.create_payout_transaction(payout_address);
        self.add_pending_transaction(payout_transaction);
    }

    fn create_payout_transaction(&self, payout_address: &'static str) -> Transaction {
        Transaction::new(self.name, payout_address, self.payout)
    }

    fn reset_pending_transactions(&mut self) {
        self.pending_transactions = vec![];
    }

    pub fn add_pending_transaction(&mut self, transaction: Transaction) {
        let pending = &self.pending_transactions;
        let mut pending_slice = pending[..].to_owned();
        pending_slice.push(transaction);
        self.pending_transactions = pending_slice;
    }
}