use std::thread;
use std::time::Duration;

fn scheduled_task() {
    println!("Task executed");
}

fn main() {
    let delay = Duration::from_secs(5);

    println!("Tas will be executed after 5 seconds...");

    let handle = therad::spawn(move || {
        thread::sleep(delay);
        scheduled_task();
    });

    handle.join().unwrap();
}
