// FILE: step_shape_limits_and_fits.rs
// occt: StepShape_LimitsAndFits

//! Added for Dimensional Tolerances

#[derive(Clone, Debug)]
pub struct LimitsAndFits {
    form_variance: String,
    zone_variance: String,
    grade: String,
    source: String,
}

impl LimitsAndFits {
    /// Constructor
    pub fn new() -> Self {
        LimitsAndFits {
            form_variance: String::new(),
            zone_variance: String::new(),
            grade: String::new(),
            source: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        form_variance: String,
        zone_variance: String,
        grade: String,
        source: String,
    ) {
        self.form_variance = form_variance;
        self.zone_variance = zone_variance;
        self.grade = grade;
        self.source = source;
    }

    /// Returns FormVariance
    pub fn form_variance(&self) -> &str {
        &self.form_variance
    }

    /// Set FormVariance
    pub fn set_form_variance(&mut self, form_variance: String) {
        self.form_variance = form_variance;
    }

    /// Returns ZoneVariance
    pub fn zone_variance(&self) -> &str {
        &self.zone_variance
    }

    /// Set ZoneVariance
    pub fn set_zone_variance(&mut self, zone_variance: String) {
        self.zone_variance = zone_variance;
    }

    /// Returns Grade
    pub fn grade(&self) -> &str {
        &self.grade
    }

    /// Set Grade
    pub fn set_grade(&mut self, grade: String) {
        self.grade = grade;
    }

    /// Returns Source
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Set Source
    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }
}

impl Default for LimitsAndFits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let laf = LimitsAndFits::new();
        assert_eq!(laf.form_variance(), "");
        assert_eq!(laf.zone_variance(), "");
        assert_eq!(laf.grade(), "");
        assert_eq!(laf.source(), "");
    }

    #[test]
    fn test_init() {
        let mut laf = LimitsAndFits::new();
        laf.init(
            "H".to_string(),
            "X".to_string(),
            "5".to_string(),
            "ISO".to_string(),
        );
        assert_eq!(laf.form_variance(), "H");
        assert_eq!(laf.zone_variance(), "X");
        assert_eq!(laf.grade(), "5");
        assert_eq!(laf.source(), "ISO");
    }

    #[test]
    fn test_setters() {
        let mut laf = LimitsAndFits::new();
        laf.set_form_variance("P".to_string());
        laf.set_zone_variance("Y".to_string());
        laf.set_grade("6".to_string());
        laf.set_source("JIS".to_string());
        assert_eq!(laf.form_variance(), "P");
        assert_eq!(laf.zone_variance(), "Y");
        assert_eq!(laf.grade(), "6");
        assert_eq!(laf.source(), "JIS");
    }
}
