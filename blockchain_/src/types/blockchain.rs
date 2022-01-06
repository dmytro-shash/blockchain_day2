use std::sync::{Arc, Mutex};

use crate::types::BlockVec;

type SyncedBlockVec = Arc<Mutex<BlockVec>>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub difficulty: u32,
    blocks: SyncedBlockVec,
}