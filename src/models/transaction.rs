use k256::PublicKey;
use serde::{Serialize, Deserialize};
use super::utils::{Hash, U256};

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
    height: u64
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingTransaction {
    inputs: Vec<Input>,
    outputs: Vec<Output>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transaction {
    Coinbase(CoinbaseTransaction),
    Spending(SpendingTransaction)
}
