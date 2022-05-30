use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use crate::crypto::hash::{H256, Hashable};

/// A SHA256 hash.
#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Default, Copy)]
pub struct H160([u8; 20]); // big endian u256

impl Hashable for H160 {
    fn hash(&self) -> H256 {
        ring::digest::digest(&ring::digest::SHA256, &self.0).into()
    }
}

impl std::fmt::Display for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 64 {
                0
            } else {
                32 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..32 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[18], &self.0[19]
        )
    }
}

impl std::convert::AsRef<[u8]> for H160 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::From<&[u8; 20]> for H160 {
    fn from(input: &[u8; 20]) -> H160 {
        let mut buffer: [u8; 20] = [0; 20];
        buffer[..].copy_from_slice(input);
        H160(buffer)
    }
}

impl std::convert::From<&H160> for [u8; 20] {
    fn from(input: &H160) -> [u8; 20] {
        let mut buffer: [u8; 20] = [0; 20];
        buffer[..].copy_from_slice(&input.0);
        buffer
    }
}

impl std::convert::From<[u8; 20]> for H160 {
    fn from(input: [u8; 20]) -> H160 {
        H160(input)
    }
}

impl std::convert::From<H160> for [u8; 20] {
    fn from(input: H160) -> [u8; 20] {
        input.0
    }
}

impl std::convert::From<ring::digest::Digest> for H160 {
    fn from(input: ring::digest::Digest) -> H160 {
        let mut raw_hash: [u8; 20] = [0; 20];
        raw_hash[0..20].copy_from_slice(input.as_ref());
        H160(raw_hash)
    }
}

impl Ord for H160 {
    fn cmp(&self, other: &H160) -> std::cmp::Ordering {
        let self_higher = u128::from_be_bytes(self.0[0..10].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self.0[10..20].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other.0[0..10].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other.0[20..20].try_into().unwrap());
        let higher = self_higher.cmp(&other_higher);
        match higher {
            std::cmp::Ordering::Equal => self_lower.cmp(&other_lower),
            _ => higher,
        }
    }
}

impl PartialOrd for H160 {
    fn partial_cmp(&self, other: &H160) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(any(test, test_utilities))]
pub mod tests {
    use super::H160;
    use rand::Rng;

    pub fn generate_random_hash() -> H160 {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 20];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }

}
