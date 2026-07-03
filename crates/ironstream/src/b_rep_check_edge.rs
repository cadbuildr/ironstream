// FILE: b_rep_check_edge.rs
// occt: BRepCheck_Edge

/// Check edge result class inheriting from BRepCheck_Result
pub struct EdgeChecker {
    geometric_controls: bool,
    is_exact_method: bool,
}

impl EdgeChecker {
    /// Create an EdgeChecker for a given edge
    pub fn new() -> Self {
        EdgeChecker {
            geometric_controls: false,
            is_exact_method: false,
        }
    }

    /// Get geometric controls flag
    pub fn geometric_controls(&self) -> bool {
        self.geometric_controls
    }

    /// Set geometric controls flag
    pub fn set_geometric_controls(&mut self, flag: bool) {
        self.geometric_controls = flag;
    }

    /// Get exact method flag
    pub fn is_exact_method(&self) -> bool {
        self.is_exact_method
    }

    /// Set exact method flag
    pub fn set_exact_method(&mut self, flag: bool) {
        self.is_exact_method = flag;
    }

    /// Get tolerance value
    pub fn tolerance(&self) -> f64 {
        1e-7
    }
}

impl Default for EdgeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_checker_creation() {
        let checker = EdgeChecker::new();
        assert!(!checker.geometric_controls());
        assert!(!checker.is_exact_method());
    }

    #[test]
    fn test_geometric_controls_flag() {
        let mut checker = EdgeChecker::new();
        assert!(!checker.geometric_controls());
        checker.set_geometric_controls(true);
        assert!(checker.geometric_controls());
    }

    #[test]
    fn test_exact_method_flag() {
        let mut checker = EdgeChecker::new();
        assert!(!checker.is_exact_method());
        checker.set_exact_method(true);
        assert!(checker.is_exact_method());
    }

    #[test]
    fn test_tolerance() {
        let checker = EdgeChecker::new();
        assert!(checker.tolerance() > 0.0);
    }
}
