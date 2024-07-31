use std::time::{SystemTime, UNIX_EPOCH};

fn current_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_sincee(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

fn format_timestamp(timestamp: u64, offset_hours: i32) -> String {
    let adjusted_timestamp = timestamp as i64 + (offset_hours as i64 * 3600);
    let secs = adjusted_timestamp % 60;
    let mins = (adjusted_timestamp / 60) % 60;
    let hours = (adjusted_timestamp / 3600) % 24;
    let days = adjusted_timestamp / 86400;

    format!("Days since epoch: {}, Time: {:02}:{:02}:{:02}", days, hours, mins, secs)
}

fn main() {
    let utc_timestamp = current_timestamp();
    println!("Current UTC timestamp: {}", utc_timestamp);
    println!("Current UTC time: {}", format_timestamp(utc_timestamp, 0));

    let us_offset_hours = -4;
    let sweden_offset_hours = 2;

    println!("Current time in US (Eastern Time): {}", format_timestamp(utc_timestamp, us_offset_hours));
    println!("Current time in sweden: {}", format_timestamp(utc_timestamp, sweden_offset_hours));
}
