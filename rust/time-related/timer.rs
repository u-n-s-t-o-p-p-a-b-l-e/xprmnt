use std::time::{Duration, Instant};

fn main() {
    let duration = Duration::from_secs(5);
    let start = Instant::now();

    println!("Timer started for 5 seconds...");

    while start.elapsed() < duration {
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Timer ended");
}
