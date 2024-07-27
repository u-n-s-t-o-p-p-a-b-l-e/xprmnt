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
        let layout = match layout::from_size_align(size, 1) {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Invalid layout for size {}", size);
                continue;;
            },
        };

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            eprintln!("Failed to allocate memory");
            continue;
        }

        println!("Memory allocated at: {:?}", ptr);

        unsafe {
            ptr::write_bytes(ptr, 0xAA, size);
        }

        println!("Memory initialized with pattern 0xAA");

        print!("Press Enter to deallocate the memory...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();

        unsafe {
            dealloc(ptr, layout);
        }

        println!("Memory deallocated");
    }
}
