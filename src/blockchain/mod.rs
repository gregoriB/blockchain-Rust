use chrono::Utc;
use crate::transaction::Transaction;
use crate::block::Block;

pub struct BlockChain {
    timestamp: i64,
    chain: Vec<Block>,
    name: &'static str,
    difficulty: u32,
    payout: u32,
    supply: u32
}

#[cfg(test)]
mod tests {
    use crate::BlockChain;
    use crate::Transaction;

    fn create_test_blockchain() -> BlockChain {
        let name = "test coin";
        let difficulty = 2;
        let payout = 1;
        let supply = 100;
        let blockchain = Blockchain::new(name, difficulty, payout, supply);
        blockchain
    }

    fn create_test_transaction() -> Transaction {
        let sender = "test_sender";
        let receiver = "test_receiver";
        let amount = 1;
        let transaction = Transaction::new(sender, receiver, amount);
        transaction
    }

    #[test]
    fn blockchain_instantiated_with_correct_properties() {
        // timestamp name difficulty payout supply chain pending observers
        let blockchain = create_test_blockchain();
        assert_eq!(blockchain.name, name);
        assert_eq!(blockchain.difficulty, name);
        assert_eq!(blockchain.payout, name);
        assert_eq!(blockchain.supply, name);
    }

    #[test]
    fn blockchain_creates_proper_genesis_block() {
        let blockchain = create_test_blockchain();
        let gen_block = blockchain.chain[0];
        let gen_block_transaction = gen_block.get_transactions[0];
        assert_eq!(gen_block_transaction.sender, "test");
    }

    #[test]
    fn blockchain_adds_new_transaction_to_pending() {
        let blockchain = create_test_blockchain();
        let transaction = create_test_transaction();
        blockchain.add_transaction(transaction);
        assert_eq!(blockchain.pending.len(), 1);
        assert_eq!(blockchain.pending[0], transaction);
    }

    #[test]
    fn blockchain_adds_new_block() {
        // TODO: find better testing solutions because this is getting ridiculous already
        let blockchain = create_test_blockchain();
        let transaction = create_test_transaction();
        blockchain.add_transaction(transaction);
        let payout_address = "test_payout";
        blockchain.add_block(payout_address);
        let prev_block = blockchain[0];
        let new_block = blockchain[1];
        let prev = &new_block.prev_hash;
        let next = &new_block.next_hash;
        let hash = &new_block.hash;
        assert_eq!(prev_block.hash, prev);
        assert_eq!(next.len(), 0);
        assert_eq!(hash.len(), 64);
        assert_eq!(prev_block.next, hash);
    }
}
