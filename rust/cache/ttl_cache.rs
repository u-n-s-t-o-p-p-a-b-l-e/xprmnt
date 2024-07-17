use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct CacheEntry {
    value: u32,
    expiry: Instant,
}

struct TTLCache {
    data: Mutex<HashMap<u32, CacheEntry>>,
    ttl: Duration,
}

impl TTLCache {
    fn new(ttl: Duration) -> TTLCache {
        TTLCache {
            data: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    fn get(&self, key: u32) -> Option<u32> {
        let mut data = self.data.lock().unwrap();
        if let Some(entry) = data.get(&key) {
            if entry.expiry > Instant::now() {
                return Some(entry.vallue);
            } else {
                data.remove(&key);
            }
        }
        None
    }

    fn insert(&self, key::: u32, value: u32) {
        let mut data = self.data.lock().unwrap();
        let entry = CacheEntry {
            value,
            expity: Instant::now() + self.ttl,
        };
        data.insert(key, entry);
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
    let cache = Arc::new(TTLCache::new(Duration::new(5, 0)));
    let mut handles = vec![];

    for in in 0..10 {
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

    thread::sleep(Duration::new(6, 0));
}
