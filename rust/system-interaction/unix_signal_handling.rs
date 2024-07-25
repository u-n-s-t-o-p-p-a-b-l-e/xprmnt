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
