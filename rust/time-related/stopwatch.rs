use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    let sum: u64 = (1..=1_000_000).sum();
}
