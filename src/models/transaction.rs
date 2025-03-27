use k256::PublicKey;
use serde::{Serialize, Deserialize};
use crate::config::COINBASE_REWARD;

use super::{syntactic_validation::SyntacticValidation, utils::{Hash, U256}};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    txid: Hash,
    index: u64,
    sig: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    to: PublicKey,
    value: U256
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoinbaseTransaction {
    outputs: Vec<Output>,
    height: u64,
    data: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingTransaction {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    data: Vec<u8>
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
