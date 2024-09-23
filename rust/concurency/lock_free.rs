use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::mem::MaybeUninit;

struct Node<T> {
    data: MaybeUninit<T>,
    next: AtomitPtr<Node<T>>,
}
