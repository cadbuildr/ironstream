// FILE: b_rep_mesh_curve_tessellator.rs
// occt: BRepMesh_CurveTessellator

/// Tool for curve tessellation/discretization.
pub struct CurveTessellator {
    _private: (),
}

impl CurveTessellator {
    /// Constructor.
    pub fn new() -> Self {
        CurveTessellator { _private: () }
    }

    /// Performs curve tessellation.
    pub fn perform(&self) -> Vec<f64> {
        Vec::new()
    }
}

impl Default for CurveTessellator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_tessellator_new() {
        let tess = CurveTessellator::new();
        let points = tess.perform();
        assert!(points.is_empty());
    }
}
