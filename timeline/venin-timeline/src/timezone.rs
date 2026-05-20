use chrono::{DateTime, FixedOffset, Utc};

pub fn convert_utc_to_fixed_offset(
    timestamp: DateTime<Utc>,
    offset: FixedOffset,
) -> DateTime<FixedOffset> {
    timestamp.with_timezone(&offset)
}

pub fn parse_fixed_offset(value: &str) -> Option<FixedOffset> {
    let trimmed = value.trim();
    if trimmed.eq_ignore_ascii_case("utc") || trimmed == "Z" || trimmed == "+00:00" {
        return FixedOffset::east_opt(0);
    }

    let sign = match trimmed.as_bytes().first()? {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };

    let (hours, minutes) = trimmed.get(1..)?.split_once(':')?;
    let hours: i32 = hours.parse().ok()?;
    let minutes: i32 = minutes.parse().ok()?;

    if hours > 23 || minutes > 59 {
        return None;
    }

    FixedOffset::east_opt(sign * ((hours * 60 * 60) + (minutes * 60)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_positive_offset() {
        let offset = parse_fixed_offset("+09:30").unwrap();
        assert_eq!(offset.local_minus_utc(), 34_200);
    }

    #[test]
    fn converts_utc_to_fixed_offset() {
        let timestamp = chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let converted =
            convert_utc_to_fixed_offset(timestamp, parse_fixed_offset("+09:30").unwrap());
        assert_eq!(converted.to_rfc3339(), "2024-01-01T09:30:00+09:30");
    }
}
