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
}
