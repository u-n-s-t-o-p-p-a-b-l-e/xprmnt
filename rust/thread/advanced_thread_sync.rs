use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::Duration;

struct SharedData {
    data: RwLock<String>,
    write_in_progress: Mutex<bool>,
    condvar: Condvar,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            data: RwLock::new(String::new()),
            write_in_progress: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }
}
