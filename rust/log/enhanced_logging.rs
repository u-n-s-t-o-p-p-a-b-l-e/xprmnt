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
}
