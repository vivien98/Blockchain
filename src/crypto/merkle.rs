use super::hash::{Hashable, H256};

/// A Merkle tree.
#[derive(Debug, Default,Clone)]
pub struct MerkleTree {
    root: MerkleNode,
    node_array_by_levels: Vec<Vec<MerkleNode>>,
}

#[derive(Debug, Default,Clone)]
pub struct MerkleNode {
    value: H256,
    child1: Option<Box<MerkleNode>>,
    child2: Option<Box<MerkleNode>>,
    parent: Option<Box<MerkleNode>>,
    is_leaf: bool,
}

pub fn sibling_hash(node: &MerkleNode)-> H256{
    let parent: MerkleNode = *(node.parent.clone().unwrap());
    let child1: MerkleNode = *(parent.child1.clone().unwrap());
    let child2: MerkleNode = *(parent.child2.clone().unwrap());
    if child1.value == node.value {
        child2.value
    }else{
        child1.value

    }
}

impl MerkleTree {

    pub fn new<T:Hashable+Clone>(data: &[T]) -> MerkleTree {
        let size_of_data = data.len();
        let mut vector_data = data.to_vec();
        if size_of_data % 2 == 1 {
            vector_data.push(vector_data[size_of_data-1].clone());
        }
        let mut hashed_data = vec![];
        for datum in vector_data {
            let mut hashed_datum = MerkleNode{value: Hashable::hash(&datum),child1:None,child2:None,parent:None,is_leaf: true};
            hashed_data.push(hashed_datum);
        }
        let mut prev_level: Vec<MerkleNode> = hashed_data;
        let mut node_array_by_levels = vec![];
        while prev_level.len() > 1 {
            let mut size_of_level = prev_level.len();
            if size_of_level % 2 == 1 {
                let mut last = prev_level[size_of_level-1].clone();
                last.is_leaf = true;
                prev_level.push(last);
            }
            size_of_level = prev_level.len();
            let mut this_level = vec![];
            
            for i in 0..size_of_level/2 {
                let c1 : &[u8] = prev_level[2*i].value.as_ref();
                let c2 : &[u8] = prev_level[2*i+1].value.as_ref();
                let concatenated: &[u8] = &[c1,c2].concat();
                let root: H256 = ring::digest::digest(&ring::digest::SHA256, &concatenated).into();
                let mut new_node : MerkleNode = MerkleNode{value: root, child1: Some(Box::new(prev_level[2*i].clone())), child2: Some(Box::new(prev_level[2*i+1].clone())),parent:None,is_leaf:false};
                this_level.push(new_node);
                prev_level[2*i].parent = Some(Box::new(this_level[i].clone()));
                prev_level[2*i+1].parent = Some(Box::new(this_level[i].clone()));
            }
            node_array_by_levels.push(prev_level.clone());
            prev_level = this_level;
        }
        let mut root: MerkleNode = prev_level[0].clone();
        MerkleTree{root: root, node_array_by_levels: node_array_by_levels}
    }
    
    pub fn root(&self) -> H256 {
        self.root.value
    }

    /// Returns the Merkle Proof of data at index i
    pub fn proof(&self, index: usize) -> Vec<H256> {
        
        let mut result_vec = vec![];
        let mut trail: MerkleNode = self.node_array_by_levels[0][index].clone();;
        for level in self.node_array_by_levels.clone() {
            result_vec.push(sibling_hash(&trail));
            trail = *(trail.parent.unwrap());
        }
        result_vec

    }
}


/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {

    let mut trail_hash: H256 = *datum;
    let mut level_idx: usize = index;
    for hash in proof {
        let c1 : &[u8] = (*hash).as_ref();
        let c2 : &[u8] = trail_hash.as_ref();

        if level_idx % 2 == 1 {
            let concatenated = &[c1,c2].concat();
            trail_hash = ring::digest::digest(&ring::digest::SHA256, &concatenated).into();
        }else {
            let concatenated = &[c2,c1].concat();
            trail_hash = ring::digest::digest(&ring::digest::SHA256, &concatenated).into();
        }
        level_idx = level_idx/2;
    }
    (trail_hash == *root)
}

#[cfg(test)]
mod tests {
    use crate::crypto::hash::H256;
    use super::*;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    #[test]
    fn root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }
}
