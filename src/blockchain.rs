use crate::block::{Header,Content,Block};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, H160, Hashable};
//use crate::crypto::address::H160;
use chrono::{DateTime,Utc};
use crate::transaction::{Transaction,SignedTransaction};
use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::crypto::merkle::{MerkleNode,MerkleTree};
use hex_literal::hex;
use std::convert::TryInto;

pub struct Blockchain {
    pub blockMap : HashMap<H256,Block>,
    pub tip : H256,
    pub chainLength : u32
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        
        let sign1: H256 = (hex!("0000000000000000000000000000000000000000000000000000000000000000")).into();
        let sign2: H256 = (hex!("0000000000000000000000000000000000000000000000000000000000000001")).into();
        let pub_key: H256 = (hex!("0000000000000000000000000000000000000000000000000000000000000002")).into();
        let signature: [H256;2] = [sign1,sign2];
        let address_gen: [u8;32] =  pub_key.hash().into();//(hex!("0000000000000000000000000000000000000000")).into();
        let truncated_addr: [u8;20] = (&address_gen[12..]).try_into().unwrap();

        let genesis_transaction = SignedTransaction{input: truncated_addr.into(), output: truncated_addr.into(), amount: 0.00, pub_key: pub_key, signature: signature};
        let nonce: u32 = 0;
        let timestamp: u128 = UNIX_EPOCH.duration_since(UNIX_EPOCH).expect("dafuq").as_millis();
        let difficulty: H256 = (hex!("3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")).into();
        let mut data: Vec<SignedTransaction> = Vec::new();
        data.push(genesis_transaction.clone());
        let content = Content{data: data.clone()};
        let merkle_tree : MerkleTree = MerkleTree::new(&data);
        let parent: H256 = (hex!("0000000000000000000000000000000000000000000000000000000000000000")).into();
        let header = Header{parent : parent, nonce: nonce, difficulty:difficulty,timestamp:timestamp,merkle_root: merkle_tree.root()}; 
        let gen_block = Block{header: header, content: content};

        let mut blockMap = HashMap::new();
        let genHash: H256 = Hashable::hash(&gen_block);
        blockMap.insert(genHash,gen_block.clone());

        //let mut leaves: Vec<H256> = Vec::new();
        //leaves.push(genHash);

        let mut tip: H256 = genHash;
        Blockchain{blockMap: blockMap, tip: tip, chainLength: 1}
    }

    /// Insert a block into blockchain
    pub fn insert(&mut self, block: &Block) {
        let blockHash: H256 = Hashable::hash(block);
        self.blockMap.insert(blockHash, (*block).clone());

        //let index = self.leaves.iter().position(|x| *x == (*block).header.parent).unwrap();
        //self.leaves.remove(index);
        //self.leaves.push(blockHash);

        let mut count = 1;
        let mut thisParent: H256 = (*block).header.parent;
        while thisParent != (hex!("0000000000000000000000000000000000000000000000000000000000000000")).into() {
            thisParent = self.blockMap[&thisParent].header.parent;
            count = count + 1;
        }
        if count > self.chainLength {
            self.chainLength = count;
            self.tip = blockHash;
        }

    }

    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        self.tip
    }

    /// Get the last block's hash of the longest chain
    //#[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        let mut hashVec: Vec<H256> = Vec::new();
        hashVec.push(self.tip);
        let mut thisParent: H256 = self.blockMap[&(self.tip)].header.parent;
        while thisParent != (hex!("0000000000000000000000000000000000000000000000000000000000000000")).into() {
            
            hashVec.push(thisParent);
            thisParent = self.blockMap[&thisParent].header.parent;
            
        }

        hashVec
    }
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::block::test::generate_random_block;
    use crate::crypto::hash::Hashable;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());

    }
}
