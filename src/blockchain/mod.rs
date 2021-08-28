use chrono::Utc;
use crate::transaction::Transaction;
use crate::block::Block;

pub struct BlockChain {
    timestamp: i64,
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    name: &'static str,
    difficulty: u32,
    payout: u32,
    supply: u32
}


use demonstrate::demonstrate;

demonstrate! {
    describe "Blockchain" {
        use crate::BlockChain;
        use crate::Transaction;

        before {
            let name = "test coin";
            let difficulty = 2;
            let payout = 1;
            let supply = 100;
            let blockchain = BlockChain::new(name, difficulty, payout, supply);

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
            let gen_block = blockchain.chain[0];
            let gen_block_transaction = gen_block.get_transactions()[0];
            assert_eq!(gen_block_transaction.get_sender(), "test");
        }

        it "Adds a new transaction to pending transactions" {
            blockchain.add_transaction(transaction);
            assert_eq!(blockchain.pending_transactions.len(), 1);
            assert_eq!(blockchain.pending_transactions[0].get_sender(), transaction.get_sender());
        }

        it "Adds a new block to the chain" {
            blockchain.add_transaction(transaction);
            let payout_address = "test_payout";
            blockchain.add_block(payout_address);
            let prev_block = blockchain.chain[0];
            let new_block = blockchain.chain[1];
            let prev = new_block.get_prev_hash();
            let next = new_block.get_next_hash();
            let hash = new_block.get_hash();
            assert_eq!(prev_block.get_hash(), prev);
            assert_eq!(prev_block.get_next_hash(), hash);
        }
    }
}