struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let current = self.count;
            self.count += 1;
            Some(current)
        } else {
            None
        }
    } 
}

fn main() {
    let mut counter = Counter { count: 0, max: 5 };

    while let Some(number) = counter.next() {
        println!("{}", number);
    }
}
