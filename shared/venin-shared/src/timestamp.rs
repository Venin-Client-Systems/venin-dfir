use chrono::{DateTime, TimeZone, Utc};

pub const CHROMIUM_WEBKIT_TO_UNIX_SECONDS: i64 = 11_644_473_600;
const MICROS_PER_SECOND: i64 = 1_000_000;

pub fn chromium_webkit_microseconds_to_utc(value: i64) -> Option<DateTime<Utc>> {
    if value <= 0 {
        return None;
    }

    let unix_micros = value.checked_sub(CHROMIUM_WEBKIT_TO_UNIX_SECONDS * MICROS_PER_SECOND)?;
    let seconds = unix_micros.div_euclid(MICROS_PER_SECOND);
    let micros = unix_micros.rem_euclid(MICROS_PER_SECOND);
    Utc.timestamp_opt(seconds, (micros as u32) * 1_000).single()
}

pub fn unix_seconds_to_chromium_webkit_microseconds(unix_seconds: i64) -> i64 {
    (unix_seconds + CHROMIUM_WEBKIT_TO_UNIX_SECONDS) * MICROS_PER_SECOND
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_chromium_epoch_to_unix_epoch() {
        let raw = unix_seconds_to_chromium_webkit_microseconds(0);
        let converted = chromium_webkit_microseconds_to_utc(raw).unwrap();
        assert_eq!(converted.to_rfc3339(), "1970-01-01T00:00:00+00:00");
    }

    #[test]
    fn converts_deterministic_utc_timestamp() {
        let raw = unix_seconds_to_chromium_webkit_microseconds(1_704_067_200);
        let converted = chromium_webkit_microseconds_to_utc(raw).unwrap();
        assert_eq!(converted.to_rfc3339(), "2024-01-01T00:00:00+00:00");
    }

    #[test]
    fn treats_zero_as_absent_timestamp() {
        assert!(chromium_webkit_microseconds_to_utc(0).is_none());
    }
}
