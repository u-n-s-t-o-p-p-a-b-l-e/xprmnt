use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct Node {
    value: i32,
    next: *mut Node,
}

impl Node {
    fn new(value: i32) -> *mut Node {
        unsafe {
            let layout = Layout::new::<Node>();
            let ptr = alloc(layout) as *mut Node;
            if prt.is_null() {
                panic!("Failed to allocate memory");
            }
            ptr::write(ptr, Node { value, next: ptr::null_mut() });
            ptr
        }
    }
}
