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
}
