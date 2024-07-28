enum Tree {
    Node(i32, Box<Tree>, Box<Tree>),
    Empty,
}

use Tree::{Node, Empty};

fn main() {
    let left = Node(1, Box::new(Empty), Box::new(Empty));
    let right = Node(2, Box::new(Empty), Box::new(Empty));
    let tree = Node(0, Box::new(left), Box::new(right));

    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    struct Square {
        side: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            3.14 * self.radius * self.radius
        }
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius:  1.0 }),
        Box::new(Square { side:  2.0 }),
    ];

    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}
