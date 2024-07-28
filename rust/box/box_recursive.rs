enum Tree {
    Node(i32, Box<Tree>, Box<Tree>),
    Empty,
}

use Tree::{Node, Empty};

fn main() {
    let left = Node(1, Box::new(Empty), Box::new(Empty));
    let right = Node(2, Box::new(Empty), Box::new(Empty));
    let tree = Node(0, Box::new(Empty), Box::new(Empty));

    trait Shape {
        fn area(&self) -> f64;
    }
}
