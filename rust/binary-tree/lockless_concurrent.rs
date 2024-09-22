use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::cmp::Ordering as CmpOrdering;

struct Node<T> {
    value: T,
    left: AtomicPtr<Node<T>>,
    right: AtomicPtr<Node<T>>,
}
