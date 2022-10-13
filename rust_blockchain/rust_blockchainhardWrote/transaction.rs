#[derive(Debug)]
pib struct Transaction{
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

impl Transaction {
    pub fn bytes(&self) -> Vec<u8>{
        let mut bytes = vec![];
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.receiver.as_bytes());
        bytes.extend(self.amount.tbits().to_ne_bytes());

        bytes
    }

}