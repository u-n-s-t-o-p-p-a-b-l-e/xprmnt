use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::sync::Arc;
use std::io::{self, Write};

fn main() {
    let atomic_counter = Arc::new(AtomicI32::new(0));

    loop {
        print!("Enter the number of threads to spawn (0 to exit): ");
        io::stdout().flush().unwrap();
    }
}
