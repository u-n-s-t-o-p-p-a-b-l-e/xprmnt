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
    }
}
