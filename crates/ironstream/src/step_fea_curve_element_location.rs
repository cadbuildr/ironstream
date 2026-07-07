// FILE: step_fea_curve_element_location.rs
// occt: StepFEA_CurveElementLocation

/// Representation of STEP entity CurveElementLocation.
#[derive(Clone)]
pub struct CurveElementLocation {
    node: Option<String>,
    parameter: f64,
}

impl CurveElementLocation {
    pub fn new() -> Self {
        Self {
            node: None,
            parameter: 0.0,
        }
    }

    pub fn init(&mut self, node: Option<String>, parameter: f64) {
        self.node = node;
        self.parameter = parameter;
    }

    pub fn node(&self) -> Option<&str> {
        self.node.as_deref()
    }

    pub fn set_node(&mut self, n: Option<String>) {
        self.node = n;
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn set_parameter(&mut self, p: f64) {
        self.parameter = p;
    }
}

impl Default for CurveElementLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let loc = CurveElementLocation::new();
        assert!(loc.node().is_none());
        assert_eq!(loc.parameter(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut loc = CurveElementLocation::new();
        loc.init(Some("N1".to_string()), 0.5);

        assert_eq!(loc.node(), Some("N1"));
        assert_eq!(loc.parameter(), 0.5);
    }

    #[test]
    fn test_setters() {
        let mut loc = CurveElementLocation::new();
        loc.set_node(Some("Node".to_string()));
        loc.set_parameter(0.75);

        assert_eq!(loc.parameter(), 0.75);
    }
}
