// FILE: step_element_curve_edge.rs
// occt: StepElement_CurveEdge

pub struct CurveEdge {
    pub name: Option<String>,
    pub curve_ref: Option<String>,
}

impl CurveEdge {
    pub fn new() -> Self {
        CurveEdge {
            name: None,
            curve_ref: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_curve_ref(&mut self, curve_ref: String) {
        self.curve_ref = Some(curve_ref);
    }

    pub fn get_curve_ref(&self) -> Option<&str> {
        self.curve_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let edge = CurveEdge::new();
        assert!(edge.name.is_none());
        assert!(edge.curve_ref.is_none());
    }

    #[test]
    fn test_set_curve_ref() {
        let mut edge = CurveEdge::new();
        edge.set_curve_ref("curve1".to_string());
        assert_eq!(edge.get_curve_ref(), Some("curve1"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut edge = CurveEdge::new();
        edge.set_name("edge1".to_string());
        assert_eq!(edge.get_name(), Some("edge1"));
    }
}
