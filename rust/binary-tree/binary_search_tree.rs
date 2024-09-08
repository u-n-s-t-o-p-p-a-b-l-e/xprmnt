use std::comp::Ordering;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Oprion<Box<Node<T>>>,
}
