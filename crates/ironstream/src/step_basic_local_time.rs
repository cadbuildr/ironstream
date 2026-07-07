// FILE: step_basic_local_time.rs
// occt: StepBasic_LocalTime

/// Representation of STEP entity LocalTime
#[derive(Clone, Debug)]
pub struct LocalTime {
    hour_component: i32,
    minute_component: Option<i32>,
    second_component: Option<f64>,
    zone: Option<String>,
}

impl LocalTime {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            hour_component: 0,
            minute_component: None,
            second_component: None,
            zone: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        hour_component: i32,
        has_minute_component: bool,
        minute_component: i32,
        has_second_component: bool,
        second_component: f64,
        zone: String,
    ) {
        self.hour_component = hour_component;
        if has_minute_component {
            self.minute_component = Some(minute_component);
        }
        if has_second_component {
            self.second_component = Some(second_component);
        }
        self.zone = Some(zone);
    }

    /// Set hour component
    pub fn set_hour_component(&mut self, hour: i32) {
        self.hour_component = hour;
    }

    /// Get hour component
    pub fn hour_component(&self) -> i32 {
        self.hour_component
    }

    /// Set minute component
    pub fn set_minute_component(&mut self, minute: i32) {
        self.minute_component = Some(minute);
    }

    /// Unset minute component
    pub fn unset_minute_component(&mut self) {
        self.minute_component = None;
    }

    /// Get minute component
    pub fn minute_component(&self) -> Option<i32> {
        self.minute_component
    }

    /// Check if minute component is defined
    pub fn has_minute_component(&self) -> bool {
        self.minute_component.is_some()
    }

    /// Set second component
    pub fn set_second_component(&mut self, second: f64) {
        self.second_component = Some(second);
    }

    /// Unset second component
    pub fn unset_second_component(&mut self) {
        self.second_component = None;
    }

    /// Get second component
    pub fn second_component(&self) -> Option<f64> {
        self.second_component
    }

    /// Check if second component is defined
    pub fn has_second_component(&self) -> bool {
        self.second_component.is_some()
    }

    /// Set zone
    pub fn set_zone(&mut self, zone: String) {
        self.zone = Some(zone);
    }

    /// Get zone
    pub fn zone(&self) -> Option<&str> {
        self.zone.as_deref()
    }
}

impl Default for LocalTime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let time = LocalTime::new();
        assert_eq!(time.hour_component(), 0);
        assert!(time.minute_component().is_none());
        assert!(time.second_component().is_none());
        assert!(time.zone().is_none());
    }

    #[test]
    fn test_hour_component() {
        let mut time = LocalTime::new();
        time.set_hour_component(14);
        assert_eq!(time.hour_component(), 14);
    }

    #[test]
    fn test_minute_component() {
        let mut time = LocalTime::new();
        time.set_minute_component(30);
        assert!(time.has_minute_component());
        assert_eq!(time.minute_component(), Some(30));
        time.unset_minute_component();
        assert!(!time.has_minute_component());
    }

    #[test]
    fn test_second_component() {
        let mut time = LocalTime::new();
        time.set_second_component(45.5);
        assert!(time.has_second_component());
        assert_eq!(time.second_component(), Some(45.5));
        time.unset_second_component();
        assert!(!time.has_second_component());
    }

    #[test]
    fn test_zone() {
        let mut time = LocalTime::new();
        time.set_zone("UTC+1".to_string());
        assert_eq!(time.zone(), Some("UTC+1"));
    }

    #[test]
    fn test_init() {
        let mut time = LocalTime::new();
        time.init(14, true, 30, true, 45.5, "UTC+2".to_string());
        assert_eq!(time.hour_component(), 14);
        assert_eq!(time.minute_component(), Some(30));
        assert_eq!(time.second_component(), Some(45.5));
        assert_eq!(time.zone(), Some("UTC+2"));
    }
}
