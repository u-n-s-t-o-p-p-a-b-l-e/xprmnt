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

    fn reset(&mut self) {
        self.start = None;
        self.duration = Duration::new(0, 0);
    }

    fn elapsed(&self) -> Duration {
        match self.start {
            Some(start_time) => self.duration + start_time.elapsed(),
            None => self.duration,
        }
    }
}

fn main() {
    stopwatch.start();
    sleep(Duration::from_secs(3));
    stopwatch.stop();
    println!("Elapsed time: {:?}", stopwatch.elapsed());

    stopwatch.reset();
    println!("After reset: {:?}", stopwatch.elapsed);

    stopwatch.start();
    sleep(Duration::from_secs(2));
    println!("Elapsed time after restart: {:?}", stopwatch.elapsed());
}
