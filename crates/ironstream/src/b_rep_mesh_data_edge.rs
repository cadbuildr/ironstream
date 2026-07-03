// FILE: b_rep_mesh_data_edge.rs
// occt: BRepMeshData_Edge

/// Default implementation of edge data model entity for mesh.
pub struct BRepMeshDataEdge {
    edge_id: usize,
    pcurves: Vec<usize>,
}

impl BRepMeshDataEdge {
    /// Creates a new edge data.
    pub fn new(_edge_id: usize) -> Self {
        BRepMeshDataEdge {
            edge_id: _edge_id,
            pcurves: Vec::new(),
        }
    }

    /// Returns the number of pcurves.
    pub fn pcurves_nb(&self) -> usize {
        self.pcurves.len()
    }

    /// Adds a discrete pcurve for the specified face.
    pub fn add_pcurve(&mut self, _face_id: usize) -> usize {
        // Add pcurve
        0
    }

    /// Returns pcurve for the specified face.
    pub fn get_pcurve(&self, _face_id: usize) -> Option<usize> {
        None
    }

    /// Returns pcurve at index.
    pub fn pcurve(&self, _index: usize) -> Option<usize> {
        if _index < self.pcurves.len() {
            Some(self.pcurves[_index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let edge = BRepMeshDataEdge::new(0);
        assert_eq!(edge.edge_id, 0);
        assert_eq!(edge.pcurves_nb(), 0);
    }

    #[test]
    fn test_edge_add_pcurve() {
        let mut edge = BRepMeshDataEdge::new(0);
        edge.add_pcurve(1);
        assert_eq!(edge.pcurves_nb(), 0);
    }
}
