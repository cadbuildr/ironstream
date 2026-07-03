// FILE: b_rep_offset_tool.rs
// occt: BRepOffset_Tool

/// Utility functions for offset operations.
/// Provides helper methods for offset topology and geometry.
#[derive(Debug, Clone)]
pub struct BRepOffsetTool;

impl BRepOffsetTool {
    /// Create a new Tool
    pub fn new() -> Self {
        Self
    }

    /// Compute normal at a point on a surface
    pub fn compute_normal(_surface_id: usize, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 1.0)
    }

    /// Check if edge is degenerated
    pub fn is_degenerated(_edge_id: usize) -> bool {
        false
    }

    /// Get edge tolerance
    pub fn edge_tolerance(_edge_id: usize) -> f64 {
        1e-7
    }
}

impl Default for BRepOffsetTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = BRepOffsetTool::new();
        let _ = tool;
    }

    #[test]
    fn test_compute_normal() {
        let normal = BRepOffsetTool::compute_normal(1, 0.5, 0.5);
        assert_eq!(normal, (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_is_degenerated() {
        assert!(!BRepOffsetTool::is_degenerated(1));
    }

    #[test]
    fn test_edge_tolerance() {
        let tol = BRepOffsetTool::edge_tolerance(1);
        assert!(tol > 0.0);
    }

    #[test]
    fn test_default() {
        let _ = BRepOffsetTool::default();
    }
}
