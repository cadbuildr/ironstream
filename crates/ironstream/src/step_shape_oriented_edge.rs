// FILE: step_shape_oriented_edge.rs
// occt: StepShape_OrientedEdge

//! Representation of STEP entity OrientedEdge

#[derive(Clone, Debug)]
pub struct OrientedEdge {
    name: String,
    edge_element: Option<String>,
    orientation: bool,
    edge_start: Option<String>,
    edge_end: Option<String>,
}

impl OrientedEdge {
    /// Returns an OrientedEdge
    pub fn new() -> Self {
        OrientedEdge {
            name: String::new(),
            edge_element: None,
            orientation: false,
            edge_start: None,
            edge_end: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        edge_element: Option<String>,
        orientation: bool,
    ) {
        self.name = name;
        self.edge_element = edge_element;
        self.orientation = orientation;
    }

    /// Set EdgeElement
    pub fn set_edge_element(&mut self, element: Option<String>) {
        self.edge_element = element;
    }

    /// Returns EdgeElement
    pub fn edge_element(&self) -> &Option<String> {
        &self.edge_element
    }

    /// Set Orientation
    pub fn set_orientation(&mut self, orientation: bool) {
        self.orientation = orientation;
    }

    /// Returns Orientation
    pub fn orientation(&self) -> bool {
        self.orientation
    }

    /// Set EdgeStart (override from parent)
    pub fn set_edge_start(&mut self, start: Option<String>) {
        self.edge_start = start;
    }

    /// Returns EdgeStart (override from parent)
    pub fn edge_start(&self) -> &Option<String> {
        &self.edge_start
    }

    /// Set EdgeEnd (override from parent)
    pub fn set_edge_end(&mut self, end: Option<String>) {
        self.edge_end = end;
    }

    /// Returns EdgeEnd (override from parent)
    pub fn edge_end(&self) -> &Option<String> {
        &self.edge_end
    }

    /// Returns name field (inherited)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field (inherited)
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for OrientedEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let edge = OrientedEdge::new();
        assert_eq!(edge.name(), "");
        assert!(!edge.orientation());
        assert!(edge.edge_element().is_none());
    }

    #[test]
    fn test_init() {
        let mut edge = OrientedEdge::new();
        edge.init("Edge1".to_string(), Some("edge_elem1".to_string()), true);
        assert_eq!(edge.name(), "Edge1");
        assert!(edge.orientation());
    }

    #[test]
    fn test_set_orientation() {
        let mut edge = OrientedEdge::new();
        edge.set_orientation(true);
        assert!(edge.orientation());
    }

    #[test]
    fn test_edge_vertices() {
        let mut edge = OrientedEdge::new();
        edge.set_edge_start(Some("v1".to_string()));
        edge.set_edge_end(Some("v2".to_string()));
        assert_eq!(edge.edge_start(), &Some("v1".to_string()));
        assert_eq!(edge.edge_end(), &Some("v2".to_string()));
    }
}
