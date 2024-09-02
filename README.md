# Bloom Filter for probabilistic inclusion checks
This library is a succinct implementation of a Bloom Filter designed to work with generic Strings.
Use any unique representation e.g. a hash function of your choice to represent your data uniquely.

Define a field by setting `p` to a *sufficiently large number*. Since `p` represents the size of the
output space for all hash functions, it does not need to be prime.

A simple example using just one hash function (Sha256) where the hash of a generic data String (`uid`):
```rust
        let p = 7;
        let mut filter = BloomFilter::new(p);
        // any uid representing your data, e.g. a hash.
        let dummy_data_uid = "Hello World".to_string();
        let dummy_data_uid_sha256_hash = dummy_data_uid.sha256().hex_to_big_int();
        let sha256_index = dummy_data_uid_sha256_hash % p;
        filter.set_index(sha256_index.clone());
        let exists: bool = filter.has_index(&sha256_index);
```
If `exists` is true, the index for this particular hash function exists in the set. Due to the fact that Bloom Filters
are probablistic it is recommended to carefully choose the amount of hash functions used and their output space `p`.

# Hash functions implemented so far (easy to extend on demand)
- sha256
- sha384