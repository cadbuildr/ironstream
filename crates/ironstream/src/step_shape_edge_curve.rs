// FILE: step_shape_edge_curve.rs
// occt: StepShape_EdgeCurve

//! Representation of STEP entity EdgeCurve

#[derive(Clone, Debug)]
pub struct EdgeCurve {
    name: String,
    edge_start: Option<String>,
    edge_end: Option<String>,
    edge_geometry: Option<String>, // Placeholder for Curve handle
    same_sense: bool,
}

impl EdgeCurve {
    /// Returns an EdgeCurve
    pub fn new() -> Self {
        EdgeCurve {
            name: String::new(),
            edge_start: None,
            edge_end: None,
            edge_geometry: None,
            same_sense: false,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        edge_start: Option<String>,
        edge_end: Option<String>,
        edge_geometry: Option<String>,
        same_sense: bool,
    ) {
        self.name = name;
        self.edge_start = edge_start;
        self.edge_end = edge_end;
        self.edge_geometry = edge_geometry;
        self.same_sense = same_sense;
    }

    /// Set EdgeGeometry
    pub fn set_edge_geometry(&mut self, geometry: Option<String>) {
        self.edge_geometry = geometry;
    }

    /// Returns EdgeGeometry
    pub fn edge_geometry(&self) -> &Option<String> {
        &self.edge_geometry
    }

    /// Set SameSense
    pub fn set_same_sense(&mut self, sense: bool) {
        self.same_sense = sense;
    }

    /// Returns SameSense
    pub fn same_sense(&self) -> bool {
        self.same_sense
    }

    /// Returns name field (inherited)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field (inherited)
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns EdgeStart (inherited)
    pub fn edge_start(&self) -> &Option<String> {
        &self.edge_start
    }

    /// Set EdgeStart (inherited)
    pub fn set_edge_start(&mut self, edge_start: Option<String>) {
        self.edge_start = edge_start;
    }

    /// Returns EdgeEnd (inherited)
    pub fn edge_end(&self) -> &Option<String> {
        &self.edge_end
    }

    /// Set EdgeEnd (inherited)
    pub fn set_edge_end(&mut self, edge_end: Option<String>) {
        self.edge_end = edge_end;
    }
}

impl Default for EdgeCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = EdgeCurve::new();
        assert_eq!(curve.name(), "");
        assert!(!curve.same_sense());
        assert!(curve.edge_geometry().is_none());
    }

    #[test]
    fn test_init() {
        let mut curve = EdgeCurve::new();
        curve.init(
            "EdgeCurve1".to_string(),
            Some("v1".to_string()),
            Some("v2".to_string()),
            Some("geom1".to_string()),
            true,
        );
        assert_eq!(curve.name(), "EdgeCurve1");
        assert!(curve.same_sense());
        assert_eq!(curve.edge_geometry(), &Some("geom1".to_string()));
    }

    #[test]
    fn test_set_same_sense() {
        let mut curve = EdgeCurve::new();
        curve.set_same_sense(true);
        assert!(curve.same_sense());
    }

    #[test]
    fn test_inherited_properties() {
        let mut curve = EdgeCurve::new();
        curve.set_edge_start(Some("start".to_string()));
        curve.set_edge_end(Some("end".to_string()));
        assert_eq!(curve.edge_start(), &Some("start".to_string()));
        assert_eq!(curve.edge_end(), &Some("end".to_string()));
    }
}
