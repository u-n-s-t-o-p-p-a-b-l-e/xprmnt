use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::cmp::Ordering as CmpOrdering;

struct Node<T> {
    value: T,
    left: AtomicPtr<Node<T>>,
    right: AtomicPtr<Node<T>>,
}

impl<T: Ord + Clone> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: AtomicPtr::new(ptr::null_mut()),
            right: AtomicPtr::new(ptr::null_mut()),
        }
    }
}
