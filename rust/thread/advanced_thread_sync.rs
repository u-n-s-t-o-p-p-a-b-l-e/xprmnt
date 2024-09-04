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

    fn write_data(&self, new_data: String) {
        let mut write_lock = self.write_in_progress.lock().unwrap();

        while *write_lock {
            write_lock = self.condvar.wait(write_lock).unwrap();
        }

        *write_lock = true;

        {
            let mut data = self.data.write().unwrap();
            *data = new_data;
            println!("Data written: {}", *data);
        }

        *write_lock = false;
        self.condvar.notify_all();
    }

    fn read_data(&self) -> String {
        let data = self.data.read().unwrap();
        data.clone();
    }
}
