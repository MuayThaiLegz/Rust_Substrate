use ed25519_dalek::Keypair;
use rustblockchainlib::*;

fn main() {
    let mut blockchain = Blockchain::new();

    let roy_key = Wallet::new();
    let eddie_key = Wallet::new();
    let max_key = Wallet::new();

    let miner_key = Wallet::new();


    let mut first_transaction = Transaction {
        sender: Some(roy_key.public),
        receiver: Some(eddie_key.public),
        amount: 2000.0,
        signature: None,    
    };

    first_transaction.sign_transaction(Keypair {
        public: roy_key.public,
        secret: roy_key.secret,
    });

    blockchain.add_new_transaction(first_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    let sam_key = Wallet::new();
    let michal_key = Wallet::new();


    let mut second_transaction = Transaction {
        sender: Some(sam_key.public),
        receiver: Some(michal_key.public),
        amount: 2500.0,
        signature: None,
    };

    second_transaction.sign_transaction(Keypair {
        public: sam_key.public,
        secret: sam_key.secret,
    });

    blockchain.add_new_transaction(second_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    let mut third_transaction = Transaction {
        sender:  Some(michal_key.public),
        receiver: Some(max_key.public),
        amount: 1000.0,
        signature: None,
    };

    third_transaction.sign_transaction(Keypair {
        public: michal_key.public,
        secret: michal_key.secret,
    });

    blockchain.add_new_transaction(third_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);   
}