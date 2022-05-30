use serde::{Serialize,Deserialize};
use bincode;
use ring::{digest};
use hex;
use std::{string,vec};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use ring::rand;

#[derive(Debug,Serialize,Deserialize)]
struct NameHash {
    name : String,
    hash : String
}

fn main() {
    // let name = String::from("Viraj Nadkarni");// String struct from literal string
    // let encoded : Vec<u8> = bincode::serialize(&name).unwrap(); // convert to vector of bytes, unwrap gives the output in the expected format (Vec in this case)
    // let hashVal = digest::digest(&digest::SHA256,&encoded); // &encoded converts vector to &[u8]
    // let hexHash = hex::encode(hashVal.as_ref()); // as_ref used to get &[u8]

    // let nameHash = NameHash {
    //     name : name,
    //     hash : hexHash
    // };

    // println!("{:?}", nameHash);
    // let encodedStruct : Vec<u8> = bincode::serialize(&nameHash).unwrap(); // this function works on NameHash struct since we derived the Serialize trait
    // println!("{:?}", encodedStruct);
    // let decodedStruct : NameHash = bincode::deserialize(&encodedStruct).unwrap();// this function works on NameHash struct since we derived the Deserialize trait
    // println!("{:?}", decodedStruct);
    // Name converted to hash ^^



    // Convert signature and public key to hash
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref().into()).unwrap();
    let pub_key:[u8;32] = key.public_key().as_ref().try_into().unwrap();
    
    let encoded_pubkey : Vec<u8> = bincode::serialize(&pub_key).unwrap(); 
    println!("{:?}", encoded_pubkey);
    println!("{:?}", pub_key.len());
    let sig1 = Ed25519KeyPair::sign(&key, &encoded_pubkey);
    let sig: [u8;64]= (sig1).as_ref().try_into().unwrap();
    //let encoded_sig : Vec<u8> = bincode::serialize(&sig).unwrap(); 
    //println!("{:?}", encoded_sig);
    // let hashVal = digest::digest(&digest::SHA256,&encoded); // &encoded converts vector to &[u8]
    // let hexHash = hex::encode(hashVal.as_ref()); // as_ref used to get &[u8]

    // let nameHash = NameHash {
    //     name : name,
    //     hash : hexHash
    // };

    // println!("{:?}", nameHash);
    // let encodedStruct : Vec<u8> = bincode::serialize(&nameHash).unwrap(); // this function works on NameHash struct since we derived the Serialize trait
    // println!("{:?}", encodedStruct);
    // let decodedStruct : NameHash = bincode::deserialize(&encodedStruct).unwrap();// this function works on NameHash struct since we derived the Deserialize trait
    // println!("{:?}", decodedStruct);


}
