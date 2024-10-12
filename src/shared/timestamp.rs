use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp_ms() -> u64 {
    let now = SystemTime::now();

    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    duration_since_epoch.as_millis() as u64
}
