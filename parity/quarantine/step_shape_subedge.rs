// FILE: step_shape_subedge.rs
// occt: StepShape_Subedge

use std::sync::Arc;

/// Placeholder for StepShape_Vertex
pub struct Vertex {
    id: usize,
}

impl Vertex {
    pub fn new(id: usize) -> Self {
        Vertex { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepShape_Edge (base class)
pub struct Edge {
    name: Arc<str>,
    edge_start: Option<Arc<Vertex>>,
    edge_end: Option<Arc<Vertex>>,
}

impl Edge {
    pub fn new(name: Arc<str>) -> Self {
        Edge {
            name,
            edge_start: None,
            edge_end: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a subedge in STEP format.
/// Inherits from StepShape_Edge.
pub struct Subedge {
    name: Arc<str>,
    edge_start: Option<Arc<Vertex>>,
    edge_end: Option<Arc<Vertex>>,
    parent_edge: Option<Arc<Edge>>,
}

impl Subedge {
    /// Create a new Subedge
    pub fn new() -> Self {
        Subedge {
            name: Arc::from(""),
            edge_start: None,
            edge_end: None,
            parent_edge: None,
        }
    }

    /// Initialize with all fields (inherited and own)
    pub fn init(
        &mut self,
        name: Arc<str>,
        edge_start: Arc<Vertex>,
        edge_end: Arc<Vertex>,
        parent_edge: Arc<Edge>,
    ) {
        self.name = name;
        self.edge_start = Some(edge_start);
        self.edge_end = Some(edge_end);
        self.parent_edge = Some(parent_edge);
    }

    /// Get the parent edge
    pub fn parent_edge(&self) -> Option<&Arc<Edge>> {
        self.parent_edge.as_ref()
    }

    /// Set the parent edge
    pub fn set_parent_edge(&mut self, parent_edge: Arc<Edge>) {
        self.parent_edge = Some(parent_edge);
    }

    /// Get the name (from inherited fields)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Get the edge start vertex (from inherited fields)
    pub fn edge_start(&self) -> Option<&Arc<Vertex>> {
        self.edge_start.as_ref()
    }

    /// Set the edge start vertex
    pub fn set_edge_start(&mut self, vertex: Arc<Vertex>) {
        self.edge_start = Some(vertex);
    }

    /// Get the edge end vertex (from inherited fields)
    pub fn edge_end(&self) -> Option<&Arc<Vertex>> {
        self.edge_end.as_ref()
    }

    /// Set the edge end vertex
    pub fn set_edge_end(&mut self, vertex: Arc<Vertex>) {
        self.edge_end = Some(vertex);
    }
}

impl Default for Subedge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subedge_creation() {
        let se = Subedge::new();
        assert_eq!(se.name(), "");
        assert!(se.parent_edge().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut se = Subedge::new();
        let start = Arc::new(Vertex::new(1));
        let end = Arc::new(Vertex::new(2));
        let parent = Arc::new(Edge::new(Arc::from("parent_edge")));
        let name = Arc::from("subedge_1");

        se.init(name.clone(), start.clone(), end.clone(), parent.clone());

        assert_eq!(se.name(), "subedge_1");
        assert!(se.parent_edge().is_some());
        assert!(se.edge_start().is_some());
        assert!(se.edge_end().is_some());
    }

    #[test]
    fn test_set_parent_edge() {
        let mut se = Subedge::new();
        let parent = Arc::new(Edge::new(Arc::from("parent")));

        se.set_parent_edge(parent);

        assert!(se.parent_edge().is_some());
        assert_eq!(se.parent_edge().unwrap().name(), "parent");
    }

    #[test]
    fn test_set_edge_vertices() {
        let mut se = Subedge::new();
        let start = Arc::new(Vertex::new(10));
        let end = Arc::new(Vertex::new(20));

        se.set_edge_start(start);
        se.set_edge_end(end);

        assert!(se.edge_start().is_some());
        assert_eq!(se.edge_start().unwrap().id(), 10);
        assert!(se.edge_end().is_some());
        assert_eq!(se.edge_end().unwrap().id(), 20);
    }

    #[test]
    fn test_full_initialization() {
        let mut se = Subedge::new();
        se.set_name(Arc::from("full_subedge"));

        let start = Arc::new(Vertex::new(5));
        let end = Arc::new(Vertex::new(15));
        let parent = Arc::new(Edge::new(Arc::from("full_parent")));

        se.set_edge_start(start);
        se.set_edge_end(end);
        se.set_parent_edge(parent);

        assert_eq!(se.name(), "full_subedge");
        assert!(se.edge_start().is_some());
        assert!(se.edge_end().is_some());
        assert!(se.parent_edge().is_some());
    }
}
