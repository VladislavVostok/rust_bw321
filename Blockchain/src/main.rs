use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use sha2::digest::Update;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction{
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block{
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

impl Block{
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String ) -> Block{
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = Self::calculate_hash(index, timestamp, &transactions, &previous_hash)

        Block{
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: u64, transactions: &[Transaction], previous_hash: &str) -> String{
        let input = format!("{}{}{:?}{}", index, timestamp, transactions, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();

        format!("{:x}", result)
    }
}

struct Blockchain{
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>
}

impl Blockchain{
    fn new() -> Blockchain{
        let genesis_block = Block::new(0, vec![], "0".to_string());
        Blockchain{
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    fn add_transaction(&mut self, transaction: Transaction){
        self.pending_transactions.push((transaction));
    }

    fn mine_block(&mut self){
        let last_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(last_block.index + 1, self.pending_transactions.clone(), last_block.hash);
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }
}



fn main() {
    println!("Hello, world!");
}
