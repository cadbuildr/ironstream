// FILE: step_basic_week_of_year_and_day_date.rs
// occt: StepBasic_WeekOfYearAndDayDate

//! A date represented by week of year and day of week.

/// Represents a date specified by year, week of year, and optionally day of week.
#[derive(Debug, Clone)]
pub struct StepBasicWeekOfYearAndDayDate {
    /// The year component
    year_component: i32,
    /// The week of year (1-53)
    week_component: i32,
    /// The day of week (1-7), optional
    day_component: i32,
    /// Whether day component is defined
    has_day_component: bool,
}

impl StepBasicWeekOfYearAndDayDate {
    /// Create a new WeekOfYearAndDayDate
    pub fn new() -> Self {
        Self {
            year_component: 0,
            week_component: 0,
            day_component: 0,
            has_day_component: false,
        }
    }

    /// Initialize with all components
    pub fn init(&mut self, year: i32, week: i32, has_day: bool, day: i32) {
        self.year_component = year;
        self.week_component = week;
        self.has_day_component = has_day;
        if has_day {
            self.day_component = day;
        }
    }

    /// Set the year component
    pub fn set_year_component(&mut self, year: i32) {
        self.year_component = year;
    }

    /// Get the year component
    pub fn year_component(&self) -> i32 {
        self.year_component
    }

    /// Set the week component
    pub fn set_week_component(&mut self, week: i32) {
        self.week_component = week;
    }

    /// Get the week component
    pub fn week_component(&self) -> i32 {
        self.week_component
    }

    /// Set the day component
    pub fn set_day_component(&mut self, day: i32) {
        self.day_component = day;
        self.has_day_component = true;
    }

    /// Unset the day component
    pub fn unset_day_component(&mut self) {
        self.day_component = 0;
        self.has_day_component = false;
    }

    /// Get the day component
    pub fn day_component(&self) -> i32 {
        self.day_component
    }

    /// Check if day component is defined
    pub fn has_day_component(&self) -> bool {
        self.has_day_component
    }
}

impl Default for StepBasicWeekOfYearAndDayDate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let date = StepBasicWeekOfYearAndDayDate::new();
        assert_eq!(date.year_component(), 0);
        assert_eq!(date.week_component(), 0);
        assert!(!date.has_day_component());
    }

    #[test]
    fn test_init_without_day() {
        let mut date = StepBasicWeekOfYearAndDayDate::new();
        date.init(2023, 25, false, 0);
        assert_eq!(date.year_component(), 2023);
        assert_eq!(date.week_component(), 25);
        assert!(!date.has_day_component());
    }

    #[test]
    fn test_init_with_day() {
        let mut date = StepBasicWeekOfYearAndDayDate::new();
        date.init(2023, 25, true, 3);
        assert_eq!(date.year_component(), 2023);
        assert_eq!(date.week_component(), 25);
        assert_eq!(date.day_component(), 3);
        assert!(date.has_day_component());
    }

    #[test]
    fn test_set_year() {
        let mut date = StepBasicWeekOfYearAndDayDate::new();
        date.set_year_component(2024);
        assert_eq!(date.year_component(), 2024);
    }

    #[test]
    fn test_set_week() {
        let mut date = StepBasicWeekOfYearAndDayDate::new();
        date.set_week_component(30);
        assert_eq!(date.week_component(), 30);
    }

    #[test]
    fn test_set_and_unset_day() {
        let mut date = StepBasicWeekOfYearAndDayDate::new();
        date.set_day_component(5);
        assert_eq!(date.day_component(), 5);
        assert!(date.has_day_component());
        date.unset_day_component();
        assert!(!date.has_day_component());
    }

    #[test]
    fn test_default() {
        let date = StepBasicWeekOfYearAndDayDate::default();
        assert_eq!(date.year_component(), 0);
    }
}
