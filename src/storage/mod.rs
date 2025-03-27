use sled::Db;

use crate::models::{block::Block, transaction::Transaction, utils::Hash};

pub struct BlockchainDB {
    db: Db,
}

impl BlockchainDB {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).unwrap();
        BlockchainDB { db }
    }

    pub fn save_block(&self, block: &Block) {
        self.db.insert(block.hash(), serde_json::to_vec(block).unwrap()).unwrap();
    }

    pub fn save_tx(&self, tx: &Transaction) {
        self.db.insert(tx.hash(), serde_json::to_vec(tx).unwrap()).unwrap();
    }

    pub fn get_block(&self, hash: &Hash) -> Option<Block> {
        let Ok(Some(data)) = self.db.get(hash) else {
            return None;
        };

        let Ok(obj): Result<Block, serde_json::Error> = serde_json::from_slice(&data) else {
            return None;
        };
        Some(obj)
    }

    pub fn get_tx(&self, hash: &Hash) -> Option<Transaction> {
        let Ok(Some(data)) = self.db.get(hash) else {
            return None;
        };

        let Ok(obj): Result<Transaction, serde_json::Error> = serde_json::from_slice(&data) else {
            return None;
        };
        Some(obj)
    }
}
