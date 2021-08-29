use demonstrate::demonstrate;

demonstrate! {
    describe "Transaction" {
        use block_chain::transaction::Transaction;

        before {
            let sender = "1234";
            let receiver = "5678";
            let amount = 100;
            let transaction = Transaction::new(sender, receiver, amount);
        }

        it "Instantiates transaction with the correct properties" {
            assert_eq!(transaction.get_sender(), sender);
            assert_eq!(transaction.get_receiver(), receiver);
            assert_eq!(transaction.get_amount(), amount);
            assert_eq!(transaction.get_hash().len(), 64);
        }
    }
}