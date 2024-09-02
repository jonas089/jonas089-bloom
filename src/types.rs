use std::collections::HashSet;

use num_bigint::BigInt;
use num_traits::Num;
use sha2::{Digest, Sha256, Sha384};

pub struct BloomFilter {
    pub p: usize,
    pub a: HashSet<BigInt>,
}
impl BloomFilter {
    pub fn new(p: usize) -> Self {
        Self {
            p,
            a: HashSet::new(),
        }
    }
    pub fn set_index(&mut self, v: BigInt) {
        self.a.insert(v);
    }
    pub fn has_index(&self, v: &BigInt) -> bool {
        match self.a.get(v) {
            Some(_) => true,
            None => false,
        }
    }
}

pub trait Hasher {
    fn sha256(&self) -> String;
    fn sha384(&self) -> String;
    fn hex_to_big_int(&self) -> BigInt;
}
impl Hasher for String {
    fn sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    fn sha384(&self) -> String {
        let mut hasher = Sha384::new();
        hasher.update(self);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    fn hex_to_big_int(&self) -> BigInt {
        BigInt::from_str_radix(self, 16).expect("Failed to parse hex string")
    }
}

#[test]
fn test_bloom_indices() {
    // set the field modulus for the bloom filter
    let p = 7;
    // set dummy data hash to be checked against the filter
    let dummy_data_uid = "Hello World".to_string();
    // use different hashing algos to determine the indices
    let dummy_data_uid_sha256_hash = dummy_data_uid.sha256().hex_to_big_int();
    let dummy_data_uid_sha384_hash = dummy_data_uid.sha384().hex_to_big_int();
    let sha256_index = dummy_data_uid_sha256_hash % p;
    let sha384_index = dummy_data_uid_sha384_hash % p;
    // simple assertion for this deterministic test-case, duplicates can occur over a small field
    assert_ne!(sha256_index, sha384_index);
    println!(
        "Sha256 Index: {:?}, Sha384 Index: {:?}",
        &sha256_index, sha384_index
    );
}
