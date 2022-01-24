use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use ethereum_types::U256;
use serde::{Deserialize, Serialize};

use crate::types::BlockHash;

use super::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: i64,
    pub nonce: u64,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        nonce: u64,
        previous_hash: BlockHash,
        transactions: Vec<Transaction>,
    ) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp_millis(),
            nonce,
            previous_hash,
            hash: BlockHash::default(),
            transactions,
        };
        block.hash = block.calculate_hash();

        block
    }

pub fn calculate_hash(&self) -> BlockHash {
        let mut hashable_data = self.clone();
        hashable_data.hash = BlockHash::default();
        let serialized = serde_json::to_string(&hashable_data).unwrap();


        let mut byte_hash = <[u8; 32]>::default();
        let mut hasher = Sha256::new();

        hasher.input_str(&serialized);
        hasher.result(&mut byte_hash);

        U256::from(byte_hash)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_initializing() {
        let block = Block::new(10, BlockHash::default(), vec![]);

        dbg!(block);
    }
}


/*

calculate_hash - должен включать транзакции в хеш
насколько оправдано использование ethereum_types::U256 ? С хешом обычно не проводят математические операции по этому он может быть просто строкой \ массивом байт
create_genesis_block - генезис блок можно сминтить только 1 раз (либо если не один то нужно обнулять чейн) + в будущем у тебя чейн может прийти от другой ноды так что ты не всегда генерируешь генезис блок
не пиши функции которые ты не используешь типо get_last_block \ get_all_blocks - код нужно поддерживать а это время 🙂
нужно больше тестов


*/

