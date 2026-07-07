// FILE: step_fea_curve_element_interval_linearly_varying.rs
// occt: StepFEA_CurveElementIntervalLinearlyVarying

/// Representation of STEP entity CurveElementIntervalLinearlyVarying.
#[derive(Clone)]
pub struct CurveElementIntervalLinearlyVarying {
    start_node: Option<String>,
    end_node: Option<String>,
    descriptor_start: Option<String>,
    descriptor_end: Option<String>,
}

impl CurveElementIntervalLinearlyVarying {
    pub fn new() -> Self {
        Self {
            start_node: None,
            end_node: None,
            descriptor_start: None,
            descriptor_end: None,
        }
    }

    pub fn init(
        &mut self,
        start: Option<String>,
        end: Option<String>,
        desc_start: Option<String>,
        desc_end: Option<String>,
    ) {
        self.start_node = start;
        self.end_node = end;
        self.descriptor_start = desc_start;
        self.descriptor_end = desc_end;
    }

    pub fn start_node(&self) -> Option<&str> {
        self.start_node.as_deref()
    }

    pub fn set_start_node(&mut self, n: Option<String>) {
        self.start_node = n;
    }

    pub fn end_node(&self) -> Option<&str> {
        self.end_node.as_deref()
    }

    pub fn set_end_node(&mut self, n: Option<String>) {
        self.end_node = n;
    }

    pub fn descriptor_start(&self) -> Option<&str> {
        self.descriptor_start.as_deref()
    }

    pub fn set_descriptor_start(&mut self, d: Option<String>) {
        self.descriptor_start = d;
    }

    pub fn descriptor_end(&self) -> Option<&str> {
        self.descriptor_end.as_deref()
    }

    pub fn set_descriptor_end(&mut self, d: Option<String>) {
        self.descriptor_end = d;
    }
}

impl Default for CurveElementIntervalLinearlyVarying {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let interval = CurveElementIntervalLinearlyVarying::new();
        assert!(interval.start_node().is_none());
        assert!(interval.descriptor_start().is_none());
    }

    #[test]
    fn test_init() {
        let mut interval = CurveElementIntervalLinearlyVarying::new();
        interval.init(
            Some("N1".to_string()),
            Some("N2".to_string()),
            Some("D1".to_string()),
            Some("D2".to_string()),
        );

        assert_eq!(interval.start_node(), Some("N1"));
        assert_eq!(interval.descriptor_start(), Some("D1"));
        assert_eq!(interval.descriptor_end(), Some("D2"));
    }
}
