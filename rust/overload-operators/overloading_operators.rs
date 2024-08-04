use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point {
        fn sub(self, other: Point) -> Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };

    let p3 = p1 + p2;
    println!("p1 + p2 = {:?}", p3);

    let p4 = p2 - p1;
    println!("p2 - p1 = {:?}", p4);

    let p5 = p1 * 2.0;
    println!("p1 * 2.0 = {:?}", p5);
}
