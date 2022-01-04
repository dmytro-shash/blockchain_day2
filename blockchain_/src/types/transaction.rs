use hex;
use sha2::{Digest, Sha256};

use crate::types::Hash;

#[derive(Default, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
        }
    }

    pub fn hash(&self) -> Hash {
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
        dbg!(tx.hash());
    }
}