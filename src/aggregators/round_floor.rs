use std::time::Duration;

use chrono::{DateTime, Local, TimeZone};

/// Module to calculate bucket time
/// Done by rounding up DateTime<Local> to Duration
pub fn round_floor(dt: DateTime<Local>, interval: Duration) -> DateTime<Local> {
    let i = duration_to_nanos(interval);
    let t = datetime_to_nanos(dt);

    nanos_to_datetime(t - (t % i))
}

fn duration_to_nanos(d: Duration) -> i128 {
    d.as_secs() as i128 * 1_000_000_000 + d.subsec_nanos() as i128
}

fn datetime_to_nanos(dt: DateTime<Local>) -> i128 {
    dt.timestamp() as i128 * 1_000_000_000 + dt.timestamp_subsec_nanos() as i128
}

fn nanos_to_datetime(nanos: i128) -> DateTime<Local> {
    let secs = nanos / 1_000_000_000;
    let nsecs = (nanos % 1_000_000_000) as u32;

    Local
        .timestamp_opt(secs as i64, nsecs)
        .single()
        .expect("valid timestamp")
}
