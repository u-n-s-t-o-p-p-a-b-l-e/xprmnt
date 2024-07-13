use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let interval = Duration::from_secs(2);
    let mut next_time = Instant::now() + interval;

    for _ in 0..5 {
        println!("Running task...");
        sleep(next_time - Instant::now());
        next_time += interval;
    }

    println!("Finished repeated tasks.");
}
