// FILE: b_rep_mesh_data_wire.rs
// occt: BRepMeshData_Wire

/// Wire data for mesh operations.
pub struct BRepMeshDataWire {
    wire_id: usize,
    edges: Vec<usize>,
}

impl BRepMeshDataWire {
    /// Creates a new wire data.
    pub fn new(_wire_id: usize) -> Self {
        BRepMeshDataWire {
            wire_id: _wire_id,
            edges: Vec::new(),
        }
    }

    /// Adds an edge to the wire.
    pub fn add_edge(&mut self, _edge_id: usize) {
        self.edges.push(_edge_id);
    }

    /// Returns the number of edges.
    pub fn edges_nb(&self) -> usize {
        self.edges.len()
    }

    /// Gets the edge at index.
    pub fn edge(&self, _index: usize) -> Option<usize> {
        if _index < self.edges.len() {
            Some(self.edges[_index])
        } else {
            None
        }
    }

    /// Clears all edges.
    pub fn clear(&mut self) {
        self.edges.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_creation() {
        let wire = BRepMeshDataWire::new(0);
        assert_eq!(wire.wire_id, 0);
        assert_eq!(wire.edges_nb(), 0);
    }

    #[test]
    fn test_wire_add_edge() {
        let mut wire = BRepMeshDataWire::new(0);
        wire.add_edge(1);
        assert_eq!(wire.edges_nb(), 1);
        assert_eq!(wire.edge(0), Some(1));
    }

    #[test]
    fn test_wire_clear() {
        let mut wire = BRepMeshDataWire::new(0);
        wire.add_edge(1);
        wire.clear();
        assert_eq!(wire.edges_nb(), 0);
    }
}
