// FILE: step_shape_connected_edge_set.rs
// occt: StepShape_ConnectedEdgeSet

//! Representation of STEP entity ConnectedEdgeSet

#[derive(Clone, Debug)]
pub struct ConnectedEdgeSet {
    name: String,
    ces_edges: Vec<String>, // Placeholder for Edge handles
}

impl ConnectedEdgeSet {
    /// Empty constructor
    pub fn new() -> Self {
        ConnectedEdgeSet {
            name: String::new(),
            ces_edges: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, edges: Vec<String>) {
        self.name = name;
        self.ces_edges = edges;
    }

    /// Returns field CesEdges
    pub fn ces_edges(&self) -> &[String] {
        &self.ces_edges
    }

    /// Set field CesEdges
    pub fn set_ces_edges(&mut self, edges: Vec<String>) {
        self.ces_edges = edges;
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

impl Default for ConnectedEdgeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ces = ConnectedEdgeSet::new();
        assert_eq!(ces.name(), "");
        assert_eq!(ces.ces_edges().len(), 0);
    }

    #[test]
    fn test_init() {
        let mut ces = ConnectedEdgeSet::new();
        ces.init("TestSet".to_string(), vec!["edge1".to_string(), "edge2".to_string()]);
        assert_eq!(ces.name(), "TestSet");
        assert_eq!(ces.ces_edges().len(), 2);
    }

    #[test]
    fn test_set_ces_edges() {
        let mut ces = ConnectedEdgeSet::new();
        ces.set_ces_edges(vec!["e1".to_string(), "e2".to_string()]);
        assert_eq!(ces.ces_edges().len(), 2);
    }

    #[test]
    fn test_set_name() {
        let mut ces = ConnectedEdgeSet::new();
        ces.set_name("MyEdges".to_string());
        assert_eq!(ces.name(), "MyEdges");
    }
}
