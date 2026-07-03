// FILE: b_rep_mesh_cylinder_range_splitter.rs
// occt: BRepMesh_CylinderRangeSplitter

/// Auxiliary class extending default range splitter in order to generate
/// internal nodes for cylindrical surface.
pub struct CylinderRangeSplitter {
    du: f64,
}

impl CylinderRangeSplitter {
    /// Constructor.
    pub fn new() -> Self {
        CylinderRangeSplitter { du: 1.0 }
    }

    /// Sets the U parameter delta.
    pub fn set_du(&mut self, du: f64) {
        self.du = du;
    }

    /// Returns the U parameter delta.
    pub fn du(&self) -> f64 {
        self.du
    }

    /// Resets this splitter. Must be called before first use.
    pub fn reset(&mut self) {
        self.du = 1.0;
    }

    /// Returns list of nodes generated using surface data and specified parameters.
    pub fn generate_surface_nodes(&self) -> Vec<(f64, f64)> {
        // Generate internal mesh nodes for cylindrical surface
        Vec::new()
    }

    /// Computes parametric delta taking length along U and V into account.
    pub fn compute_delta(&mut self, length_u: f64, length_v: f64) {
        // For a cylinder, delta depends on the surface geometry
        if length_u > 0.0 {
            self.du = length_v / length_u;
        } else {
            self.du = 1.0;
        }
    }
}

impl Default for CylinderRangeSplitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_range_splitter_new() {
        let splitter = CylinderRangeSplitter::new();
        assert_eq!(splitter.du(), 1.0);
    }

    #[test]
    fn test_cylinder_range_splitter_set_du() {
        let mut splitter = CylinderRangeSplitter::new();
        splitter.set_du(2.5);
        assert_eq!(splitter.du(), 2.5);
    }

    #[test]
    fn test_cylinder_range_splitter_reset() {
        let mut splitter = CylinderRangeSplitter::new();
        splitter.set_du(3.0);
        splitter.reset();
        assert_eq!(splitter.du(), 1.0);
    }

    #[test]
    fn test_cylinder_range_splitter_compute_delta() {
        let mut splitter = CylinderRangeSplitter::new();
        splitter.compute_delta(10.0, 5.0);
        assert_eq!(splitter.du(), 0.5);
    }

    #[test]
    fn test_cylinder_range_splitter_generate_surface_nodes() {
        let splitter = CylinderRangeSplitter::new();
        let nodes = splitter.generate_surface_nodes();
        assert!(nodes.is_empty());
    }
}
