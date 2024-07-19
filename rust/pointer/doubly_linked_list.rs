use std::cell::RefCell;
use std::rc::{Rc, Weak}

#[derive(Debug)]
Struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct DoubliLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, value: i32) {
        let new_code = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    fn print_forward(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!();
    }

    fn print_backward(&self) {
        let mut current = self.tail.clone();
        while let Some(node) = current {
            print!("{} ", node.borrow().value);
        }
        println!();
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);

    println!("Forward:");
    list.print_forward();

    println!("Backward:");
    list.print_backward();
}
