use std:sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(5));

    let mut handles = vec![];

    for _ in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data_clone.read().unwrap();
            println!("Read value: {}", *read_guard);
        });
        handles.push(handle);
    }
}
