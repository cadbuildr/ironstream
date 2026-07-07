// FILE: step_shape_path.rs
// occt: StepShape_Path

use std::sync::Arc;

/// Placeholder for StepShape_OrientedEdge
#[derive(Clone)]
pub struct OrientedEdge {
    id: usize,
}

/// Represents a path in STEP format.
/// Inherits from StepShape_TopologicalRepresentationItem.
pub struct Path {
    name: Arc<str>,
    edge_list: Vec<Arc<OrientedEdge>>,
}

impl Path {
    /// Create a new Path
    pub fn new() -> Self {
        Path {
            name: Arc::from(""),
            edge_list: Vec::new(),
        }
    }

    /// Initialize with name and edge list
    pub fn init(&mut self, name: Arc<str>, edge_list: Vec<Arc<OrientedEdge>>) {
        self.name = name;
        self.edge_list = edge_list;
    }

    /// Set the edge list
    pub fn set_edge_list(&mut self, edge_list: Vec<Arc<OrientedEdge>>) {
        self.edge_list = edge_list;
    }

    /// Get the edge list
    pub fn edge_list(&self) -> &[Arc<OrientedEdge>] {
        &self.edge_list
    }

    /// Get an edge by index (1-based as per OCCT convention)
    pub fn edge_list_value(&self, num: usize) -> Option<Arc<OrientedEdge>> {
        if num > 0 && num <= self.edge_list.len() {
            Some(self.edge_list[num - 1].clone())
        } else {
            None
        }
    }

    /// Get the number of edges
    pub fn nb_edge_list(&self) -> usize {
        self.edge_list.len()
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_creation() {
        let path = Path::new();
        assert_eq!(path.name(), "");
        assert_eq!(path.nb_edge_list(), 0);
    }

    #[test]
    fn test_init_method() {
        let mut path = Path::new();
        let edges = vec![
            Arc::new(OrientedEdge { id: 1 }),
            Arc::new(OrientedEdge { id: 2 }),
            Arc::new(OrientedEdge { id: 3 }),
        ];
        let name: Arc<str> = Arc::from("path_1");

        path.init(name.clone(), edges.clone());

        assert_eq!(path.name(), "path_1");
        assert_eq!(path.nb_edge_list(), 3);
    }

    #[test]
    fn test_set_edge_list() {
        let mut path = Path::new();
        let edges = vec![
            Arc::new(OrientedEdge { id: 1 }),
            Arc::new(OrientedEdge { id: 2 }),
        ];

        path.set_edge_list(edges);
        assert_eq!(path.nb_edge_list(), 2);
    }

    #[test]
    fn test_edge_list_value() {
        let mut path = Path::new();
        let edges = vec![
            Arc::new(OrientedEdge { id: 10 }),
            Arc::new(OrientedEdge { id: 20 }),
            Arc::new(OrientedEdge { id: 30 }),
        ];

        path.set_edge_list(edges);

        // 1-based indexing
        let edge1 = path.edge_list_value(1);
        assert!(edge1.is_some());
        assert_eq!(edge1.unwrap().id, 10);

        let edge2 = path.edge_list_value(2);
        assert!(edge2.is_some());
        assert_eq!(edge2.unwrap().id, 20);

        // Out of bounds
        let edge_out = path.edge_list_value(4);
        assert!(edge_out.is_none());
    }
}
