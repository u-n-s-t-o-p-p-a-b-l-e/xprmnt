use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

enum LogLevel {
    INFO,
    WARNING,
    ERROR,
}
