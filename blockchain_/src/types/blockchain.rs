use chrono::prelude::Utc;
use hex;
use sha2::{Digest, Sha256};

use crate::types::{Hash, Transaction};
use crate::types::block::Block;


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
        let mut block = Block {
            transactions: vec![self.transaction_pool.first().expect("there is no tx in tx_pool to produce next blocks").clone()],
            hash: None,
            prev_hash: self.get_last_block_hash(),
        };
        self.transaction_pool.remove(0);
        block.update_hash();
        self.blocks.push(block);
    }

    pub fn get_last_block_hash(&self) -> Option<Hash> {
        self.blocks.last().map(|block| block.hash())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_initial_blockchain() {
        let mut blc = Blockchain::new();
        dbg!(blc.clone());
        dbg!(blc.get_last_block_hash());
        blc.new_transaction("bob".to_string(), "alice".to_string(), 100);
        blc.new_transaction("alice".to_string(), "frank".to_string(), 50);
        blc.new_block();
        blc.new_block();
        blc.new_block();

        dbg!(blc);
    }
}





