// FILE: step_basic_date.rs
// occt: StepBasic_Date

pub struct StepBasic_Date {
    year_component: i32,
}

impl StepBasic_Date {
    pub fn new() -> Self {
        StepBasic_Date {
            year_component: 0,
        }
    }

    pub fn init(&mut self, year_component: i32) {
        self.year_component = year_component;
    }

    pub fn set_year_component(&mut self, year_component: i32) {
        self.year_component = year_component;
    }

    pub fn year_component(&self) -> i32 {
        self.year_component
    }
}

impl Default for StepBasic_Date {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let date = StepBasic_Date::new();
        assert_eq!(date.year_component(), 0);
    }

    #[test]
    fn test_init() {
        let mut date = StepBasic_Date::new();
        date.init(2024);
        assert_eq!(date.year_component(), 2024);
    }
}
