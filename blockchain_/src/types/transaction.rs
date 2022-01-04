use hex;
use sha2::{Sha256, Digest};

use crate::types::Hash;

pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    fn new(from: String, to: String, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
        }
    }
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();

        hasher.update(format!(
            "{:?}",
            (
                self.from.clone(),
                self.to.clone(),
                self.amount
            )
        ));

        hex::encode(hasher.finalize())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_tx() {
        let tx = Transaction::new("alice".to_string(), "bob".to_string(), 12);
        println!("{}", tx.hash());
    }
}