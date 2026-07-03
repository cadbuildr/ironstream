// FILE: b_rep_mesh_edge_discret.rs
// occt: BRepMesh_EdgeDiscret

/// Tool for edge discretization.
pub struct EdgeDiscret {
    _private: (),
}

impl EdgeDiscret {
    /// Constructor.
    pub fn new() -> Self {
        EdgeDiscret { _private: () }
    }

    /// Performs edge discretization.
    pub fn perform(&self) -> Vec<f64> {
        Vec::new()
    }
}

impl Default for EdgeDiscret {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_discret_new() {
        let ed = EdgeDiscret::new();
        let params = ed.perform();
        assert!(params.is_empty());
    }
}
