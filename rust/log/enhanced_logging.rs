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

            let level_str = match level {
                LogLeve::ERROR => "ERROR",
                LogLeve::WARNING => "WARNING",
                LogLeve::INFO => "INFO",
                LogLeve::DEBUG => "DEBUG",
            };

            let log_entry = format!(
                "[{}] {}: {} ({}:{})\n",
                timestamp, level_str, message, file, line
            );

            print!("{}", log_entry);

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("enhanced_log.txt")
                .unwrap();

            file.write_all(log_entry.as_bytes()).unwrap();
        }
    }
}

macro_rules! log {
    ($logger:expr, $level:expr, $message:expr) => {
        $logger.log($level, $message, file!(), line!());
    };
}

fn main() {
    let logger = Logger::new(LogLevel::INFO);

    log!(logger, LogLevel::INFO, "Application started");
    log!(logger, LogLevel::DEBUG, "Debugging information");
    log!(logger, LogLevel::WARNING, "Low disk space");
    log!(logger, LogLevel::ERROR, "Failed to open file");
}
