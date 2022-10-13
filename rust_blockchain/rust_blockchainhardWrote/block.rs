use super:: *;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {

    pub fn new(pre_hash: String, transaction: Vec<Transaction>) -> Self {
        let time = new();
        //let empty_string = String::new();
        //let nonce - 0u64;
        Block {
            timestamp: ttime,
            hash: calculate_hash(&pre_hash, &transaction, time ),
            pre_hash,
            transaction,
            //nonce,
        }
    }
}