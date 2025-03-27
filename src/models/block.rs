use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use serde_canonical_json::CanonicalFormatter;
use sha2::{digest::FixedOutput, Digest, Sha256};
use super::{syntactic_validation::SyntacticValidation, utils::Hash};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    previd: Option<Hash>,
    nonce: u64,
    timestamp: u64,
    txids: Vec<Hash>,
    miner: String,
    note: String,
}

impl Block {
    pub fn hash(&self) -> Hash {
        let mut data: Vec<u8> = Vec::new();
        let mut ser = Serializer::with_formatter(&mut data, CanonicalFormatter::new());
        self.serialize(&mut ser).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(&data);

        hasher.finalize_fixed().to_vec()
    }

    /// Returns the Genesis Block
    pub fn genesis() -> Self {
        Self {
            previd: None,
            nonce: 0,
            timestamp: 1743063884,
            // TODO: Add genesis coinbase txid
            txids: vec![vec![0;32]],
            miner: String::from("Sam"),
            note: String::from("Genesis Block"),
        }
    }
}

impl SyntacticValidation for Block {
    fn is_valid(self: &Self) -> bool {
        self.txids.len() > 0 && self.miner.len() > 0
    }
}
