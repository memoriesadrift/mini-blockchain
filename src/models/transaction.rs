use k256::PublicKey;
use serde::{Serialize, Deserialize};
use serde_canonical_json::CanonicalFormatter;
use serde_json::Serializer;
use sha2::{digest::FixedOutput, Digest, Sha256};
use crate::config::COINBASE_REWARD;

use super::{syntactic_validation::SyntacticValidation, utils::{Hash, U256}};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub txid: Hash,
    pub index: u64,
    pub sig: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    pub to: PublicKey,
    pub value: U256
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoinbaseTransaction {
    pub outputs: Vec<Output>,
    pub height: u64,
    pub data: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingTransaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub data: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transaction {
    Coinbase(CoinbaseTransaction),
    Spending(SpendingTransaction)
}

impl Transaction {
    pub fn hash(&self) -> Hash {
        let mut data: Vec<u8> = Vec::new();
        let mut ser = Serializer::with_formatter(&mut data, CanonicalFormatter::new());
        self.serialize(&mut ser).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(&data);

        hasher.finalize_fixed().to_vec()
    }
}

// TODO: Possibly add a limit to data length in validation
impl SyntacticValidation for CoinbaseTransaction {
    fn is_valid(self: &Self) -> bool {
        self.outputs.len() == 1 && self.outputs[0].value == COINBASE_REWARD
    }
}

impl SyntacticValidation for SpendingTransaction {
    fn is_valid(self: &Self) -> bool {
        self.outputs.len() > 0 && self.inputs.len() > 1
    }
}
