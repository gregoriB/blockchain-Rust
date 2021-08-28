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

// #[cfg(test)]
// mod tests {
//     use crate::BlockChain;
//     use crate::Transaction;

//     fn create_test_blockchain() -> BlockChain {
//         let name = "test coin";
//         let difficulty = 2;
//         let payout = 1;
//         let supply = 100;
//         let blockchain = BlockChain::new(name, difficulty, payout, supply);
//         blockchain
//     }

//     fn create_test_transaction() -> Transaction {
//         let sender = "test_sender";
//         let receiver = "test_receiver";
//         let amount = 1;
//         let transaction = Transaction::new(sender, receiver, amount);
//         transaction
//     }

//     #[ignore]
//     #[test]
//     fn blockchain_instantiated_with_correct_properties() {
//         let blockchain = BlockChain::new("test coin", 2, 1, 100);
//         assert_eq!(blockchain.name, "test coin");
//         assert_eq!(blockchain.difficulty, 2);
//         assert_eq!(blockchain.payout, 1);
//         assert_eq!(blockchain.supply, 100);
//     }

//     #[ignore]
//     #[test]
//     fn blockchain_creates_proper_genesis_block() {
//         let blockchain = create_test_blockchain();
//         let gen_block = blockchain.chain[0];
//         let gen_block_transaction = gen_block.get_transactions()[0];
//         assert_eq!(gen_block_transaction.get_sender(), "test");
//     }

//     #[ignore]
//     #[test]
//     fn blockchain_adds_new_transaction_to_pending_transactions() {
//         let blockchain = create_test_blockchain();
//         let transaction = create_test_transaction();
//         blockchain.add_transaction(transaction);
//         assert_eq!(blockchain.pending_transactions.len(), 1);
//         assert_eq!(blockchain.pending_transactions[0].get_sender(), transaction.get_sender());
//     }

//     #[ignore]
//     #[test]
//     fn blockchain_adds_new_block() {
//         // TODO: find better testing solutions because this is getting ridiculous already
//         let blockchain = create_test_blockchain();
//         let transaction = create_test_transaction();
//         blockchain.add_transaction(transaction);
//         let payout_address = "test_payout";
//         blockchain.add_block(payout_address);
//         let prev_block = blockchain.chain[0];
//         let new_block = blockchain.chain[1];
//         let prev = new_block.get_prev_hash();
//         let next = new_block.get_next_hash();
//         let hash = new_block.get_hash();
//         assert_eq!(prev_block.get_hash(), prev);
//         assert_eq!(prev_block.get_next_hash(), hash);
//     }
// }
