pub use transaction::Transaction;

use crate::types::block::Block;

mod block;
mod blockchain;
mod transaction;

pub type Hash = String;
pub type BlockHash = String;
pub type BlockVec = Vec<Block>;
