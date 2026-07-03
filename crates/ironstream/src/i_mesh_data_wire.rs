// FILE: i_mesh_data_wire.rs
// occt: IMeshData_Wire

/// Interface class representing discrete model of a wire.
/// Wire represents an ordered set of edges.
pub struct IMeshDataWire {
    shape_data: String,
    deflection: f64,
    status: i32,
    edges: Vec<usize>,
    orientations: Vec<bool>,
}

impl IMeshDataWire {
    /// Create new wire with given shape data.
    pub fn new(shape_data: String) -> Self {
        IMeshDataWire {
            shape_data,
            deflection: f64::MAX,
            status: 0,
            edges: Vec::new(),
            orientations: Vec::new(),
        }
    }

    /// Get shape data.
    pub fn get_shape(&self) -> &str {
        &self.shape_data
    }

    /// Set shape data.
    pub fn set_shape(&mut self, shape_data: String) {
        self.shape_data = shape_data;
    }

    /// Get deflection value.
    pub fn get_deflection(&self) -> f64 {
        self.deflection
    }

    /// Set deflection value.
    pub fn set_deflection(&mut self, value: f64) {
        self.deflection = value;
    }

    /// Get status.
    pub fn get_status_mask(&self) -> i32 {
        self.status
    }

    /// Set status flag.
    pub fn set_status(&mut self, value: i32) {
        self.status |= value;
    }

    /// Check if status is set.
    pub fn is_status_set(&self, value: i32) -> bool {
        (self.status & value) != 0
    }

    /// Get number of edges.
    pub fn edges_nb(&self) -> usize {
        self.edges.len()
    }

    /// Add edge with orientation (forward/reverse).
    pub fn add_edge(&mut self, edge: usize, orientation: bool) -> usize {
        self.edges.push(edge);
        self.orientations.push(orientation);
        self.edges.len() - 1
    }

    /// Get edge at index.
    pub fn get_edge(&self, index: usize) -> Option<usize> {
        self.edges.get(index).copied()
    }

    /// Get edge orientation at index (true = forward, false = reverse).
    pub fn get_edge_orientation(&self, index: usize) -> Option<bool> {
        self.orientations.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let wire = IMeshDataWire::new("wire_shape".to_string());
        assert_eq!(wire.get_shape(), "wire_shape");
        assert_eq!(wire.edges_nb(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut wire = IMeshDataWire::new("wire".to_string());
        let idx1 = wire.add_edge(10, true);
        let idx2 = wire.add_edge(20, false);
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(wire.edges_nb(), 2);
    }

    #[test]
    fn test_get_edge() {
        let mut wire = IMeshDataWire::new("wire".to_string());
        wire.add_edge(100, true);
        assert_eq!(wire.get_edge(0), Some(100));
        assert_eq!(wire.get_edge_orientation(0), Some(true));
    }

    #[test]
    fn test_status() {
        let mut wire = IMeshDataWire::new("wire".to_string());
        wire.set_status(0x01);
        assert!(wire.is_status_set(0x01));
        assert_eq!(wire.get_status_mask(), 0x01);
    }
}
