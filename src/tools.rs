use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time in seconds since 1970-01-01.
pub fn epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}