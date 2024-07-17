use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Cache {
    data: Mutex<HashMap<u32, u32>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            data: Mutex::new(HashMap::new()),
        }
    }

    fn get(&self, key: u32) -> Option<u32> {
        let data = self.data.lock().unwrap();
        data.get(&key).cloned()
    }

    fn insert(&self, key: u32, value: u32) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    fn compute(&self, key: u32) -> u32 {
        if let Some(value) = self.get(key) {
            return value;
        }
        let value = key * key;
        self.insert(key, value);
        value
    }
}

fn main() {
    let cache = Arc::new(Cache::new());
    let mut handles = vec![];

    for i in 0..10 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let result = cache.compute(i);
            println!("Computed {}: {}", i, rewult);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..10 {
        if let Some(value) = cache.get(i) {
            println!("Cache hit {}: {}", i, value);
        } else {
            println!("Cache miss {}", i);
        }
    }
}
