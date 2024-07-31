use std::time::{SystemTime, UNIX_EPOCH};

fn current_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_sincee(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}
