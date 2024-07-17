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
        let value Some(value) = self.get(key) {
            return value;
        }
    }
}
