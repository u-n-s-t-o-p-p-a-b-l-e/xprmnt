use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::Write;

extern "C" {
    fn signal(sig: i32, handler: extern fn(i32)) -> extern fn(i32);
}

const SIGINT: i32 = 2;

extern fn handle_signal(_: i32) {
    println!("\nReceived Ctrl+C! Exiting...");
    std::process::exit(0);
}


fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    unsafe {
        signal(SIGINT, handle_signal);
    }

    println!("Press Ctrl+C to exit...");
    while r.load(Ordering::SeqCst) {
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}


