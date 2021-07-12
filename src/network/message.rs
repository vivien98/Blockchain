use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, Hashable};
use crate::block::{Header,Content,Block};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Ping(String),
    Pong(String),
    NewBlockHashes(Vec<H256>),
    GetBlock(Vec<H256>),
    Block(Vec<Block>)

}
