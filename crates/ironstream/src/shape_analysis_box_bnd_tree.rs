// FILE: shape_analysis_box_bnd_tree.rs
// occt: ShapeAnalysis_BoxBndTree

/// Bounding box tree for shape analysis.
pub struct BoxBndTree {
    tolerance: f64,
}

impl BoxBndTree {
    /// Create a new BoxBndTree.
    pub fn new() -> Self {
        BoxBndTree {
            tolerance: 1e-7,
        }
    }

    /// Set tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Get tolerance.
    pub fn get_tolerance(&self) -> f64 {
        self.tolerance
    }
}

impl Default for BoxBndTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree = BoxBndTree::new();
        assert_eq!(tree.get_tolerance(), 1e-7);
    }

    #[test]
    fn test_set_tolerance() {
        let mut tree = BoxBndTree::new();
        tree.set_tolerance(0.01);
        assert_eq!(tree.get_tolerance(), 0.01);
    }
}
