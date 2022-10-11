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

    pub fn new(transaction: Vec<Transaction>) -> Self {
        let time = new();
        let empty_string = String::new();
        let nonce - 0u64;
        Block {
            timestamp: ttime,
            hash: empty_string.clone(),
            pre_hash: empty_string.clone(),
            transaction,
            nonce,
        }
    }

    pub fn set_hash(&mut self) {
        self.hash = calculate_hash(
            &self.pre_hash
            &self.transaction

            &self.timestamp
            &self.nonce
        )
    }
    ///
    /// Start mining process to solve puzzle 
    /// Guess the nonce until miner finds a hash that 
    /// matches the set difficulty level
    pub fn mine(&mut self) {
        let targer = get_difficult_string();

        while &self.hash[..DIFFICULT_LEVEL as usize] != targer {
            self.nonce += 1;
            self.hash calculate_hash(
                &self.pre_hash
                &self.transaction
            
                &self.timestamp
                &self.nonce

            )
        }

        println!("Block Mined");
    }

    pub fn has_valid_transactions($self) -> bool {
        for tran in &self.transaction {
            if !tran.is_valid_transaction() {
                return false;
            }
        }

        return true;
    }
}