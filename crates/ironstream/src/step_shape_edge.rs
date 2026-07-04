// FILE: step_shape_edge.rs
// occt: StepShape_Edge

//! Representation of STEP entity Edge

#[derive(Clone, Debug)]
pub struct Edge {
    name: String,
    edge_start: Option<String>, // Placeholder for Vertex handle
    edge_end: Option<String>,   // Placeholder for Vertex handle
}

impl Edge {
    /// Returns an Edge
    pub fn new() -> Self {
        Edge {
            name: String::new(),
            edge_start: None,
            edge_end: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, edge_start: Option<String>, edge_end: Option<String>) {
        self.name = name;
        self.edge_start = edge_start;
        self.edge_end = edge_end;
    }

    /// Set EdgeStart
    pub fn set_edge_start(&mut self, edge_start: Option<String>) {
        self.edge_start = edge_start;
    }

    /// Returns EdgeStart
    pub fn edge_start(&self) -> &Option<String> {
        &self.edge_start
    }

    /// Set EdgeEnd
    pub fn set_edge_end(&mut self, edge_end: Option<String>) {
        self.edge_end = edge_end;
    }

    /// Returns EdgeEnd
    pub fn edge_end(&self) -> &Option<String> {
        &self.edge_end
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

impl Default for Edge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let edge = Edge::new();
        assert_eq!(edge.name(), "");
        assert!(edge.edge_start().is_none());
        assert!(edge.edge_end().is_none());
    }

    #[test]
    fn test_init() {
        let mut edge = Edge::new();
        edge.init(
            "Edge1".to_string(),
            Some("vertex1".to_string()),
            Some("vertex2".to_string()),
        );
        assert_eq!(edge.name(), "Edge1");
        assert_eq!(edge.edge_start(), &Some("vertex1".to_string()));
        assert_eq!(edge.edge_end(), &Some("vertex2".to_string()));
    }

    #[test]
    fn test_set_edge_start() {
        let mut edge = Edge::new();
        edge.set_edge_start(Some("v1".to_string()));
        assert_eq!(edge.edge_start(), &Some("v1".to_string()));
    }

    #[test]
    fn test_set_edge_end() {
        let mut edge = Edge::new();
        edge.set_edge_end(Some("v2".to_string()));
        assert_eq!(edge.edge_end(), &Some("v2".to_string()));
    }
}
