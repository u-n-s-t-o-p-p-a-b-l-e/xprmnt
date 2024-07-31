use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

struct SpinLock {
    locked: AtomicBool,
}
