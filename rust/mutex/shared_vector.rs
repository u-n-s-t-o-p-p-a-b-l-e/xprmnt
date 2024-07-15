use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(i);
        });
        handle.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let vec = data.lock().unwrap();
    println!("Shared Vector: {:?}", *vec);
}
