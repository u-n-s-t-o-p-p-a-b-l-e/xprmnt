use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::Write;

extern "C" {
    fn signal(sig: i32, handler: extern fn(i32)) -> extern fn(i32);
}
