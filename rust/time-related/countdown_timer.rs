use std::thread::sleep;
use std::time::Duration;

fn main() {
    let countdown = 10;

    for i in (1..=countdown).rev() {
        println!("Time remaining: {} seconds", i);
        sleep(Duration::from_secs(1));
    }

    println!("Time's up!");
}
