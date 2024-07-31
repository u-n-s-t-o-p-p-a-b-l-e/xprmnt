use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::sync::Arc;
use std::io::{self, Write};

fn main() {
    let atomic_counter = Arc::new(AtomicI32::new(0));

    loop {
        print!("Enter the number of threads to spawn (0 to exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num_threads: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                continue;
            },
        };

        if num_threads == 0 {
            println!("Exiting...");
            break;
        }

        let mut handles = vec![];

        for _ in 0..num_threads {
            let atomic_counter_clone = Arc::clone(&atomic_counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    atomic_conter_clone.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }
    }
}
