trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; 
}

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    while let Some(value) = counter.next() {
        println!("Counter: {}", value);
    }
}
