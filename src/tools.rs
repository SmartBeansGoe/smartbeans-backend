use rocket::data::ToByteUnit;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time in seconds since 1970-01-01.
pub fn epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Convert rocket::Data into String
/// I hope 2 MiB will be enough for everything.
pub async fn data_to_string(data: rocket::Data<'_>) -> String {
    data.open(2.mebibytes())
        .into_string()
        .await
        .unwrap()
        .into_inner()
}