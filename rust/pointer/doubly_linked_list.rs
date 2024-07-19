use std::cell::RefCell;
use std::rc::{Rc, Weak}

#[derive(Debug)]
Struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

