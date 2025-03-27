use k256::PublicKey;
use serde::{Serialize, Deserialize};
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

// TODO: Possibly add a limit to data length in validation
impl SyntacticValidation for CoinbaseTransaction {
    fn is_valid(self: &Self) -> bool {
        self.outputs.len() == 1 && self.outputs[0].value == U256::from(COINBASE_REWARD)
    }
}

impl SyntacticValidation for SpendingTransaction {
    fn is_valid(self: &Self) -> bool {
        self.outputs.len() > 0 && self.inputs.len() > 1
    }
}
