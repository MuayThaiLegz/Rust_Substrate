use rustblockchainlib::*;

fn main() {
    let first_block = Block::new(
        "0".to_owner(),
        vec!{Transaction{
            sender: String::from("Ryan"),
            amount: 2000.0,
        }},
    );

    println!("{:#?", first_block);

    
    let second_block = Block::new(
        "0".to_owner(),
        vec!{Transaction{
            sender: String::from("Pie"),
            amount: 2000.0,
        }},
    );

    println!("{:#?", second_block);
}