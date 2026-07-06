// FILE: step_shape_seam_edge.rs
// occt: StepShape_SeamEdge

use std::sync::Arc;

/// Placeholder for StepGeom_Pcurve
pub struct Pcurve {
    id: usize,
}

impl Pcurve {
    pub fn new(id: usize) -> Self {
        Pcurve { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepShape_Edge
pub struct Edge {
    id: usize,
}

impl Edge {
    pub fn new(id: usize) -> Self {
        Edge { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents a seam edge in STEP format.
/// Inherits from StepShape_OrientedEdge.
pub struct SeamEdge {
    name: Arc<str>,
    edge_element: Option<Arc<Edge>>,
    orientation: bool,
    pcurve_reference: Option<Arc<Pcurve>>,
}

impl SeamEdge {
    /// Create a new SeamEdge
    pub fn new() -> Self {
        SeamEdge {
            name: Arc::from(""),
            edge_element: None,
            orientation: false,
            pcurve_reference: None,
        }
    }

    /// Initialize with all fields (inherited and own)
    pub fn init(
        &mut self,
        name: Arc<str>,
        edge_element: Arc<Edge>,
        orientation: bool,
        pcurve_reference: Arc<Pcurve>,
    ) {
        self.name = name;
        self.edge_element = Some(edge_element);
        self.orientation = orientation;
        self.pcurve_reference = Some(pcurve_reference);
    }

    /// Get the pcurve reference
    pub fn pcurve_reference(&self) -> Option<&Arc<Pcurve>> {
        self.pcurve_reference.as_ref()
    }

    /// Set the pcurve reference
    pub fn set_pcurve_reference(&mut self, pcurve_reference: Arc<Pcurve>) {
        self.pcurve_reference = Some(pcurve_reference);
    }

    /// Get the name (from inherited fields)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Get the edge element (from inherited fields)
    pub fn edge_element(&self) -> Option<&Arc<Edge>> {
        self.edge_element.as_ref()
    }

    /// Set the edge element
    pub fn set_edge_element(&mut self, edge_element: Arc<Edge>) {
        self.edge_element = Some(edge_element);
    }

    /// Get the orientation (from inherited fields)
    pub fn orientation(&self) -> bool {
        self.orientation
    }

    /// Set the orientation
    pub fn set_orientation(&mut self, orientation: bool) {
        self.orientation = orientation;
    }
}

impl Default for SeamEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seam_edge_creation() {
        let se = SeamEdge::new();
        assert_eq!(se.name(), "");
        assert_eq!(se.orientation(), false);
        assert!(se.pcurve_reference().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut se = SeamEdge::new();
        let edge = Arc::new(Edge::new(1));
        let pcurve = Arc::new(Pcurve::new(10));
        let name: Arc<str> = Arc::from("seam_1");

        se.init(name.clone(), edge.clone(), true, pcurve.clone());

        assert_eq!(se.name(), "seam_1");
        assert_eq!(se.orientation(), true);
        assert!(se.pcurve_reference().is_some());
        assert_eq!(se.pcurve_reference().unwrap().id(), 10);
    }

    #[test]
    fn test_set_pcurve_reference() {
        let mut se = SeamEdge::new();
        let pcurve = Arc::new(Pcurve::new(42));

        se.set_pcurve_reference(pcurve);

        assert!(se.pcurve_reference().is_some());
        assert_eq!(se.pcurve_reference().unwrap().id(), 42);
    }

    #[test]
    fn test_set_edge_element() {
        let mut se = SeamEdge::new();
        let edge = Arc::new(Edge::new(5));

        se.set_edge_element(edge);

        assert!(se.edge_element().is_some());
        assert_eq!(se.edge_element().unwrap().id(), 5);
    }

    #[test]
    fn test_set_orientation() {
        let mut se = SeamEdge::new();
        se.set_orientation(true);

        assert_eq!(se.orientation(), true);
    }
}
