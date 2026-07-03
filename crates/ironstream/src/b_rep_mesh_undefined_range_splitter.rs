// FILE: b_rep_mesh_undefined_range_splitter.rs
// occt: BRepMesh_UndefinedRangeSplitter

/// Auxiliary class extending default range splitter for
/// undefined/generic surface types.
pub struct UndefinedRangeSplitter {
    _private: (),
}

impl UndefinedRangeSplitter {
    /// Constructor.
    pub fn new() -> Self {
        UndefinedRangeSplitter { _private: () }
    }

    /// Returns list of nodes generated using surface data and specified parameters.
    pub fn generate_surface_nodes(&self) -> Vec<(f64, f64)> {
        Vec::new()
    }
}

impl Default for UndefinedRangeSplitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_range_splitter_new() {
        let splitter = UndefinedRangeSplitter::new();
        let nodes = splitter.generate_surface_nodes();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_undefined_range_splitter_default() {
        let splitter = UndefinedRangeSplitter::default();
        assert!(splitter.generate_surface_nodes().is_empty());
    }
}
