use chrono::{TimeZone, Utc};

#[allow(unused)]
pub trait UtcFromMillis {
    fn utc_from_millis(&self) -> chrono::DateTime<Utc>;
}

impl UtcFromMillis for u64 {
    fn utc_from_millis(&self) -> chrono::DateTime<Utc> {
        let secs = (self / 1_000) as i64;
        let nsecs = ((self % 1_000) * 1_000_000) as u32;
        Utc.timestamp_opt(secs, nsecs)
            .unwrap()
    }
}
