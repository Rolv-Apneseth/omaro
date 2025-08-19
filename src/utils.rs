use std::borrow::Cow;

use chrono::Duration;
use ratatui::layout::{Constraint, Flex, Layout, Rect};

/// Returns the given duration as a human-readable string representation.
pub fn human_duration(duration: chrono::Duration) -> String {
    if duration < Duration::minutes(1) {
        return String::from("<1 min");
    }

    if duration < Duration::minutes(2) {
        return String::from("1 min");
    }

    if duration < Duration::hours(1) {
        return format!("{} mins", duration.num_minutes());
    }

    if duration < Duration::hours(2) {
        return String::from("1 hour");
    }

    if duration < Duration::days(2) {
        return format!("{} hours", duration.num_hours());
    }

    if duration < Duration::days(730) {
        return format!("{} days", duration.num_days());
    }

    format!("{} years", duration.num_days() / 365)
}

/// Returns the given number of bytes as a human-readable string representation.
pub fn human_bytes(mut bytes: u32) -> String {
    let unit = if bytes < 1_000 {
        "B"
    } else if bytes < 1_000_000 {
        bytes /= 1_000;
        "kB"
    } else if bytes < 1_000_000_000 {
        bytes /= 1_000_000;
        "MB"
    } else {
        bytes /= 1_000_000_000;
        "GB"
    };

    format!("{bytes}{unit}")
}

/// Truncates a string to the given number of characters.
pub fn truncate(s: &str, max_chars: usize) -> Cow<'_, str> {
    if max_chars <= 1 {
        return s[..max_chars].into();
    }

    if max_chars >= s.chars().count() {
        return s.into();
    }

    match s.char_indices().nth(max_chars.saturating_sub(1)) {
        None => Cow::from(s),
        Some((idx, _)) => Cow::Owned(format!("{}—", &s[..idx].trim_end())),
    }
}

/// Utility function for centering a [`Rect`] given the horizontal and vertical
/// constraints.
pub fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

#[cfg(test)]
mod test {
    use chrono::Duration;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("abc", 0).chars().count(), 0);
        assert_eq!(truncate("abc", 1).chars().count(), 1);
        assert_eq!(truncate("abc", 2).chars().count(), 2);
        assert_eq!(truncate("abc", 3).chars().count(), 3);
        assert_eq!(truncate("abc", 5).chars().count(), 3);
        assert_eq!(
            truncate("alksdfaksldfklaslkfasfkskladfalk", 7)
                .chars()
                .count(),
            7
        );
        assert_eq!(
            truncate("alksdfaksldfklaslkfasfkskladfalklsdkfks", 18)
                .chars()
                .count(),
            18
        );
    }

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(0), String::from("0B"));
        assert_eq!(human_bytes(10), String::from("10B"));
        assert_eq!(human_bytes(1_000), String::from("1kB"));
        assert_eq!(human_bytes(9_999), String::from("9kB"));
        assert_eq!(human_bytes(999_999), String::from("999kB"));
        assert_eq!(human_bytes(1_000_000), String::from("1MB"));
        assert_eq!(human_bytes(8_200_000), String::from("8MB"));
        assert_eq!(human_bytes(175_500_000), String::from("175MB"));
        assert_eq!(human_bytes(1_000_000_000), String::from("1GB"));
        assert_eq!(human_bytes(2_000_000_000), String::from("2GB"));
    }

    #[test]
    fn test_human_duration() {
        assert_eq!(human_duration(Duration::zero()), String::from("<1 min"));
        assert_eq!(human_duration(Duration::seconds(1)), String::from("<1 min"));
        assert_eq!(human_duration(Duration::seconds(60)), String::from("1 min"));
        assert_eq!(human_duration(Duration::minutes(2)), String::from("2 mins"));
        assert_eq!(
            human_duration(Duration::minutes(60)),
            String::from("1 hour")
        );
        assert_eq!(
            human_duration(Duration::minutes(61)),
            String::from("1 hour")
        );
        assert_eq!(
            human_duration(Duration::minutes(119)),
            String::from("1 hour")
        );
        assert_eq!(human_duration(Duration::hours(2)), String::from("2 hours"));
        assert_eq!(
            human_duration(Duration::hours(17)),
            String::from("17 hours")
        );
        assert_eq!(
            human_duration(Duration::hours(24)),
            String::from("24 hours")
        );
        assert_eq!(
            human_duration(Duration::hours(36)),
            String::from("36 hours")
        );
        assert_eq!(human_duration(Duration::hours(48)), String::from("2 days"));
        assert_eq!(human_duration(Duration::hours(54)), String::from("2 days"));
        assert_eq!(human_duration(Duration::hours(71)), String::from("2 days"));
        assert_eq!(human_duration(Duration::hours(72)), String::from("3 days"));
        assert_eq!(human_duration(Duration::days(80)), String::from("80 days"));
        assert_eq!(
            human_duration(Duration::days(300)),
            String::from("300 days")
        );
        assert_eq!(
            human_duration(Duration::days(600)),
            String::from("600 days")
        );
        assert_eq!(
            human_duration(Duration::days(729)),
            String::from("729 days")
        );
        assert_eq!(human_duration(Duration::days(730)), String::from("2 years"));
        assert_eq!(human_duration(Duration::days(800)), String::from("2 years"));
        assert_eq!(
            human_duration(Duration::days(1200)),
            String::from("3 years")
        );
    }
}
