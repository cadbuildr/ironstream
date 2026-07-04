// FILE: step_shape_manifold_solid_brep.rs
// occt: StepShape_ManifoldSolidBrep

//! Representation of STEP entity ManifoldSolidBrep

#[derive(Clone, Debug)]
pub struct ManifoldSolidBrep {
    name: String,
    outer: Option<String>, // Placeholder for ConnectedFaceSet handle
}

impl ManifoldSolidBrep {
    /// Returns a ManifoldSolidBrep
    pub fn new() -> Self {
        ManifoldSolidBrep {
            name: String::new(),
            outer: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, outer: Option<String>) {
        self.name = name;
        self.outer = outer;
    }

    /// Set Outer
    pub fn set_outer(&mut self, outer: Option<String>) {
        self.outer = outer;
    }

    /// Returns Outer
    pub fn outer(&self) -> &Option<String> {
        &self.outer
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for ManifoldSolidBrep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let msb = ManifoldSolidBrep::new();
        assert_eq!(msb.name(), "");
        assert!(msb.outer().is_none());
    }

    #[test]
    fn test_init() {
        let mut msb = ManifoldSolidBrep::new();
        msb.init("MSB1".to_string(), Some("shell1".to_string()));
        assert_eq!(msb.name(), "MSB1");
        assert_eq!(msb.outer(), &Some("shell1".to_string()));
    }

    #[test]
    fn test_set_outer() {
        let mut msb = ManifoldSolidBrep::new();
        msb.set_outer(Some("outer1".to_string()));
        assert_eq!(msb.outer(), &Some("outer1".to_string()));
    }
}
