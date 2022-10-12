use chrono::Duration;
use std::fmt;

pub const MINUTE: i64 = 60;
pub const HOUR: i64 = 3_600;
pub const DAY: i64 = 86_400;

pub trait HumanDuration {
    type Displayer: fmt::Display;
    fn as_human(&self) -> Self::Displayer;
}

impl HumanDuration for Duration {
    type Displayer = HumanDurationDisplay;
    fn as_human(&self) -> Self::Displayer {
        HumanDurationDisplay(self.clone())
    }
}

pub struct HumanDurationDisplay(Duration);

impl fmt::Display for HumanDurationDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.0.num_seconds();

        let mut seconds = total_seconds % DAY;
        let days = total_seconds / DAY;

        let hours = seconds / HOUR;
        seconds %= HOUR;

        let minutes = seconds / MINUTE;
        seconds %= MINUTE;

        let mut res = String::new();
        if days > 0 {
            res.push_str(&format!("{}d", days));
        }
        if hours > 0 {
            if !res.is_empty() {
                res.push_str(" ");
            }
            res.push_str(&format!("{}h", hours));
        }
        // only show minutes if days are not shown
        if minutes > 0 && days < 1 {
            if !res.is_empty() && hours < 1 {
                res.push_str(" ");
            }
            if hours > 0 {
                res.push_str(&format!("{:02}m", minutes));
            } else {
                res.push_str(&format!("{}m", minutes));
            }
        }
        if days < 1 && hours < 1 && minutes < 1 {
            res.push_str(&format!("{}s", seconds));
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_only() {
        let duration = Duration::seconds(10);
        assert_eq!(duration.as_human().to_string(), "10s");
    }

    #[test]
    fn minutes_only() {
        let duration = Duration::minutes(10);
        assert_eq!(duration.as_human().to_string(), "10m");
    }

    #[test]
    fn hours_only() {
        let duration = Duration::hours(10);
        assert_eq!(duration.as_human().to_string(), "10h");
    }

    #[test]
    fn days_only() {
        let duration = Duration::days(10);
        assert_eq!(duration.as_human().to_string(), "10d");
    }

    #[test]
    fn days_and_hours() {
        let duration = Duration::days(10) + Duration::hours(10);
        assert_eq!(duration.as_human().to_string(), "10d 10h");
    }

    #[test]
    fn day_hours_and_minutes() {
        let duration = Duration::days(2) + Duration::hours(10) + Duration::minutes(2);
        assert_eq!(duration.as_human().to_string(), "2d 10h");
    }
}
