// FILE: step_basic_calendar_date.rs
// occt: StepBasic_CalendarDate

pub struct StepBasic_CalendarDate {
    year_component: i32,
    day_component: i32,
    month_component: i32,
}

impl StepBasic_CalendarDate {
    pub fn new() -> Self {
        StepBasic_CalendarDate {
            year_component: 0,
            day_component: 0,
            month_component: 0,
        }
    }

    pub fn init(&mut self, year_component: i32, day_component: i32, month_component: i32) {
        self.year_component = year_component;
        self.day_component = day_component;
        self.month_component = month_component;
    }

    pub fn year_component(&self) -> i32 {
        self.year_component
    }

    pub fn set_day_component(&mut self, day_component: i32) {
        self.day_component = day_component;
    }

    pub fn day_component(&self) -> i32 {
        self.day_component
    }

    pub fn set_month_component(&mut self, month_component: i32) {
        self.month_component = month_component;
    }

    pub fn month_component(&self) -> i32 {
        self.month_component
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cal_date = StepBasic_CalendarDate::new();
        assert_eq!(cal_date.year_component(), 0);
        assert_eq!(cal_date.day_component(), 0);
        assert_eq!(cal_date.month_component(), 0);
    }

    #[test]
    fn test_init() {
        let mut cal_date = StepBasic_CalendarDate::new();
        cal_date.init(2024, 15, 7);
        assert_eq!(cal_date.year_component(), 2024);
        assert_eq!(cal_date.day_component(), 15);
        assert_eq!(cal_date.month_component(), 7);
    }
}
