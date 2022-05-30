use super::message::Message;
use super::peer;
use crate::network::server::Handle as ServerHandle;
use crossbeam::channel;
use log::{debug, warn};

use std::thread;
use std::sync::{Arc, Mutex};
use crate::crypto::hash::H256;
use crate::crypto::hash::Hashable;
use std::collections::HashMap;
use crate::blockchain::Blockchain;
use crate::block::{Header,Content,Block};
use crate::transaction::{SignedTransaction};


#[derive(Clone)]
pub struct Context {
    msg_chan: channel::Receiver<(Vec<u8>, peer::Handle)>,
    num_worker: usize,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>
}

pub fn new(
    num_worker: usize,
    msg_src: channel::Receiver<(Vec<u8>, peer::Handle)>,
    server: &ServerHandle,
    blockchain: &Arc<Mutex<Blockchain>>
) -> Context {
    Context {
        msg_chan: msg_src,
        num_worker,
        server: server.clone(),
        blockchain: Arc::clone(blockchain)
    }
}

impl Context {
    pub fn start(self) {
        let num_worker = self.num_worker;
        for i in 0..num_worker {
            let cloned = self.clone();
            thread::spawn(move || {
                cloned.worker_loop();
                warn!("Worker thread {} exited", i);
            });
        }
    }

    fn worker_loop(&self) {

        let mut orphanBlocks = HashMap::new();// has parent Hash -> child Block mapping

        loop {
            let msg = self.msg_chan.recv().unwrap();
            let (msg, peer) = msg;
            let msg: Message = bincode::deserialize(&msg).unwrap();

            match msg {
                Message::Ping(nonce) => {
                    debug!("Ping: {}", nonce);
                    peer.write(Message::Pong(nonce.to_string()));
                }
                Message::Pong(nonce) => {
                    debug!("Pong: {}", nonce);
                }
                Message::NewBlockHashes(blockHashVec) => {

                    let mut wantedBlocks: Vec<H256> = Vec::new();
                    for blockHash in blockHashVec.iter() {
                        if self.blockchain.lock().unwrap().blockMap.contains_key(blockHash) {
                            continue;
                        }else {
                            wantedBlocks.push(*blockHash);
                            
                        }
                    }
                    if wantedBlocks.len() > 0 {
                        peer.write(Message::GetBlock(wantedBlocks));
                        debug!("want blocks")
                    }
                    
                    
                }
                Message::GetBlock(blockHashVec) => {

                    let mut blockVecToSend: Vec<Block> = Vec::new();
                    for blockHash in blockHashVec.iter() {
                        if self.blockchain.lock().unwrap().blockMap.contains_key(blockHash) {
                            blockVecToSend.push(self.blockchain.lock().unwrap().blockMap[blockHash].clone());
                        }else {
                            continue;                            
                        }
                    }
                    if blockVecToSend.len() > 0 {
                        peer.write(Message::Block(blockVecToSend.clone()));
                        debug!("sending blocks")
                    }

                }
                Message::Block(blockVec) => {

                    //let mut newBlockParentMap = HashMap::new();
                    let mut insertedBlocks: Vec<H256> = Vec::new();
                    let mut getBlocks: Vec<H256> = Vec::new();

                    for block in blockVec.iter() {
                        orphanBlocks.insert((*block).header.parent,block.clone());
                    }

                    for block in blockVec.iter() {
                        let blockHash: H256 = (*block).hash();
                        if self.blockchain.lock().unwrap().blockMap.contains_key(&blockHash) {
                            continue;
                        }else {
                            if self.blockchain.lock().unwrap().blockMap.contains_key(&block.header.parent) && blockHash <= block.header.difficulty {
                                self.blockchain.lock().unwrap().insert(&(block.clone()));
                                insertedBlocks.push(blockHash);
                                orphanBlocks.remove(&(*block).header.parent);
                                let mut parent: H256 = blockHash;
                                while orphanBlocks.contains_key(&parent) {
                                    self.blockchain.lock().unwrap().insert(&(orphanBlocks[&parent].clone()));
                                    let mut new_parent = orphanBlocks[&parent].hash();
                                    orphanBlocks.remove(&parent);
                                    parent = new_parent;
                                    insertedBlocks.push(parent);
                                }
                                debug!("receiving blocks")
                            }else {
                                if blockHash <= block.header.difficulty {
                                    //orphanBlocks.insert((*block).header.parent,block.clone());
                                    getBlocks.push((*block).header.parent);
                                }
                            }
                        }
                    }
                    if getBlocks.len() > 0 {
                         peer.write(Message::GetBlock(getBlocks));
                     } 
                    if insertedBlocks.len() > 0 {
                        self.server.broadcast(Message::NewBlockHashes(insertedBlocks)); 
                    }
                         
                }
                Message::NewTransactionHashes(transHashVec) => {

                    let mut wantedTrans: Vec<H256> = Vec::new();
                    
                    
                }
                Message::GetTransaction(transHashVec) => {

                    let mut transVecToSend: Vec<SignedTransaction> = Vec::new();

                }
                Message::Transaction(transVec) => {

                    
                         
                }
            }
        println!("----------------         Chain length {:?}", self.blockchain.lock().unwrap().chainLength);// , self.blockchain.lock().unwrap().blockMap.keys().len());
        println!("Chain Tip {:?}", self.blockchain.lock().unwrap().tip());
        }
    }
}
