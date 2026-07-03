// FILE: ch_fi2d_builder.rs
// occt: ChFi2d_Builder

use crate::ch_fi2d_construction_error::ChFi2dConstructionError;
use std::collections::HashMap;

/// Algorithm to build fillets on planar wire.
/// Constructs fillets or chamfers for linear and circular edges of a face.
#[derive(Debug, Clone)]
pub struct ChFi2dBuilder {
    /// Current status
    status: ChFi2dConstructionError,
    /// Fillet edges
    fillets: Vec<usize>,
    /// Chamfer edges
    chamfers: Vec<usize>,
    /// History of modifications (edge id -> new edge id)
    history: HashMap<usize, usize>,
}

impl ChFi2dBuilder {
    /// Create an empty builder
    pub fn new() -> Self {
        Self {
            status: ChFi2dConstructionError::Ready,
            fillets: Vec::new(),
            chamfers: Vec::new(),
            history: HashMap::new(),
        }
    }

    /// Initialize with a face
    pub fn init_face(&mut self, _face_id: usize) {
        self.status = ChFi2dConstructionError::Ready;
    }

    /// Initialize with a reference and modified face
    pub fn init_faces(&mut self, _ref_face_id: usize, _mod_face_id: usize) {
        self.status = ChFi2dConstructionError::Ready;
    }

    /// Get current status
    pub fn status(&self) -> ChFi2dConstructionError {
        self.status
    }

    /// Set status
    pub fn set_status(&mut self, status: ChFi2dConstructionError) {
        self.status = status;
    }

    /// Get number of fillets
    pub fn nb_fillet(&self) -> usize {
        self.fillets.len()
    }

    /// Get number of chamfers
    pub fn nb_chamfer(&self) -> usize {
        self.chamfers.len()
    }

    /// Get fillet edges
    pub fn fillet_edges(&self) -> &[usize] {
        &self.fillets
    }

    /// Get chamfer edges
    pub fn chamfer_edges(&self) -> &[usize] {
        &self.chamfers
    }

    /// Add a fillet edge
    pub fn add_fillet_edge(&mut self, edge_id: usize) {
        self.fillets.push(edge_id);
    }

    /// Add a chamfer edge
    pub fn add_chamfer_edge(&mut self, edge_id: usize) {
        self.chamfers.push(edge_id);
    }

    /// Record history entry
    pub fn record_history(&mut self, old_edge_id: usize, new_edge_id: usize) {
        self.history.insert(old_edge_id, new_edge_id);
    }

    /// Check if edge has descendant
    pub fn has_descendant(&self, edge_id: usize) -> bool {
        self.history.contains_key(&edge_id)
    }

    /// Get descendant edge
    pub fn descendant_edge(&self, edge_id: usize) -> Option<usize> {
        self.history.get(&edge_id).copied()
    }
}

impl Default for ChFi2dBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = ChFi2dBuilder::new();
        assert_eq!(builder.status(), ChFi2dConstructionError::Ready);
        assert_eq!(builder.nb_fillet(), 0);
        assert_eq!(builder.nb_chamfer(), 0);
    }

    #[test]
    fn test_add_fillet_edge() {
        let mut builder = ChFi2dBuilder::new();
        builder.add_fillet_edge(1);
        builder.add_fillet_edge(2);
        assert_eq!(builder.nb_fillet(), 2);
    }

    #[test]
    fn test_add_chamfer_edge() {
        let mut builder = ChFi2dBuilder::new();
        builder.add_chamfer_edge(10);
        assert_eq!(builder.nb_chamfer(), 1);
    }

    #[test]
    fn test_record_history() {
        let mut builder = ChFi2dBuilder::new();
        builder.record_history(5, 15);
        assert!(builder.has_descendant(5));
        assert_eq!(builder.descendant_edge(5), Some(15));
    }

    #[test]
    fn test_status() {
        let mut builder = ChFi2dBuilder::new();
        builder.set_status(ChFi2dConstructionError::IsDone);
        assert_eq!(builder.status(), ChFi2dConstructionError::IsDone);
    }

    #[test]
    fn test_default() {
        let builder = ChFi2dBuilder::default();
        assert_eq!(builder.nb_fillet(), 0);
    }
}
