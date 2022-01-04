use crate::types::{Hash, Transaction};


struct Block{
    transactions: Vec<Transaction>,
    hash: Hash
}


impl Block {
    fn hash(&self) -> Hash {
        unimplemented!()
    }
}