use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter the size of memory to allocate (in bytes, 0 to exit): ");
        io::stdout().flush().unwrap();
    }
}
