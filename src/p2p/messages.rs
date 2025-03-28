use serde::{Deserialize, Serialize};

use crate::models::{block::Block, transaction::Transaction};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloMessage {
    pub msg: String,
    pub greeting: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub msg: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockMessage {
    pub msg: String,
    pub block: Block,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub msg: String,
    pub tx: Transaction,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetObjectMessage {
    pub msg: String,
    pub hash: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello(HelloMessage),
    Error(ErrorMessage),
    Block(BlockMessage),
    Transaction(TransactionMessage),
    GetObject(GetObjectMessage),
}
