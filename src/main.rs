mod block;
mod transaction;
mod common;

use crate::block::Block;
use crate::transaction::Transaction;

fn main() {
    let transactions = vec![Transaction::new("1234", "5678", 100)];
    let mut block = Block::new(transactions);
    println!("Transactions: {:?}", block.get_transactions());
    block.mine(4);
    println!("HASH: {:?}", block.get_hash());
}