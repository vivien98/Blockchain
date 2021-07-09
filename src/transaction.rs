use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use rand::Rng;
use untrusted::Input;
use crate::crypto::hash::{H256, Hashable};
use std::convert::TryInto;


#[derive(Serialize, Deserialize, Debug, Default,Clone)]
pub struct Transaction {
    pub input: String,
    pub output: String,
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
    pub input: String,
    pub output: String,
    pub amount: f32,
    pub signature: [H256;2],
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
         let byte_transaction = bincode::serialize(&self).unwrap();
         ring::digest::digest(&ring::digest::SHA256, &byte_transaction).into()
     }

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

    pub fn generate_random_transaction() -> Transaction {
        let mut rng = rand::thread_rng();
        Transaction{input: String::from("Alice"), output: String::from("Bob"), amount: rng.gen_range(0.0,10.0)}
    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}
