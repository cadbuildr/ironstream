// FILE: step_basic_ordinal_date.rs
// occt: StepBasic_OrdinalDate

/// Represents a STEP OrdinalDate entity with a year and day component.
#[derive(Clone, Debug)]
pub struct StepBasicOrdinalDate {
    year_component: i32,
    day_component: i32,
}

impl StepBasicOrdinalDate {
    /// Create a new empty StepBasicOrdinalDate.
    pub fn new() -> Self {
        StepBasicOrdinalDate {
            year_component: 0,
            day_component: 0,
        }
    }

    /// Initialize all fields.
    pub fn init(&mut self, year_component: i32, day_component: i32) {
        self.year_component = year_component;
        self.day_component = day_component;
    }

    /// Returns the year component.
    pub fn year_component(&self) -> i32 {
        self.year_component
    }

    /// Returns the day component.
    pub fn day_component(&self) -> i32 {
        self.day_component
    }

    /// Set the day component.
    pub fn set_day_component(&mut self, day_component: i32) {
        self.day_component = day_component;
    }
}

impl Default for StepBasicOrdinalDate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let date = StepBasicOrdinalDate::new();
        assert_eq!(date.year_component(), 0);
        assert_eq!(date.day_component(), 0);
    }

    #[test]
    fn test_init() {
        let mut date = StepBasicOrdinalDate::new();
        date.init(2025, 123);

        assert_eq!(date.year_component(), 2025);
        assert_eq!(date.day_component(), 123);
    }

    #[test]
    fn test_set_day_component() {
        let mut date = StepBasicOrdinalDate::new();
        date.init(2024, 50);
        date.set_day_component(200);

        assert_eq!(date.year_component(), 2024);
        assert_eq!(date.day_component(), 200);
    }
}
