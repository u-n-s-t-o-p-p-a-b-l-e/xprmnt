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

    {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data_clone.write().unwrap();
            *write_guard += 1;
            println!("Written value: {}", *write_guard);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *data.read().unwrap());
}
