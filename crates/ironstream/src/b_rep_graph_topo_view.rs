// FILE: b_rep_graph_topo_view.rs
// occt: BRepGraph_TopoView

use std::collections::HashMap;

/// View for accessing topology in the graph.
pub struct BRepGraphTopoView {
    graph_id: usize,
    vertices: HashMap<u32, (f64, f64, f64)>,
    edges: HashMap<u32, (u32, u32)>,
    faces: HashMap<u32, Vec<u32>>,
}

impl BRepGraphTopoView {
    /// Create a new topo view.
    pub fn new(graph_id: usize) -> Self {
        Self {
            graph_id,
            vertices: HashMap::new(),
            edges: HashMap::new(),
            faces: HashMap::new(),
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Add a vertex.
    pub fn add_vertex(&mut self, v_id: u32, x: f64, y: f64, z: f64) {
        self.vertices.insert(v_id, (x, y, z));
    }

    /// Get a vertex position.
    pub fn get_vertex(&self, v_id: u32) -> Option<(f64, f64, f64)> {
        self.vertices.get(&v_id).copied()
    }

    /// Add an edge connecting two vertices.
    pub fn add_edge(&mut self, e_id: u32, v1: u32, v2: u32) {
        self.edges.insert(e_id, (v1, v2));
    }

    /// Get an edge's endpoints.
    pub fn get_edge(&self, e_id: u32) -> Option<(u32, u32)> {
        self.edges.get(&e_id).copied()
    }

    /// Add a face with vertices.
    pub fn add_face(&mut self, f_id: u32, vertices: Vec<u32>) {
        self.faces.insert(f_id, vertices);
    }

    /// Get a face's vertices.
    pub fn get_face(&self, f_id: u32) -> Option<&[u32]> {
        self.faces.get(&f_id).map(|v| v.as_slice())
    }

    /// Get the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get the number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Get the number of faces.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Clear all topology.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.faces.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let view = BRepGraphTopoView::new(1);
        assert_eq!(view.graph_id(), 1);
        assert_eq!(view.vertex_count(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut view = BRepGraphTopoView::new(1);
        view.add_vertex(1, 0.0, 0.0, 0.0);

        assert_eq!(view.get_vertex(1), Some((0.0, 0.0, 0.0)));
        assert_eq!(view.vertex_count(), 1);
    }

    #[test]
    fn test_add_edge() {
        let mut view = BRepGraphTopoView::new(1);
        view.add_edge(1, 1, 2);

        assert_eq!(view.get_edge(1), Some((1, 2)));
        assert_eq!(view.edge_count(), 1);
    }

    #[test]
    fn test_add_face() {
        let mut view = BRepGraphTopoView::new(1);
        view.add_face(1, vec![1, 2, 3, 4]);

        assert_eq!(view.get_face(1), Some(&[1, 2, 3, 4][..]));
        assert_eq!(view.face_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut view = BRepGraphTopoView::new(1);
        view.add_vertex(1, 0.0, 0.0, 0.0);
        view.add_edge(1, 1, 2);
        view.add_face(1, vec![1, 2, 3]);

        view.clear();
        assert_eq!(view.vertex_count(), 0);
        assert_eq!(view.edge_count(), 0);
        assert_eq!(view.face_count(), 0);
    }
}
