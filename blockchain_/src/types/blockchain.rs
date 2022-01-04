use crate::types::block::Block;

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new(None, None)]
        }
    }
}