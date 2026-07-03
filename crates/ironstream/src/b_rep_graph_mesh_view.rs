// FILE: b_rep_graph_mesh_view.rs
// occt: BRepGraph_MeshView

use std::collections::HashMap;

/// View for accessing mesh representations in the graph.
pub struct BRepGraphMeshView {
    graph_id: usize,
    meshes: HashMap<u32, Vec<f64>>,
    triangulations: HashMap<u32, Vec<u32>>,
}

impl BRepGraphMeshView {
    /// Create a new mesh view.
    pub fn new(graph_id: usize) -> Self {
        Self {
            graph_id,
            meshes: HashMap::new(),
            triangulations: HashMap::new(),
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Get mesh vertices.
    pub fn get_mesh(&self, id: u32) -> Option<&[f64]> {
        self.meshes.get(&id).map(|v| v.as_slice())
    }

    /// Add a mesh.
    pub fn add_mesh(&mut self, id: u32, vertices: Vec<f64>) {
        self.meshes.insert(id, vertices);
    }

    /// Get triangulation indices.
    pub fn get_triangulation(&self, id: u32) -> Option<&[u32]> {
        self.triangulations.get(&id).map(|v| v.as_slice())
    }

    /// Add triangulation indices.
    pub fn add_triangulation(&mut self, id: u32, indices: Vec<u32>) {
        self.triangulations.insert(id, indices);
    }

    /// Clear all meshes.
    pub fn clear(&mut self) {
        self.meshes.clear();
        self.triangulations.clear();
    }

    /// Get number of meshes.
    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let view = BRepGraphMeshView::new(1);
        assert_eq!(view.graph_id(), 1);
        assert_eq!(view.mesh_count(), 0);
    }

    #[test]
    fn test_add_mesh() {
        let mut view = BRepGraphMeshView::new(1);
        view.add_mesh(1, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

        assert_eq!(view.get_mesh(1), Some(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0][..]));
        assert_eq!(view.mesh_count(), 1);
    }

    #[test]
    fn test_add_triangulation() {
        let mut view = BRepGraphMeshView::new(1);
        view.add_triangulation(1, vec![0, 1, 2]);

        assert_eq!(view.get_triangulation(1), Some(&[0, 1, 2][..]));
    }

    #[test]
    fn test_clear() {
        let mut view = BRepGraphMeshView::new(1);
        view.add_mesh(1, vec![0.0, 0.0, 0.0]);
        view.add_triangulation(1, vec![0, 1, 2]);

        view.clear();
        assert_eq!(view.mesh_count(), 0);
        assert_eq!(view.get_triangulation(1), None);
    }
}
