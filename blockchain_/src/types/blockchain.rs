use std::sync::{Arc, Mutex};

use chrono::Utc;
use thiserror::Error;

use crate::types::block::Block;
use crate::types::transaction::Transaction;
use crate::types::{BlockHash, BlockVec};

type SyncedBlockVec = Arc<Mutex<BlockVec>>;
pub type TransactionVec = Vec<Transaction>;

type SyncedTransactionVec = Arc<Mutex<TransactionVec>>;

#[derive(Error, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BlockchainError {
    #[error("Invalid previous_hash")]
    InvalidPreviousHash,

    #[error("Invalid hash")]
    InvalidHash,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: SyncedBlockVec,
    transaction_pool: SyncedTransactionVec,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        // add the genesis block to the synced vec of blocks
        let mut blocks = BlockVec::default();
        blocks.push(genesis_block);
        let synced_blocks = Arc::new(Mutex::new(blocks));

        Blockchain {
            blocks: synced_blocks,
            transaction_pool: Arc::new(Mutex::new(vec![])),
        }
    }
    fn create_genesis_block() -> Block {
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        let mut block = Block::new(nonce, previous_hash, transactions);

        block.timestamp = 0;

        block.hash = block.calculate_hash();

        block
    }

    pub fn get_last_block(&self) -> Block {
        let blocks = self.blocks.lock().unwrap();

        blocks[blocks.len() - 1].clone()
    }

    pub fn get_all_blocks(&self) -> BlockVec {
        let blocks = self.blocks.lock().unwrap();

        blocks.clone()
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transaction_pool.lock().unwrap();
        transactions.push(transaction);
    }

    pub fn pop_transaction(&self) -> TransactionVec {
        let mut transactions = self.transaction_pool.lock().unwrap();
        let transactions_clone = transactions.clone();
        transactions.clear();

        transactions_clone
    }

    /*pub fn add_block(&self, block: Block) -> Result<()> {
        let mut blocks = self.blocks.lock().unwrap();
        let last = &blocks[blocks.len() - 1];


        // check that the previous_hash is valid
        if block.previous_hash != last.hash {
            return Err(BlockchainError::InvalidPreviousHash.into());
        }

        // check that the hash matches the data
        if block.hash != block.calculate_hash() {
            return Err(BlockchainError::InvalidHash.into());
        }

        // append the block to the end
        blocks.push(block);

        Ok(())
    }

     */

    pub fn new_block(&mut self) {
        let mut blocks = self.blocks.lock().unwrap();
        let last = &blocks[blocks.len() - 1];

        let new_block = Block::new(0, last.hash.to_owned(), self.pop_transaction());

        blocks.push(new_block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_empty_tx_pool() {
        let blockchain = Blockchain::new();

        dbg!(blockchain.clone());
        assert!(blockchain.pop_transaction().is_empty());
    }

    #[test]
    fn add_some_transactions() {
        let mut blockchain = Blockchain::new();
        let tx_1 = create_mock_transaction(10);
        let tx_2 = create_mock_transaction(20);

        blockchain.add_transaction(tx_1);
        blockchain.add_transaction(tx_2);

        blockchain.new_block();
        dbg!(blockchain);
    }

    fn create_mock_transaction(amount: u64) -> Transaction {
        Transaction {
            sender: "bob".to_string(),
            recipient: "alice".to_string(),
            amount,
        }
    }
}
