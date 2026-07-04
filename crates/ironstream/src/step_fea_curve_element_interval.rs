// FILE: step_fea_curve_element_interval.rs
// occt: StepFEA_CurveElementInterval

/// Base representation of STEP entity CurveElementInterval.
#[derive(Clone)]
pub struct CurveElementInterval {
    start_node: Option<String>,
    end_node: Option<String>,
}

impl CurveElementInterval {
    pub fn new() -> Self {
        Self {
            start_node: None,
            end_node: None,
        }
    }

    pub fn init(&mut self, start: Option<String>, end: Option<String>) {
        self.start_node = start;
        self.end_node = end;
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
}

impl Default for CurveElementInterval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let interval = CurveElementInterval::new();
        assert!(interval.start_node().is_none());
    }

    #[test]
    fn test_init() {
        let mut interval = CurveElementInterval::new();
        interval.init(Some("N1".to_string()), Some("N2".to_string()));

        assert_eq!(interval.start_node(), Some("N1"));
        assert_eq!(interval.end_node(), Some("N2"));
    }
}
