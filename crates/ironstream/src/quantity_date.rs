// FILE: quantity_date.rs
// occt: Quantity_Date

/// This class provides services to manage date information.
/// A date represents the following time intervals:
/// year, month, day, hour, minute, second, millisecond and microsecond.
/// Current time is expressed in elapsed seconds and microseconds from 00:00 GMT, January 1, 1979.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuantityDate {
    // Seconds since 1979-01-01 00:00 GMT
    my_sec: i64,
    // Microseconds within the current second (0..999999)
    my_u_sec: i32,
}

impl QuantityDate {
    /// Constructs a default date (00:00 GMT, January 1, 1979).
    pub fn new() -> Self {
        Self {
            my_sec: 0,
            my_u_sec: 0,
        }
    }

    /// Constructs a date from components.
    /// month: 1-12, day: 1-31, year: >= 1979, hour: 0-23, minute: 0-59, second: 0-59,
    /// millisecond: 0-999, microsecond: 0-999
    pub fn from_components(
        month: i32,
        day: i32,
        year: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        microsecond: i32,
    ) -> Result<Self, &'static str> {
        if !Self::is_valid(month, day, year, hour, minute, second, millisecond, microsecond) {
            return Err("Invalid date components");
        }

        let total_sec = Self::compute_seconds(month, day, year, hour, minute, second);
        let total_u_sec = millisecond * 1000 + microsecond;

        Ok(Self {
            my_sec: total_sec,
            my_u_sec: total_u_sec,
        })
    }

    /// Gets the complete date.
    pub fn values(&self) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        let (year, month, day, hour, minute, second) = Self::decompose_seconds(self.my_sec);
        let millisecond = self.my_u_sec / 1000;
        let microsecond = self.my_u_sec % 1000;
        (month, day, year, hour, minute, second, millisecond, microsecond)
    }

    /// Assigns new values to this date.
    pub fn set_values(
        &mut self,
        month: i32,
        day: i32,
        year: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        microsecond: i32,
    ) -> Result<(), &'static str> {
        if !Self::is_valid(month, day, year, hour, minute, second, millisecond, microsecond) {
            return Err("Invalid date components");
        }
        let total_sec = Self::compute_seconds(month, day, year, hour, minute, second);
        let total_u_sec = millisecond * 1000 + microsecond;
        self.my_sec = total_sec;
        self.my_u_sec = total_u_sec;
        Ok(())
    }

    /// Returns the year of this date.
    pub fn year(&self) -> i32 {
        let (year, _, _, _, _, _) = Self::decompose_seconds(self.my_sec);
        year
    }

    /// Returns the month of this date (1-12).
    pub fn month(&self) -> i32 {
        let (_, month, _, _, _, _) = Self::decompose_seconds(self.my_sec);
        month
    }

    /// Returns the day of this date (1-31).
    pub fn day(&self) -> i32 {
        let (_, _, day, _, _, _) = Self::decompose_seconds(self.my_sec);
        day
    }

    /// Returns the hour of this date (0-23).
    pub fn hour(&self) -> i32 {
        let (_, _, _, hour, _, _) = Self::decompose_seconds(self.my_sec);
        hour
    }

    /// Returns the minute of this date (0-59).
    pub fn minute(&self) -> i32 {
        let (_, _, _, _, minute, _) = Self::decompose_seconds(self.my_sec);
        minute
    }

    /// Returns the second of this date (0-59).
    pub fn second(&self) -> i32 {
        let (_, _, _, _, _, second) = Self::decompose_seconds(self.my_sec);
        second
    }

    /// Returns the millisecond of this date (0-999).
    pub fn millisecond(&self) -> i32 {
        self.my_u_sec / 1000
    }

    /// Returns the microsecond of this date (0-999).
    pub fn microsecond(&self) -> i32 {
        self.my_u_sec % 1000
    }

    /// Returns TRUE if both dates are equal.
    pub fn is_equal(&self, other: &Self) -> bool {
        self.my_sec == other.my_sec && self.my_u_sec == other.my_u_sec
    }

    /// Returns TRUE if this date is earlier than other.
    pub fn is_earlier(&self, other: &Self) -> bool {
        self.my_sec < other.my_sec || (self.my_sec == other.my_sec && self.my_u_sec < other.my_u_sec)
    }

    /// Returns TRUE if this date is later than other.
    pub fn is_later(&self, other: &Self) -> bool {
        self.my_sec > other.my_sec || (self.my_sec == other.my_sec && self.my_u_sec > other.my_u_sec)
    }

    /// Checks the validity of a date.
    pub fn is_valid(
        month: i32,
        day: i32,
        year: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        microsecond: i32,
    ) -> bool {
        if year < 1979
            || month < 1
            || month > 12
            || hour < 0
            || hour > 23
            || minute < 0
            || minute > 59
            || second < 0
            || second > 59
            || millisecond < 0
            || millisecond > 999
            || microsecond < 0
            || microsecond > 999
        {
            return false;
        }

        let days_in_month = if Self::is_leap(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        day >= 1 && day <= days_in_month[(month - 1) as usize]
    }

    /// Returns TRUE if year is a leap year.
    pub fn is_leap(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    // Helper: compute seconds since 1979-01-01 00:00
    fn compute_seconds(month: i32, day: i32, year: i32, hour: i32, minute: i32, second: i32) -> i64 {
        let mut total = 0i64;

        // Days from 1979 to year-1
        for y in 1979..year {
            total += if Self::is_leap(y) { 366 } else { 365 };
        }

        // Days from Jan to month-1
        let days_in_month = if Self::is_leap(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        for m in 0..(month - 1) as usize {
            total += days_in_month[m] as i64;
        }

        // Add days
        total += (day - 1) as i64;

        // Convert to seconds
        total = total * 86400
            + (hour as i64) * 3600
            + (minute as i64) * 60
            + (second as i64);

        total
    }

    // Helper: decompose seconds since 1979-01-01 00:00
    fn decompose_seconds(mut secs: i64) -> (i32, i32, i32, i32, i32, i32) {
        let mut year = 1979;
        let mut days = secs / 86400;
        let seconds_of_day = secs % 86400;

        // Find year
        loop {
            let days_in_year = if Self::is_leap(year) { 366 } else { 365 };
            if days < days_in_year {
                break;
            }
            days -= days_in_year;
            year += 1;
        }

        // Find month and day
        let days_in_month = if Self::is_leap(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        let mut month = 1;
        for m in 0..12 {
            if days < days_in_month[m] as i64 {
                month = (m + 1) as i32;
                break;
            }
            days -= days_in_month[m] as i64;
        }

        let day = (days + 1) as i32;

        // Find hour, minute, second
        let hour = (seconds_of_day / 3600) as i32;
        let minute = ((seconds_of_day % 3600) / 60) as i32;
        let second = (seconds_of_day % 60) as i32;

        (year, month, day, hour, minute, second)
    }
}

impl Default for QuantityDate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let date = QuantityDate::new();
        assert_eq!(date.year(), 1979);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
        assert_eq!(date.hour(), 0);
        assert_eq!(date.minute(), 0);
        assert_eq!(date.second(), 0);
    }

    #[test]
    fn test_from_components() {
        let date = QuantityDate::from_components(6, 15, 2000, 12, 30, 45, 123, 456).unwrap();
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 15);
        assert_eq!(date.year(), 2000);
        assert_eq!(date.hour(), 12);
        assert_eq!(date.minute(), 30);
        assert_eq!(date.second(), 45);
        assert_eq!(date.millisecond(), 123);
        assert_eq!(date.microsecond(), 456);
    }

    #[test]
    fn test_is_valid() {
        assert!(QuantityDate::is_valid(6, 15, 2000, 12, 30, 45, 0, 0));
        assert!(!QuantityDate::is_valid(13, 15, 2000, 12, 30, 45, 0, 0)); // month 13
        assert!(!QuantityDate::is_valid(6, 32, 2000, 12, 30, 45, 0, 0)); // day 32
        assert!(!QuantityDate::is_valid(2, 30, 2000, 12, 30, 45, 0, 0)); // Feb 30
    }

    #[test]
    fn test_leap_year() {
        assert!(QuantityDate::is_leap(2000)); // divisible by 400
        assert!(!QuantityDate::is_leap(1900)); // divisible by 100 but not 400
        assert!(QuantityDate::is_leap(2004)); // divisible by 4
        assert!(!QuantityDate::is_leap(2001)); // not divisible by 4
    }

    #[test]
    fn test_equality() {
        let d1 = QuantityDate::from_components(1, 1, 1979, 0, 0, 0, 0, 0).unwrap();
        let d2 = QuantityDate::from_components(1, 1, 1979, 0, 0, 0, 0, 0).unwrap();
        assert!(d1.is_equal(&d2));
    }

    #[test]
    fn test_comparison() {
        let d1 = QuantityDate::from_components(1, 1, 1979, 0, 0, 0, 0, 0).unwrap();
        let d2 = QuantityDate::from_components(1, 2, 1979, 0, 0, 0, 0, 0).unwrap();
        assert!(d1.is_earlier(&d2));
        assert!(d2.is_later(&d1));
    }
}
