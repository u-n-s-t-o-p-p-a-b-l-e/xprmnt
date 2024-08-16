use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Equal => {}
        }
    }

    fn in_order_traversal<'a>(&'a self, values: &mut Vec<&'a T>) {
        if let Some(ref left) = self.left {
            left.in_order_traversal(values);
        }
        values.push(&self.value);
        if let Some(ref right) = self.right {
            right.in_order_traversal(values);
        }
    }
}
