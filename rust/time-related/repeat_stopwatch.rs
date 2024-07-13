use std::time::{Duration, Instant};
use std::thread::sleep;

struct Stopwatch {
    start: Option<Instant>,
    duration: Duration,
}

impl Stopwatch {
    fn new() -> Self {
        Stopwatch {
            Start: None,
            duration: Duration::new(0, 0),
        }
    }

    fn start(&mut self) {
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }
    }

    fn
}
