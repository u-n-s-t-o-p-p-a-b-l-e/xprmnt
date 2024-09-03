use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

enum LogLevel {
    INFO,
    WARNING,
    ERROR,
}

fn log_message(level: LogLevel, message: &str) {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
}
