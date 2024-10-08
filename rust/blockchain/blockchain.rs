use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{self, Write};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.mine_block();
        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        simple_hash(input)
    }

    fn mine_block(&mut self) -> String {
        loop {
            let hash = self.calculate_hash();
            if &hash[..4] == "00" {
                return hash;
            }
            self.nonce += 1;
            if self.nonce % 1000000 == 0 {
                println!("Nonce: {}, Hash: {}", self.nonce, hash);
            }
        }
    }
}

fn simple_hash(input: String) -> String {
    let mut hash = String::new();
    for byte in input.bytes() {
        write!(hash, "{:x}", byte).unwrap();
    }
    hash
}

#[derive(Debug)]
struct Blockchain {
    chain: VecDeque<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: VecDeque::new(),
        };
        blockchain.chain.push_back(Block::new(0, "Genesis Block".to_string(), "0".to_string()));
        blockchain
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.back().unwrap().clone();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash);
        self.chain.push_back(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Block 1 Data".to_string());
    blockchain.add_block("Block 2 Data".to_string());
    blockchain.add_block("Block 3 Data".to_string());

    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
