use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter the size of memory to allocate (in bytes, 0 to exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let size: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                continue;
            },
        };

        if size == 0 {
            println!("Exiting...");
            break;
        }
    }
}
