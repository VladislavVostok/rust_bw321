use std::cmp::min;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use serde_json::Value::String;
use sha2::{Sha256, Digest};
use sha2::digest::Update;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction{
    sender: String,
    receiver: String,
    amount: f64,
    timestamp: u64,
    hash: String,
}

impl Transaction{
    fn new(sender: String, receiver: String, amount: f64 ) -> Transaction {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = Self::calculate_hash_tx(&sender, &receiver, amount, timestamp);

        Transaction{
            sender,
            receiver,
            amount,
            timestamp,
            hash,
        }
    }

    fn calculate_hash_tx(sender: &str, receiver: &str, amount: f64, timestamp: u64) -> String {

        let input = format!("{}{}{}{}", receiver, timestamp, amount, timestamp);
        let mut hasher = Sha256::new();
        Update::update(&mut hasher, input.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    fn print_transaction(&self){
        print!("{} -> {} ({}) {} {}\n", self.sender, self.receiver, self.amount, self.hash, self.timestamp)
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block{
    previous_hash: String,
    transactions: Vec<Transaction>,
    hash: String,
    timestamp: u64,
    nonce: u32,
    merkele_root: String,
    index: u64
}
impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String ) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let merkele_root = Self::calculate_merkele_root(&transactions);

        let hash = Self::calculate_hash(index, timestamp, &transactions, &previous_hash);

        let nonce = 0;

        Block{
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            merkele_root,
            nonce
        }
    }

    fn calculate_hash(index: u64, timestamp: u64, transactions: &[Transaction], previous_hash: &str) -> String{
        let input = format!("{}{}{:?}{}", index, timestamp, transactions, previous_hash);
        let mut hasher = Sha256::new();
        Update::update(&mut hasher, input.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    fn calculate_merkele_root(transactions: &Vec<Transaction>) -> String{

        let mut hasher = Sha256::new();

        if(transactions.is_empty()){
            Update::update(&mut hasher, "".as_bytes());
            return format!("{:x}", $hasher.finalize());
        }

        let mut tree : Vec<String> = Vec::new();

        for tx in transactions {
            Update::update(&mut hasher, tx.hash.as_bytes());
            tree.push(format!("{:x}", $hasher.finalize()));
        }

        let mut  level_offset : u32 = 0;
        let mut level_size = transactions.len();
        while(level_size > 1){
            let mut left:u32 = 0;

            while(left < level_size){
                let mut right = min(left + 1, level_size - 1);
                Update::update(&mut hasher, format!("{}{}",tree[level_offset + left].hash, tree[level_offset + right].hash));
                tree.push(format!("{:x}", $hasher.finalize()));

                left += 2;
            }
            level_offset += level_size;


            level_size = (level_size + 1) / 2;
        }

        let merkele_root_hash: String = match tree.is_empty() {

            true => tree.last(),
            false => String("")
        }.unwrap();

        merkele_root_hash
    }

    fn mine_block(&mut self, difficulty:u32){
        let target = "0".repeat(4);


        while(self.hash.starts_with(target.to_string())) {
            self.nonce += 1;
        }
    }
}


struct Blockchain{
    chain: Vec<Block>,
    difficulty: u32,
    balances: HashMap<String, f64>,
    pending_transactions: Vec<Transaction>
}

impl Blockchain{
    fn new() -> Blockchain{
        let genesis_block = Block::new(0, vec![], "0".to_string());
        let difficulty = 4;

        Blockchain{
            chain: vec![genesis_block],
            pending_transactions: vec![],
            balances: HashMap::new(),
            difficulty,

        }
    }

    fn create_block(&self){

        let previous_hash: String = match self.chain.is_empty() {

            true => self.chain.last().unwrap().hash,
            false => "0".to_string()
        }.unwrap();


    }


    fn add_transaction(&mut self, transaction: Transaction){
        self.pending_transactions.push(transaction);
    }

    fn mine_block(&mut self){
        let last_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(last_block.index + 1, self.pending_transactions.clone(), last_block.hash);
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }



    fn is_chain_valid(&self) -> bool{
        for i in 1..self.chain.len(){
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.transactions,
                &current_block.previous_hash,
            ){
                println!("Подстава! Блок не валидный {}", current_block.index);
                return false;
            }

            if current_block.previous_hash != previous_block.hash{
                println!("Неверный хэш предыдущего блока{}", current_block.index);
                return false;
            }
        }

        true
    }

    fn print_block(&self){
        for block in &self.chain{
            println!("{:?}", block)
        }
    }
}



fn main() {

    let mut blockchain = Blockchain::new();

    blockchain.add_transaction(Transaction
        {
            sender: "Александр".to_string(),
            receiver: "Фируз".to_string(),
            amount: 200.0,
        }
    );

    blockchain.add_transaction(Transaction
        {
            sender: "Фируз".to_string(),
            receiver: "Магазин спорт товаров \"Василиса\"".to_string(),
            amount: 100.0,
        }
    );

    blockchain.mine_block();

    blockchain.add_transaction(Transaction
    {
        sender: "Никита".to_string(),
        receiver: "Александр".to_string(),
        amount: 1000.0,
    }
    );

    blockchain.add_transaction(Transaction
    {
        sender: "Александр".to_string(),
        receiver: "Данила".to_string(),
        amount: 777.0,
    }
    );

    blockchain.mine_block();

    blockchain.print_block();

    println!("Валидный ли Blockchain? {}", blockchain.is_chain_valid());

    // blockchain.chain.push(Block{
    //     index: 23515983,
    //     timestamp: 238547983,
    //     transactions: vec![],
    //     hash: "ajikhdgoihqpw8e4uoihgtq47u9iyhtui4yh".to_string(),
    //     previous_hash: "askdjfgpauoiwy38thsdoigju0pw498".to_string(),
    // });
    //
    // blockchain.mine_block();
    //
    // blockchain.print_block();
    //
    // println!("Валидный ли Blockchain? {}", blockchain.is_chain_valid());
}


mod tests{
    use super::*;
    #[test]
    fn test_block_creation() {
        let transactions = vec![
            Transaction {
                sender: "Alice".to_string(),
                receiver: "Bob".to_string(),
                amount: 100.0,
            }
        ];
        let block = Block::new(1, transactions.clone(), "prev_hash".to_string());

        assert_eq!(block.index, 1);
        //assert_eq!(block.transactions, transactions);
        assert_eq!(block.previous_hash, "prev_hash");
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_hash_calculation() {
        let transactions = vec![Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100.0,
        }];

        let hash1 = Block::calculate_hash(1, 1234567890, &transactions, "prev_hash");
        let hash2 = Block::calculate_hash(1, 1234567890, &transactions, "prev_hash");
        let hash3 = Block::calculate_hash(2, 1234567890, &transactions, "prev_hash");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_blockchain_initialization() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert!(blockchain.pending_transactions.is_empty());
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new();
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100.0,
        };

        blockchain.add_transaction(transaction.clone());
        assert_eq!(blockchain.pending_transactions.len(), 1);
        assert_eq!(blockchain.pending_transactions[0].sender, "Alice");
    }

    #[test]
    fn test_mine_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100.0,
        });
        blockchain.mine_block();

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.chain[1].index, 1);
        assert!(blockchain.pending_transactions.is_empty());

        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100.0,
        });

        blockchain.mine_block();
        assert_eq!(blockchain.chain.len(), 3);
        assert_eq!(blockchain.chain[2].index, 2);
        assert!(blockchain.pending_transactions.is_empty());
    }

    #[test]
    fn test_chain_validation() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100.0,
        });
        blockchain.mine_block();

        assert!(blockchain.is_chain_valid());

        // Попробуем изменить данные в блоке
        blockchain.chain[1].transactions[0].amount = 200.0;
        assert!(!blockchain.is_chain_valid());
    }

}