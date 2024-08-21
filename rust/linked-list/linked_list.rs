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
            if ptr.is_null() {
                panic!("Failed to allocate memory");
            }
            ptr::write(ptr, Node { value, next: ptr::null_mut() });
            ptr
        }
    }

    fn append(&mut self, value: i32) {
        let mut current = self as *mut Node;
        unsafe {
            while !(*current).next.is_null() {
                current = (*current).next;
            }
            (*current).next = Node::new(value);
        }
    }

    fn print(&self) {
        let mut current = self as *const Node;
        unsafe {
            while !current.is_null() {
                print!("{} -> ", (*current).value);
                current = (*current).next;
            }
        }
        println!("null");
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::new::<Node>();
            let mut current = self.next;
            while !current.is_null() {
                let next= (*current).next;
                dealloc(current as *mut u8, layout);
                current = next;
            }
        }
    }
}

fn main() {
    unsafe {
        let head = Node::new(1);
        (*head).append(2);
        (*head).append(3);
        (*head).append(4);
        (*head).print();
        dealloc(head as *mut u8, Layout::new::<Node>());
    }
}
