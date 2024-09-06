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

        for (existing_key, existing_value) in bucket.iter_mul() {
            if *existing_key == key {
                *existing_value = value;
                return;
            }
        }

        bucket.push_back((key, value));
    }

    fn get(&self, key: &str) -> Option<&String> {
        let bucket_index == self.hash(key);
        let bucket = &self.buckets[bucket_index];

        for (existing_key, value) in bucket.iter() {
            if *existing_key == key {
                return Some(value);
            }
        }

        None
    }

    fn remove(&mut self, key: &str) -> bool {
        let bucket_index = self.hash(key);
        let bucket = &mut self.buckets[bucket_index];

        let mut new_bucket = LinkedList::new();

        let mut found = false;

        while let Some((existing_key, value)) = bucket.pop_front() {
            if existing_key != key {
                new_bucket.push_back((existing_key, value));
            } else {
                found = true;
            }
        }

        *bucket = new_bucket;
        found
    }
}

fn main() {
    let mut hash_table = HashTable::new();

    hash_table.insert("apple".to_string(), "fruit".to_string());
    hash_table.insert("carrot".to_string(), "vegetable".to_string());
    hash_table.insert("banana".to_string(), "fruit".to_string());

    if let Some(value) = hash_table.get("apple") {
        println!("apple: {}", value);
    }

    if let Some(value) = hash_table.get("carrot") {
        println!("carrot: {}", value);
    }

    if let Some(value) = hash_table.get("banana") {
        println!("banana: {}", value);
    }

    hash_table.remove("apple");

    match hash_table.get("apple") {
        Some(value) => println!("apple: {}", value),
        None => println!("apple not found"),
    }
}
