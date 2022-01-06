use ethereum_types::U256;

pub use transaction::Transaction;

use crate::types::block::Block;

mod transaction;
mod block;
mod blockchain;

pub type Hash = String;
pub type BlockHash = U256;
pub type BlockVec = Vec<Block>;