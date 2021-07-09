use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, Hashable};
use chrono::{DateTime,Utc};
use crate::transaction::{Transaction,SignedTransaction};
use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::crypto::merkle::{MerkleNode,MerkleTree};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Block {
	pub header: Header,
	pub content: Content
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        Hashable::hash(&self.header)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
	pub parent: H256,
	pub nonce: u32,
	pub difficulty: H256,
	pub timestamp: u128,
	pub merkle_root: H256,
}

impl Hashable for Header {
    fn hash(&self) -> H256 {
        let byte_header = bincode::serialize(&self).unwrap();
        ring::digest::digest(&ring::digest::SHA256, &byte_header).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
	pub data: Vec<SignedTransaction>
}

#[cfg(any(test, test_utilities))]
pub mod test {
    use super::*;
    use crate::crypto::hash::H256;

    pub fn generate_random_block(parent: &H256) -> Block {
    	//unimplemented!();
    	let mut rng = rand::thread_rng();

        let nonce: u32 = rng.gen();
        let sig1: [u8;32] = rng.gen();
        let sig2: [u8;32] = rng.gen();
        let sign1: H256 = sig1.into();
        let sign2: H256 = sig2.into();
        let signature: [H256;2] = [sign1,sign2];
        let genesis_transaction = SignedTransaction{input: String::from("This is random faltugiri"), output: String::from("Blockchain"), amount: 0.00, signature: signature};
        let timestamp: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("dafuq").as_millis();
        let difficulty: H256 = (hex!("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")).into();
        let mut data: Vec<SignedTransaction> = Vec::new();
        data.push(genesis_transaction.clone());
        let content = Content{data: data.clone()};
        let merkle_tree : MerkleTree = MerkleTree::new(&data);
        let header = Header{parent : *parent, nonce: nonce, difficulty:difficulty,timestamp:timestamp,merkle_root: merkle_tree.root()}; 

        let block = Block{header: header, content: content};

        block
    }
}
