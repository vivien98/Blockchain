use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use rand::Rng;
use untrusted::Input;
use crate::crypto::hash::{H256, H160, Hashable};
//use crate::crypto::address::{H160};
use std::convert::TryInto;
use crate::crypto::key_pair;


#[derive(Serialize, Deserialize, Debug, Default,Clone)]
pub struct Transaction {
    pub input: H160,
    pub output: H160,
    pub amount: f32,
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
         let byte_transaction = bincode::serialize(&self).unwrap();
         ring::digest::digest(&ring::digest::SHA256, &byte_transaction).into()
     }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedTransaction {
    pub input: H160,
    pub output: H160,
    pub amount: f32,
    pub pub_key: H256,
    pub signature: [H256;2],
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
         let byte_transaction = bincode::serialize(&self).unwrap();
         ring::digest::digest(&ring::digest::SHA256, &byte_transaction).into()
    }
}

pub fn generate_random_signed_transaction() -> (Transaction,SignedTransaction,Ed25519KeyPair) {
    let mut rng = rand::thread_rng();
    let key1 = key_pair::random();
    let key2 = key_pair::random();
    let addr1 = convertPubKeyToH160(&(key1.public_key()));
    let addr2 = convertPubKeyToH160(&(key2.public_key()));
    
    let trans = Transaction{input: addr1, output: addr2, amount: rng.gen_range(0.0,10.0)};
    let sig = sign(&trans,&key1);
    let sigH256: [H256;2] = convertSigToH256(&sig);
    let pub_key = convertPubKeyToH256(&(key1.public_key()));
    let signed_trans = SignedTransaction{input: addr1, output: addr2, amount: rng.gen_range(0.0,10.0),pub_key: pub_key,signature:sigH256};

    (trans,signed_trans,key1)
}

pub fn convertPubKeyToH160(public_key: &<Ed25519KeyPair as KeyPair>::PublicKey) -> H160 {
    let pub_key:[u8;32] = public_key.as_ref().try_into().unwrap();
    let pub_key1: H256 = pub_key.into();
    let hash_pub_key: [u8;32] = pub_key1.hash().into();
    let truncated: [u8;20] = pub_key[12..].try_into().unwrap();
    truncated.into()
}

pub fn convertPubKeyToH256(public_key: &<Ed25519KeyPair as KeyPair>::PublicKey) -> H256 {
    let pub_key:[u8;32] = public_key.as_ref().try_into().unwrap();
    let result: H256 = pub_key.into();
    result
}

pub fn convertSigToH256(signature: &Signature) -> [H256;2] {
    let inner_array: [u8;64]= (signature).as_ref().try_into().expect("incorrect length");
    let refer1: [u8;32] = (&inner_array[0..32]).try_into().expect("incorrect length");
    let refer2: [u8;32] = (&inner_array[32..64]).try_into().expect("incorrect length");
    let sig1: H256 = refer1.into();
    let sig2: H256 = refer2.into();
    [sig1,sig2]
}

pub fn convertH256ToSigRef(signature: [H256;2]) -> Vec<u8> {
    let refer1: [u8;32] = signature[0].into();
    let refer2: [u8;32] = signature[1].into();
    [refer1,refer2].concat()
}

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    let byte_transaction = bincode::serialize(&t).unwrap();
    let sig = Ed25519KeyPair::sign(&key, &byte_transaction);
    sig
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey, signature: &Signature) -> bool {
    let trans: &[u8] = &(&(bincode::serialize(&t).unwrap()));
    let msg = Input::from(trans);
    let sig = Input::from(signature.as_ref());
    let pub_key = Input::from(public_key.as_ref());
    let result = VerificationAlgorithm::verify(&EdDSAParameters,pub_key,msg,sig).is_ok();
    result
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*; 
    use crate::crypto::key_pair;

    pub fn generate_random_transaction() -> (Transaction,SignedTransaction,Ed25519KeyPair) {
        let mut rng = rand::thread_rng();
        let key1 = key_pair::random();
        let key2 = key_pair::random();
        let addr1 = convertPubKeyToH160(&(key1.public_key()));
        let addr2 = convertPubKeyToH160(&(key2.public_key()));
        
        let trans = Transaction{input: addr1, output: addr2, amount: rng.gen_range(0.0,10.0)};
        let sig = sign(&trans,&key1);
        let sigH256: [H256;2] = convertSigToH256(&sig);
        let pub_key = convertPubKeyToH256(&(key1.public_key()));
        let signed_trans = SignedTransaction{input: addr1, output: addr2, amount: rng.gen_range(0.0,10.0),pub_key: pub_key,signature:sigH256};

        (trans,signed_trans,key1)
    }

    #[test]
    fn sign_verify() {
        let (t,signed_trans,key) = generate_random_transaction();
        //let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}
