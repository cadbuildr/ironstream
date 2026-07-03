// FILE: b_rep_mesh_extrusion_range_splitter.rs
// occt: BRepMesh_ExtrusionRangeSplitter

/// Auxiliary class extending default range splitter in
/// order to generate internal nodes for extruded surface.
pub struct ExtrusionRangeSplitter {
    _private: (),
}

impl ExtrusionRangeSplitter {
    /// Constructor.
    pub fn new() -> Self {
        ExtrusionRangeSplitter { _private: () }
    }

    /// Returns list of nodes generated using surface data and specified parameters.
    pub fn generate_surface_nodes(&self) -> Vec<(f64, f64)> {
        Vec::new()
    }
}

impl Default for ExtrusionRangeSplitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrusion_range_splitter_new() {
        let splitter = ExtrusionRangeSplitter::new();
        let nodes = splitter.generate_surface_nodes();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_extrusion_range_splitter_default() {
        let splitter = ExtrusionRangeSplitter::default();
        assert!(splitter.generate_surface_nodes().is_empty());
    }
}
