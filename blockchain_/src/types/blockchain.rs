use chrono::prelude::Utc;
use hex;
use sha2::{Digest, Sha256};

use crate::types::block::Block;
use crate::types::Transaction;

#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
    transaction_pool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut hasher = Sha256::new();

        hasher.update(format!("{:?}", Utc::now().timestamp()));

        Blockchain {
            // genesis block hash is timestamp
            blocks: vec![Block::new(Some(hex::encode(hasher.finalize())), None)],

            transaction_pool: vec![],
        }
    }

    pub fn new_transaction(&mut self, from: String, to: String, amount: u64) {
        self.transaction_pool.push(Transaction::new(from, to, amount));
    }

    pub fn new_block(&mut self) {
        let peek = self.transaction_pool.first();
        self.transaction_pool.remove(0);

        Block {
            transactions: vec![peek],
            hash: None,
            prev_hash: self.blocks.pop().hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_blockchain() {
        let mut blockchain = Blockchain::new();
        blockchain.new_transaction("bob".to_string(), "alice".to_string(), 100);


        dbg!(blockchain);
    }
}





