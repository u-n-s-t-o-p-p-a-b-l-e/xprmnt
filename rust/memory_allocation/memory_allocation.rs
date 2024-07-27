use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::io::{self, Write};

fn main() {
    let size = 1024;

    let layout::from_size_align(size, 1).expect("Failed to create layout");

    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        eprintln!("Failed to allocate memory");
        return;
    }
}
