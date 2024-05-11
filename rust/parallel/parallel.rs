use std::thread;
use std::sync::{Arc, Mutex};

fn sum_of_squares(start: u64, end: u64) -> u64 {
    (start..=end).map(|x| x * x).sum()
}

fn main() {
    let start = 1;
    let end = 1000;
    let num_threads = 4;

    let result = Arc::new(Mutex::new(0u64));

    let chunk_size = (end - start + 1) / num_threads as u64;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = start + i * chunk_size;
        let end = if i == num_threads -1 {
            end
        } else {
            start + chunk_size - 1
        };

        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let chunk_result = sum_of_squares(start, end);
            let mut acc = result.lock().unwrap();
            *acc += chunk_result;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_result = *result.lock().unwrap();
    println!("Sum of squares from {} to {} is: {}", start, end, final_result);
}
