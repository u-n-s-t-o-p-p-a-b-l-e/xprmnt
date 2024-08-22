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

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if self.offset + size > self.size {
            panic!("Out of memory");
        }

        let ptr = unsafe { self.memory.add(self.offset) };
        self.offset += size;
        ptr
    }

    fn deallocate(mut self) {
        let layout = Layout::from_size_align(self.size, 8).unwrap();
        unsafe {
            dealloc(self.memory, layout);;
        }
    }
}
