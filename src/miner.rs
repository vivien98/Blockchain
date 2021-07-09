use crate::network::server::Handle as ServerHandle;

use log::info;

use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use std::time;
use std::time::{Duration, SystemTime, UNIX_EPOCH};


use std::thread;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::block::{Header,Content,Block};
use crate::transaction::{Transaction,SignedTransaction};
use crate::crypto::merkle::{MerkleNode,MerkleTree};
use crate::crypto::hash::{H256, Hashable};
use rand::Rng;



enum ControlSignal {
    Start(u64), // the number controls the lambda of interval between block generation
    Exit,
}

enum OperatingState {
    Paused,
    Run(u64),
    ShutDown,
}

pub struct Context {
    /// Channel for receiving control signal
    control_chan: Receiver<ControlSignal>,
    operating_state: OperatingState,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>
}

#[derive(Clone)]
pub struct Handle {
    /// Channel for sending signal to the miner thread
    control_chan: Sender<ControlSignal>,
}

pub fn new(
    server: &ServerHandle,
    blockchain: &Arc<Mutex<Blockchain>>
) -> (Context, Handle) {
    let (signal_chan_sender, signal_chan_receiver) = unbounded();

    let ctx = Context {
        control_chan: signal_chan_receiver,
        operating_state: OperatingState::Paused,
        server: server.clone(),
        blockchain: Arc::clone(blockchain)
    };

    let handle = Handle {
        control_chan: signal_chan_sender,
    };

    (ctx, handle)
}

impl Handle {
    pub fn exit(&self) {
        self.control_chan.send(ControlSignal::Exit).unwrap();
    }

    pub fn start(&self, lambda: u64) {
        self.control_chan
            .send(ControlSignal::Start(lambda))
            .unwrap();
    }

}

impl Context {
    pub fn start(mut self) {
        thread::Builder::new()
            .name("miner".to_string())
            .spawn(move || {
                self.miner_loop();
            })
            .unwrap();
        info!("Miner initialized into paused mode");
    }

    fn handle_control_signal(&mut self, signal: ControlSignal) {
        match signal {
            ControlSignal::Exit => {
                info!("Miner shutting down");
                self.operating_state = OperatingState::ShutDown;
            }
            ControlSignal::Start(i) => {
                info!("Miner starting in continuous mode with lambda {}", i);
                self.operating_state = OperatingState::Run(i);
            }
        }
    }

    fn miner_loop(&mut self) {
        // main mining loop
        loop {
            // check and react to control signals
            match self.operating_state {
                OperatingState::Paused => {
                    let signal = self.control_chan.recv().unwrap();
                    self.handle_control_signal(signal);
                    continue;
                }
                OperatingState::ShutDown => {
                    return;
                }
                _ => match self.control_chan.try_recv() {
                    Ok(signal) => {
                        self.handle_control_signal(signal);
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => panic!("Miner control channel detached"),
                },
            }
            if let OperatingState::ShutDown = self.operating_state {
                return;
            }

            // TODO: actual mining
            let mut rng = rand::thread_rng();
            let parent: H256 = self.blockchain.lock().unwrap().tip();
            let timestamp: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("dafuq").as_millis();
            let difficulty: H256 = self.blockchain.lock().unwrap().blockMap[&parent].header.difficulty;
            let nonce: u32 = rng.gen();

            let sig1: [u8;32] = rng.gen();
            let sig2: [u8;32] = rng.gen();
            let sign1: H256 = sig1.into();
            let sign2: H256 = sig2.into();
            let signature: [H256;2] = [sign1,sign2];
            let faltu_transaction1 = SignedTransaction{input: String::from("This is random faltugiri"), output: String::from("Blockchain"), amount: 1.00, signature: signature};
            let faltu_transaction2 = SignedTransaction{input: String::from("This is random faltugiri"), output: String::from("Blockchain"), amount: 0.20, signature: signature};
            let faltu_transaction3 = SignedTransaction{input: String::from("This is random faltugiri"), output: String::from("Blockchain"), amount: 0.03, signature: signature};

            let mut data: Vec<SignedTransaction> = Vec::new();
            data.push(faltu_transaction1.clone());
            data.push(faltu_transaction2.clone());
            data.push(faltu_transaction3.clone());

            let content = Content{data: data.clone()};

            let merkle_tree : MerkleTree = MerkleTree::new(&data);
            let header = Header{parent : parent, nonce: nonce, difficulty: difficulty,timestamp: timestamp,merkle_root: merkle_tree.root()};

            let block = Block{header: header, content: content};

            if block.hash() <= difficulty {
                self.blockchain.lock().unwrap().insert(&block);
                println!("{:?}", self.blockchain.lock().unwrap().chainLength );
            }

            if let OperatingState::Run(i) = self.operating_state {
                if i != 0 {
                    let interval = time::Duration::from_micros(i as u64);
                    thread::sleep(interval);
                }
            }
        }
    }
}
