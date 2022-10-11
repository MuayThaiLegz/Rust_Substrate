use rustblockchainlib::*;

fn main() {
    let mut blockchain = Blockchain::new();
    
    let genesis_block = Block::new(vec![Transaction{
        sender:String::from("CVS"),
        receiver:String::from("UserA"),
        amount: 2000.00,
    }]);

    let first_block = Block::new(vec![Transaction{
            sender: String::from("Jav"),
            receiver: String::from("kenta"),
            amount: 2000.0,
        }],
    );

    let second_block = Block::new(vec![Transaction{
            sender: String::from("One"),
            receiver: String::from("Two"),
            amount: 1000.0,
    }]);

    blockchain.add_block(genesis_block);
    blockchain.add_block(first_block);
    blockchain.add_block(second_block);

    println!("{:#?}", blockchain);
}