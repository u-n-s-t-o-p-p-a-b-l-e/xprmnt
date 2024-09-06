use std::collection::LinkedList;

const TABLE_SIZE: usize = 16;

struct HashTable {
    buckets: Vec<LinkedList<(String, String)>>,
}

impl HashTable {
    fn new() -> self {
        let mut buckets = Vec::with_capacity(TABLE_SIZE);
        for _ in 0..TABLE_SIZE {
            buckets.push(LinkedList::new());
        }
        HashTable { buckets }
    }

    fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 0;

        for byte in key.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as usize);
        }

        hash % TABLE_SIZE
    }

    fn insert(&mut self, key: String, value: String) {
        let bucket_index = self.hash(&key);
        let bucket = &mut self.buckets[bucket_index];
    }
}
