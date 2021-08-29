use demonstrate::demonstrate;

demonstrate! {
    describe "Block" {
        use block_chain::block::Block;
        use block_chain::transaction::Transaction;

        before {
            let transactions = vec![Transaction::new("1234", "5678", 100)];
            let first_transaction = transactions.first().unwrap().clone();
            let mut block = Block::new(transactions);
        }

        it "Instantiates block with the correct properties" {
            let block_transaction = block.get_transactions().first().unwrap().clone();
            assert_eq!(block_transaction.get_sender(), first_transaction.get_sender());
            assert_eq!(block_transaction.get_receiver(), first_transaction.get_receiver());
            assert_eq!(block_transaction.get_amount(), first_transaction.get_amount());
            assert_eq!(block.get_hash().len(), 64);
        }

        it "Mines block" {
            let difficulty = 3;
            block.mine(difficulty);
            let hash = block.get_hash();
            assert_eq!(&hash[..difficulty], str::repeat("0", difficulty));
            assert_eq!(hash.len(), 64);
        }

        it "Updates prev block hash" {
            block.set_prev_hash("prev hash");
            assert_eq!(block.get_prev_hash(), "prev hash".to_string());
        }

        it "Updates next block hash" {
            block.set_next_hash("next hash");
            assert_eq!(block.get_next_hash(), "next hash".to_string());
        }
    }
}