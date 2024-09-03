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

    let level_str = match level {
        LogLevel::INFO => "INFO",
        LogLevel::WARNING => "WARNING",
        LogLevel::ERROR => "ERROR",
    };

    let log_entry = format!("[{}] {}: {}\n", timestamp, level_str, message);

    print!("{}", log_entry);
}
