use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    let sum: u64 = (1..=1_000_000).sum();

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("Sum: {}", sum);

    println!("Sleeping for 2 seconds...");
}
