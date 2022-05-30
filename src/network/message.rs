use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, Hashable};
use crate::block::{Header,Content,Block};
use crate::transaction::{SignedTransaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Ping(String),
    Pong(String),
    NewBlockHashes(Vec<H256>),
    GetBlock(Vec<H256>),
    Block(Vec<Block>),
    NewTransactionHashes(Vec<H256>),
    GetTransaction(Vec<H256>),
    Transaction(Vec<SignedTransaction>)
}
