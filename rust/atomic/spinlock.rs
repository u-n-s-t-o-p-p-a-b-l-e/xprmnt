use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
