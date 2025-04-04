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
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String ) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = Self::calculate_hash(index, timestamp, &transactions, &previous_hash);

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
        Update::update(&mut hasher, input.as_bytes());
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