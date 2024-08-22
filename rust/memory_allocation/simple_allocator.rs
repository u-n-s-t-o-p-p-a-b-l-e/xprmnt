use std::ptr::{null_mut, write};
use std::alloc::{alloc, dealloc, Layout};

struct SimpleAllocator {
    memory: *mut u8,
    size: usize,
    offset: usize,
}

impl SimpleAllocator {
    fn new(size: usize) -> SimpleAllocator {
        let layout = Layout::from_size_align(size, 8).unwrap();
        let memory = unsafe { alloc(layout) };
        if memory.is_null() {
            panic!("Failed to allocate memory");
        }
        SimpleAllocator {
            memory,
            size,
            offset: 0,
        }
        
    }
}
