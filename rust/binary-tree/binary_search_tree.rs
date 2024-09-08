use std::comp::Ordering;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Oprion<Box<Node<T>>>,
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
        }
    }
}
