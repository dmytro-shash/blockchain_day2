use sha2::{Digest, Sha256};

use crate::types::{Hash, Transaction};

#[derive(Default, Debug)]
pub struct Block {
    transactions: Vec<Transaction>,
    hash: Option<Hash>,
    prev_hash: Option<Hash>,
}


impl Block {
    pub fn new(hash: Option<Hash>, prev_hash: Option<Hash>,) -> Self {
        let mut block = Block {
            hash,
            prev_hash,
            ..Default::default()
        };
        block.update_hash();

        block
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_hash();
    }

    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();

        hasher.update(format!("{:?}", self.prev_hash.clone()));

        for tx in self.transactions.iter() {
            hasher.update(tx.hash())
        }

        hex::encode(hasher.finalize())
    }

    fn update_hash(&mut self) {
        self.hash = Some(self.hash());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_block_creation() {
        let mut genesis_block = Block::new(None,None);

        let tx = Transaction::new("alice".to_string(), "bob".to_string(), 10);

        genesis_block.add_transaction(tx);

        dbg!(genesis_block);
    }
}