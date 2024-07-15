use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter1 = Arc::clone(&counter1);
        let counter2 = Arc::clone(&counter2);
        let handle = thread::spawn(move || {
            let mut num1 = counter1.lock().unwrap();
            let mut num2 = counter2.lock().unwrap();
            *num1 += i;
            *num2 += 10 - 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter 1: {}", *counter1.lock().unwrap());
    println!("Counter 2: {}", *counter2.lock().unwrap());
}
