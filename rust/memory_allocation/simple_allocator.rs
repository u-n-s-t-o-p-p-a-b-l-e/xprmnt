use std::ptr::{null_mut, write};
use std::alloc::{alloc, dealloc, Layout};

struct SimpleAllocator {
    memory: *mut u8,
    size: usize,
    offset: usize,
}
