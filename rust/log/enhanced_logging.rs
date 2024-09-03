use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, PartialOrd)]
enum LogLevel {
    ERROR,
    WARNING,
    INFO,
    DEBUG,
}

struct Logger {
    level: LogLevel,
}

impl Logger {
    fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    fn log(&self, level: LogLevel, message: &str, file: &str, line: u32) {
        if level <= self.level {
            let now = SystemTime::now();
            let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
    }
}
