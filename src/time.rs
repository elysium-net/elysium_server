use elysium_rust::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> Timestamp {
    Timestamp {
        millis: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // TODO: don't panic
            .as_millis() as u64,
    }
}
