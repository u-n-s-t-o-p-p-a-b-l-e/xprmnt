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

    fn insert(&self, value: &T) {
        let node = Box::into_raw(Box::new(Node::new(value.clone())));
        let mut current = self;

        loop {
            match current.value.cmp(value) {
                CmpOrdering::Greater => {
                    match current.left.compare_exchange(
                        ptr::null_mut(),
                        node,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    ) {
                        Ok(_) => break,
                        Err(next) => current = unsafe { &*next },
                    }
                }
            }
        }
    }
}
