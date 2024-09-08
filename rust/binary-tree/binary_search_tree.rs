use std::cmp::Ordering;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T:Ord + std::fmt::Display> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert (&mut self, new_value: T) {
        match new_value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(new_value);
                } else {
                    self.left = Some(Box::new(Node::new(new_value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(new_value);
                } else {
                    self.right = Some(Box::new(Node::new(new_value)));
                }
            }
            Ordering::Equal => {
                // Do nothing if the new value is equal (no duplicates allowed).
            }
        }
    }

    fn contains(&self, target_value: &T) -> bool {
        match target_value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref left) = self.left {
                    left.contains(target_value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right) = self.right {
                    right.contains(target_value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }

    fn in_order_traversal(&self) {
        if let Some(ref left) = self.left {
            left.in_order_traversal();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.in_order_traversal();
        }
    }
}

fn main() {
    let mut bst = Node::new(10);

    bst.insert(5);
    bst.insert(15);
    bst.insert(3);
    bst.insert(7);
    bst.insert(13);
    bst.insert(17);

    println!("Containt 7? {}", bst.contains(&7));
    println!("Containt 6? {}", bst.contains(&6));

    println!("In-order traversal of the tree:");
    bst.in_order_traversal();
}
