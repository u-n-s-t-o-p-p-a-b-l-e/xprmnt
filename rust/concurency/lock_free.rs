use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::mem::MaybeUninit;

struct Node<T> {
    data: MaybeUninit<T>,
    next: AtomitPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new() -> Self {
        Node {
            data: MaybeUninit::uninit(),
                  next: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}
