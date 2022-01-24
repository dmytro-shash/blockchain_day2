use chrono::prelude::*;

use serde::{Deserialize, Serialize};

use blake2::{Blake2s256, Digest};

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
    pub fn new(nonce: u64, previous_hash: BlockHash, transactions: Vec<Transaction>) -> Block {
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
        // сериализируем блок, где хранятся вcе метаданные (в т.ч. транзакции )
        // следовательно включаем их все для подсчета хеша

        // let mut hasher: CoreWrappzer<CtVariableCoreWrapper<Blake2sVarCore, FixedOutput>> = Blake2s::new();
        let mut hasher = Blake2s256::new();

        hasher.update(
            format!(
                "{:?}",
                (
                    self.previous_hash.clone(),
                    self.nonce.clone(),
                    self.transactions.clone(),
                    self.timestamp.clone()
                )
            )
            .as_bytes(),
        );

        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_initializing() {
        let block = Block::new(10, BlockHash::default(), vec![]);
        assert_eq!(block.hash, block.calculate_hash());
    }

}

/*

calculate_hash - должен включать транзакции в хеш
насколько опра  вдано использование ethereum_types::U256 ? С хешом обычно не проводят математические операции по этому он может быть просто строкой \ массивом байт
create_genesis_block - генезис блок можно сминтить только 1 раз (либо если не один то нужно обнулять чейн) + в будущем у тебя чейн может прийти от другой ноды так что ты не всегда генерируешь генезис блок
не пиши функции которые ты не используешь типо get_last_block \ get_all_blocks - код нужно поддерживать а это время 🙂
нужно больше тестов


*/
