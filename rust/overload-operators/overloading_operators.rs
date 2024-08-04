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
