use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let system_time = SystemTime::now();

    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("Current time in seconds since UNIX EPOCH: {}", duration_since_epoch.as_secs());
}
