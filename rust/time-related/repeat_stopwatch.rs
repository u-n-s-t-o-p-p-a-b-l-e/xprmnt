use std::thread::sleep;
use std::time::{Duration, Instant};

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

    fn stop(&mut self) {
        if let Some(start_time) = self.start {
            self.duration += start_time.elapsed();
            self.start = None;
        }
    }
}
