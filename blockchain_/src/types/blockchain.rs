use std::sync::{Arc, Mutex};

use anyhow::Result;
use thiserror::Error;

use crate::types::{BlockHash, BlockVec};
use crate::types::block::Block;

type SyncedBlockVec = Arc<Mutex<BlockVec>>;

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


    pub fn add_block(&self, block: Block) -> Result<()> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_initializing() {
        let blockchain = Blockchain::new();

        dbg!(blockchain);
    }
}


