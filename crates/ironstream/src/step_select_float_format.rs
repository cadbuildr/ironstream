// FILE: step_select_float_format.rs
// occt: StepSelect_FloatFormat

/// Controls floating-point format for STEP files
pub struct FloatFormat {
    zero_suppress: bool,
    main_format: String,
    format_in_range: String,
    range_min: f64,
    range_max: f64,
}

impl FloatFormat {
    /// Create a new FloatFormat with standard options
    pub fn new() -> Self {
        FloatFormat {
            zero_suppress: true,
            main_format: "%E".to_string(),
            format_in_range: "%f".to_string(),
            range_min: 0.001,
            range_max: 1000.0,
        }
    }

    /// Set FloatFormat to default value
    pub fn set_default(&mut self, _digits: i32) {
        self.zero_suppress = true;
        self.main_format = "%E".to_string();
        self.format_in_range = "%f".to_string();
        self.range_min = 0.001;
        self.range_max = 1000.0;
    }

    /// Set ZeroSuppress mode
    pub fn set_zero_suppress(&mut self, mode: bool) {
        self.zero_suppress = mode;
    }

    /// Set main format
    pub fn set_format(&mut self, format: &str) {
        self.main_format = format.to_string();
    }

    /// Set format for range with min and max bounds
    pub fn set_format_for_range(&mut self, format: &str, min: f64, max: f64) {
        if !format.is_empty() && min > 0.0 && min < max {
            self.format_in_range = format.to_string();
            self.range_min = min;
            self.range_max = max;
        }
    }

    /// Get all recorded parameters
    pub fn format(
        &self,
    ) -> (
        bool,
        String,
        bool,
        String,
        f64,
        f64,
    ) {
        let has_range = !self.format_in_range.is_empty();
        (
            self.zero_suppress,
            self.main_format.clone(),
            has_range,
            self.format_in_range.clone(),
            self.range_min,
            self.range_max,
        )
    }

    /// Perform the modification
    pub fn perform(&self) {
        // Perform formatting operation
    }

    /// Get the label description
    pub fn label(&self) -> String {
        let mut label = format!("Float Format [ZeroSuppress: {}]", self.zero_suppress);
        label.push_str(&format!(" {} ", self.main_format));
        if !self.format_in_range.is_empty() {
            label.push_str(&format!(
                ", in range {}-{} {}",
                self.range_min, self.range_max, self.format_in_range
            ));
        }
        label
    }
}

impl Default for FloatFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let fmt = FloatFormat::new();
        assert!(fmt.zero_suppress);
        assert_eq!(fmt.main_format, "%E");
        assert_eq!(fmt.range_min, 0.001);
        assert_eq!(fmt.range_max, 1000.0);
    }

    #[test]
    fn test_set_zero_suppress() {
        let mut fmt = FloatFormat::new();
        fmt.set_zero_suppress(false);
        assert!(!fmt.zero_suppress);
    }

    #[test]
    fn test_set_format() {
        let mut fmt = FloatFormat::new();
        fmt.set_format("%f");
        assert_eq!(fmt.main_format, "%f");
    }

    #[test]
    fn test_set_format_for_range() {
        let mut fmt = FloatFormat::new();
        fmt.set_format_for_range("%g", 0.1, 100.0);
        assert_eq!(fmt.format_in_range, "%g");
        assert_eq!(fmt.range_min, 0.1);
        assert_eq!(fmt.range_max, 100.0);
    }

    #[test]
    fn test_format_tuple() {
        let fmt = FloatFormat::new();
        let (zerosup, mainform, hasrange, _, _, _) = fmt.format();
        assert!(zerosup);
        assert_eq!(mainform, "%E");
        assert!(hasrange);
    }

    #[test]
    fn test_label() {
        let fmt = FloatFormat::new();
        let label = fmt.label();
        assert!(!label.is_empty());
        assert!(label.contains("Float Format"));
    }
}
