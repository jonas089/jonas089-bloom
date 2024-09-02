pub mod types;

#[cfg(test)]
mod test {
    use crate::types::{BloomFilter, Hasher};
    #[test]
    fn test_bloom_filter() {
        let p = 7;
        let mut filter = BloomFilter::new(p);
        // any uid representing your data, e.g. a hash.
        let dummy_data_uid = "Hello World".to_string();
        let dummy_data_uid_sha256_hash = dummy_data_uid.sha256().hex_to_big_int();
        let dummy_data_uid_sha384_hash = dummy_data_uid.sha384().hex_to_big_int();
        let sha256_index = dummy_data_uid_sha256_hash % p;
        let sha384_index = dummy_data_uid_sha384_hash % p;
        filter.set_index(sha256_index.clone());
        filter.set_index(sha384_index.clone());
        assert!(filter.has_index(&sha256_index) && filter.has_index(&sha384_index));
    }
}
