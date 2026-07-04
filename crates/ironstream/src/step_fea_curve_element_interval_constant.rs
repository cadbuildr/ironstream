// FILE: step_fea_curve_element_interval_constant.rs
// occt: StepFEA_CurveElementIntervalConstant

/// Representation of STEP entity CurveElementIntervalConstant.
#[derive(Clone)]
pub struct CurveElementIntervalConstant {
    start_node: Option<String>,
    end_node: Option<String>,
    descriptor: Option<String>,
}

impl CurveElementIntervalConstant {
    pub fn new() -> Self {
        Self {
            start_node: None,
            end_node: None,
            descriptor: None,
        }
    }

    pub fn init(
        &mut self,
        start: Option<String>,
        end: Option<String>,
        descriptor: Option<String>,
    ) {
        self.start_node = start;
        self.end_node = end;
        self.descriptor = descriptor;
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

    pub fn descriptor(&self) -> Option<&str> {
        self.descriptor.as_deref()
    }

    pub fn set_descriptor(&mut self, d: Option<String>) {
        self.descriptor = d;
    }
}

impl Default for CurveElementIntervalConstant {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let interval = CurveElementIntervalConstant::new();
        assert!(interval.start_node().is_none());
        assert!(interval.descriptor().is_none());
    }

    #[test]
    fn test_init() {
        let mut interval = CurveElementIntervalConstant::new();
        interval.init(
            Some("N1".to_string()),
            Some("N2".to_string()),
            Some("DESC".to_string()),
        );

        assert_eq!(interval.start_node(), Some("N1"));
        assert_eq!(interval.descriptor(), Some("DESC"));
    }
}
