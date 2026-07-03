// FILE: b_rep_mesh_data_structure_of_delaun.rs
// occt: BRepMesh_DataStructureOfDelaun

/// Data structure for Delaunay triangulation.
pub struct DataStructureOfDelaun {
    nodes: Vec<(f64, f64, f64)>,
    elements: Vec<(usize, usize, usize)>,
}

impl DataStructureOfDelaun {
    /// Constructor.
    pub fn new() -> Self {
        DataStructureOfDelaun {
            nodes: Vec::new(),
            elements: Vec::new(),
        }
    }

    /// Adds a node to the structure.
    pub fn add_node(&mut self, x: f64, y: f64, z: f64) -> usize {
        self.nodes.push((x, y, z));
        self.nodes.len() - 1
    }

    /// Adds a triangle element.
    pub fn add_element(&mut self, n1: usize, n2: usize, n3: usize) {
        self.elements.push((n1, n2, n3));
    }

    /// Returns number of nodes.
    pub fn nodes_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns number of elements.
    pub fn elements_count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for DataStructureOfDelaun {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_structure_of_delaun_new() {
        let ds = DataStructureOfDelaun::new();
        assert_eq!(ds.nodes_count(), 0);
        assert_eq!(ds.elements_count(), 0);
    }

    #[test]
    fn test_data_structure_of_delaun_add_node() {
        let mut ds = DataStructureOfDelaun::new();
        let idx = ds.add_node(1.0, 2.0, 3.0);
        assert_eq!(idx, 0);
        assert_eq!(ds.nodes_count(), 1);
    }

    #[test]
    fn test_data_structure_of_delaun_add_element() {
        let mut ds = DataStructureOfDelaun::new();
        ds.add_node(0.0, 0.0, 0.0);
        ds.add_node(1.0, 0.0, 0.0);
        ds.add_node(0.0, 1.0, 0.0);
        ds.add_element(0, 1, 2);
        assert_eq!(ds.elements_count(), 1);
    }
}
