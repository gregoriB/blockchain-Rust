use demonstrate::demonstrate;

demonstrate! {
    describe "Blockchain" {
        use block_chain::transaction::Transaction;
        use block_chain::blockchain::BlockChain;

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
            let blockchain_data = blockchain.get_data();
            assert_eq!(blockchain_data.name, "test coin");
            assert_eq!(blockchain_data.difficulty, 2);
            assert_eq!(blockchain_data.payout, 1);
            assert_eq!(blockchain_data.supply, 100);
        }

        it "Creates a genesis block when instantiated" {
            let chain = blockchain.get_chain();
            assert!(chain.len() ==  1);
            let gen_block = &chain[0];
            let gen_block_transaction = &gen_block.get_transactions()[0];
            assert_eq!(gen_block_transaction.get_sender(), "Genesis Block");
        }

        it "Adds a new transaction to pending transactions" {
            blockchain.add_pending_transaction(transaction.to_owned());
            let pending = blockchain.get_pending_transactions();
            assert!(pending.len() == 1);
            assert_eq!(pending[0].get_sender(), transaction.get_sender());
        }

        it "Adds a new block to the chain" {
            blockchain.add_pending_transaction(transaction);
            blockchain.add_block(payout_address);
            let chain = blockchain.get_chain();
            let prev_block = &chain[0];
            let new_block = &chain[1];
            assert_eq!(prev_block.get_next_hash(), new_block.get_hash());
            assert_eq!(new_block.get_prev_hash(), prev_block.get_hash());
        }

        it "Pays miner after adding block to chain" {
            blockchain.add_pending_transaction(transaction);
            blockchain.add_block(payout_address);
            let first_pending = blockchain.get_pending_transactions()[0].to_owned();
            assert_eq!(first_pending.get_receiver(), payout_address);
        }

        it "Updates the previous block\'s next hash with the new block hash" {
            blockchain.add_pending_transaction(transaction);
            blockchain.add_block(payout_address);
            let first_block = blockchain.get_chain().first().unwrap().to_owned();
            let last_block = blockchain.get_last_block();
            assert_eq!(first_block.get_next_hash(), last_block.get_hash());
        }
    }
}