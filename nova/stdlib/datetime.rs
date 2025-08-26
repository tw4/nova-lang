// Nova Standard Library - DateTime Module

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fmt;

/// DateTime structure for Nova
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NovaDateTime {
    timestamp: u64, // Unix timestamp in milliseconds
}

impl NovaDateTime {
    /// Create a new DateTime representing the current time
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        
        Self {
            timestamp: now.as_millis() as u64,
        }
    }

    /// Create a DateTime from a Unix timestamp (seconds)
    pub fn from_timestamp(seconds: u64) -> Self {
        Self {
            timestamp: seconds * 1000,
        }
    }

    /// Create a DateTime from a Unix timestamp in milliseconds
    pub fn from_timestamp_millis(millis: u64) -> Self {
        Self {
            timestamp: millis,
        }
    }

    /// Create a DateTime from year, month, day, hour, minute, second
    pub fn from_ymd_hms(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Option<Self> {
        // Basic validation
        if month == 0 || month > 12 || day == 0 || day > 31 || hour > 23 || minute > 59 || second > 59 {
            return None;
        }

        // Simple approximation - not accounting for leap years, etc.
        // In a real implementation, you'd use a proper datetime library
        let days_from_epoch = Self::days_from_civil(year, month as i32, day as i32);
        let seconds_from_epoch = days_from_epoch * 86400 + (hour * 3600 + minute * 60 + second) as i64;
        
        if seconds_from_epoch < 0 {
            return None;
        }

        Some(Self {
            timestamp: (seconds_from_epoch * 1000) as u64,
        })
    }

    /// Get Unix timestamp in seconds
    pub fn timestamp(&self) -> u64 {
        self.timestamp / 1000
    }

    /// Get Unix timestamp in milliseconds
    pub fn timestamp_millis(&self) -> u64 {
        self.timestamp
    }

    /// Get year
    pub fn year(&self) -> i32 {
        let (y, _, _) = self.to_civil();
        y
    }

    /// Get month (1-12)
    pub fn month(&self) -> u32 {
        let (_, m, _) = self.to_civil();
        m as u32
    }

    /// Get day of month (1-31)
    pub fn day(&self) -> u32 {
        let (_, _, d) = self.to_civil();
        d as u32
    }

    /// Get hour (0-23)
    pub fn hour(&self) -> u32 {
        let seconds_in_day = (self.timestamp / 1000) % 86400;
        (seconds_in_day / 3600) as u32
    }

    /// Get minute (0-59)
    pub fn minute(&self) -> u32 {
        let seconds_in_day = (self.timestamp / 1000) % 86400;
        ((seconds_in_day % 3600) / 60) as u32
    }

    /// Get second (0-59)
    pub fn second(&self) -> u32 {
        ((self.timestamp / 1000) % 60) as u32
    }

    /// Get milliseconds (0-999)
    pub fn millisecond(&self) -> u32 {
        (self.timestamp % 1000) as u32
    }

    /// Get day of week (0=Sunday, 1=Monday, ..., 6=Saturday)
    pub fn weekday(&self) -> u32 {
        let days_since_epoch = (self.timestamp / 1000) / 86400;
        ((days_since_epoch + 4) % 7) as u32 // Unix epoch was a Thursday
    }

    /// Add seconds to the datetime
    pub fn add_seconds(&self, seconds: i64) -> Self {
        let new_timestamp = if seconds >= 0 {
            self.timestamp + (seconds as u64 * 1000)
        } else {
            self.timestamp.saturating_sub((-seconds) as u64 * 1000)
        };

        Self {
            timestamp: new_timestamp,
        }
    }

    /// Add minutes to the datetime
    pub fn add_minutes(&self, minutes: i64) -> Self {
        self.add_seconds(minutes * 60)
    }

    /// Add hours to the datetime
    pub fn add_hours(&self, hours: i64) -> Self {
        self.add_seconds(hours * 3600)
    }

    /// Add days to the datetime
    pub fn add_days(&self, days: i64) -> Self {
        self.add_seconds(days * 86400)
    }

    /// Get difference in seconds between two datetimes
    pub fn diff_seconds(&self, other: &NovaDateTime) -> i64 {
        if self.timestamp >= other.timestamp {
            ((self.timestamp - other.timestamp) / 1000) as i64
        } else {
            -(((other.timestamp - self.timestamp) / 1000) as i64)
        }
    }

    /// Format the datetime as ISO 8601 string
    pub fn to_iso_string(&self) -> String {
        let (year, month, day) = self.to_civil();
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();
        let millis = self.millisecond();

        format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z", 
                year, month, day, hour, minute, second, millis)
    }

    /// Format the datetime as a readable string
    pub fn to_string(&self) -> String {
        let (year, month, day) = self.to_civil();
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();

        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", 
                year, month, day, hour, minute, second)
    }

    /// Check if this year is a leap year
    pub fn is_leap_year(&self) -> bool {
        let year = self.year();
        Self::is_leap_year_value(year)
    }

    /// Check if a given year is a leap year
    pub fn is_leap_year_value(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    // Helper function to convert days since epoch to civil date
    fn to_civil(&self) -> (i32, i32, i32) {
        let days_since_epoch = (self.timestamp / 1000 / 86400) as i32;
        Self::civil_from_days(days_since_epoch)
    }

    // Convert days since Unix epoch to civil date (year, month, day)
    fn civil_from_days(days: i32) -> (i32, i32, i32) {
        // This is a simplified algorithm - in practice you'd use a more robust one
        let mut year = 1970;
        let mut remaining_days = days;

        // Handle years
        while remaining_days >= 365 {
            if Self::is_leap_year_value(year) {
                if remaining_days >= 366 {
                    remaining_days -= 366;
                    year += 1;
                } else {
                    break;
                }
            } else {
                remaining_days -= 365;
                year += 1;
            }
        }

        // Handle months
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 1;
        
        for &days_in_this_month in &days_in_month {
            let actual_days = if month == 2 && Self::is_leap_year_value(year) {
                29
            } else {
                days_in_this_month
            };

            if remaining_days >= actual_days {
                remaining_days -= actual_days;
                month += 1;
            } else {
                break;
            }
        }

        let day = remaining_days + 1;
        (year, month, day)
    }

    // Convert civil date to days since Unix epoch
    fn days_from_civil(year: i32, month: i32, day: i32) -> i64 {
        // Simplified calculation
        let mut total_days = 0i64;

        // Add days for years since 1970
        for y in 1970..year {
            total_days += if Self::is_leap_year_value(y) { 366 } else { 365 };
        }

        // Add days for months in current year
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for m in 1..month {
            let actual_days = if m == 2 && Self::is_leap_year_value(year) {
                29
            } else {
                days_in_month[(m - 1) as usize]
            };
            total_days += actual_days as i64;
        }

        // Add days in current month
        total_days += (day - 1) as i64;

        total_days
    }
}

impl fmt::Display for NovaDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Duration structure for Nova
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NovaDuration {
    millis: u64,
}

impl NovaDuration {
    /// Create duration from milliseconds
    pub fn from_millis(millis: u64) -> Self {
        Self { millis }
    }

    /// Create duration from seconds
    pub fn from_seconds(seconds: u64) -> Self {
        Self { millis: seconds * 1000 }
    }

    /// Create duration from minutes
    pub fn from_minutes(minutes: u64) -> Self {
        Self { millis: minutes * 60 * 1000 }
    }

    /// Create duration from hours
    pub fn from_hours(hours: u64) -> Self {
        Self { millis: hours * 60 * 60 * 1000 }
    }

    /// Create duration from days
    pub fn from_days(days: u64) -> Self {
        Self { millis: days * 24 * 60 * 60 * 1000 }
    }

    /// Get total milliseconds
    pub fn total_millis(&self) -> u64 {
        self.millis
    }

    /// Get total seconds
    pub fn total_seconds(&self) -> u64 {
        self.millis / 1000
    }

    /// Get total minutes
    pub fn total_minutes(&self) -> u64 {
        self.millis / (60 * 1000)
    }

    /// Get total hours
    pub fn total_hours(&self) -> u64 {
        self.millis / (60 * 60 * 1000)
    }

    /// Get total days
    pub fn total_days(&self) -> u64 {
        self.millis / (24 * 60 * 60 * 1000)
    }

    /// Add two durations
    pub fn add(&self, other: &NovaDuration) -> NovaDuration {
        NovaDuration {
            millis: self.millis + other.millis,
        }
    }

    /// Subtract two durations
    pub fn subtract(&self, other: &NovaDuration) -> NovaDuration {
        NovaDuration {
            millis: self.millis.saturating_sub(other.millis),
        }
    }

    /// Multiply duration by a factor
    pub fn multiply(&self, factor: f64) -> NovaDuration {
        NovaDuration {
            millis: (self.millis as f64 * factor) as u64,
        }
    }

    /// Divide duration by a factor
    pub fn divide(&self, factor: f64) -> NovaDuration {
        if factor == 0.0 {
            return NovaDuration { millis: 0 };
        }
        NovaDuration {
            millis: (self.millis as f64 / factor) as u64,
        }
    }
}

impl fmt::Display for NovaDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let days = self.total_days();
        let hours = (self.millis / (60 * 60 * 1000)) % 24;
        let minutes = (self.millis / (60 * 1000)) % 60;
        let seconds = (self.millis / 1000) % 60;
        let millis = self.millis % 1000;

        if days > 0 {
            write!(f, "{}d {}h {}m {}s {}ms", days, hours, minutes, seconds, millis)
        } else if hours > 0 {
            write!(f, "{}h {}m {}s {}ms", hours, minutes, seconds, millis)
        } else if minutes > 0 {
            write!(f, "{}m {}s {}ms", minutes, seconds, millis)
        } else if seconds > 0 {
            write!(f, "{}s {}ms", seconds, millis)
        } else {
            write!(f, "{}ms", millis)
        }
    }
}

/// DateTime utilities
pub struct DateTime;

impl DateTime {
    /// Get current datetime
    pub fn now() -> NovaDateTime {
        NovaDateTime::now()
    }

    /// Parse ISO 8601 string to datetime
    pub fn parse_iso(iso_string: &str) -> Option<NovaDateTime> {
        // Simplified parsing - in practice you'd use a proper parser
        if iso_string.len() < 19 {
            return None;
        }

        let date_part = &iso_string[0..10];
        let time_part = &iso_string[11..19];

        let date_parts: Vec<&str> = date_part.split('-').collect();
        let time_parts: Vec<&str> = time_part.split(':').collect();

        if date_parts.len() != 3 || time_parts.len() != 3 {
            return None;
        }

        let year = date_parts[0].parse::<i32>().ok()?;
        let month = date_parts[1].parse::<u32>().ok()?;
        let day = date_parts[2].parse::<u32>().ok()?;
        let hour = time_parts[0].parse::<u32>().ok()?;
        let minute = time_parts[1].parse::<u32>().ok()?;
        let second = time_parts[2].parse::<u32>().ok()?;

        NovaDateTime::from_ymd_hms(year, month, day, hour, minute, second)
    }

    /// Get day names
    pub fn day_names() -> Vec<&'static str> {
        vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]
    }

    /// Get month names
    pub fn month_names() -> Vec<&'static str> {
        vec![
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ]
    }

    /// Get current timestamp in seconds
    pub fn timestamp() -> u64 {
        Self::now().timestamp()
    }

    /// Get current timestamp in milliseconds
    pub fn timestamp_millis() -> u64 {
        Self::now().timestamp_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_creation() {
        let dt = NovaDateTime::from_ymd_hms(2024, 1, 1, 12, 30, 45).unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_datetime_operations() {
        let dt = NovaDateTime::from_ymd_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let dt_plus_hour = dt.add_hours(1);
        assert_eq!(dt_plus_hour.hour(), 1);
        assert_eq!(dt.diff_seconds(&dt_plus_hour), -3600);
    }

    #[test]
    fn test_duration_operations() {
        let d1 = NovaDuration::from_seconds(60);
        let d2 = NovaDuration::from_minutes(1);
        assert_eq!(d1.total_seconds(), d2.total_seconds());

        let d3 = d1.add(&d2);
        assert_eq!(d3.total_seconds(), 120);
    }

    #[test]
    fn test_leap_year() {
        assert!(NovaDateTime::is_leap_year_value(2000));
        assert!(NovaDateTime::is_leap_year_value(2004));
        assert!(!NovaDateTime::is_leap_year_value(1900));
        assert!(!NovaDateTime::is_leap_year_value(2001));
    }

    #[test]
    fn test_iso_string() {
        let dt = NovaDateTime::from_ymd_hms(2024, 3, 15, 14, 30, 45).unwrap();
        let iso = dt.to_iso_string();
        assert!(iso.starts_with("2024-03-15T14:30:45"));
    }
}