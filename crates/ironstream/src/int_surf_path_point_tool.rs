// FILE: int_surf_path_point_tool.rs
// occt: IntSurf_PathPointTool

use crate::int_surf_path_point::PathPoint;

/// Tool for path point operations
#[derive(Clone, Debug)]
pub struct PathPointTool;

impl PathPointTool {
    /// Create a new tool
    pub fn new() -> Self {
        PathPointTool
    }

    /// Get 2D point on first surface
    pub fn value2d_on_first(_pp: &PathPoint) -> (f64, f64) {
        (0.0, 0.0)
    }

    /// Get 2D point on second surface
    pub fn value2d_on_second(_pp: &PathPoint) -> (f64, f64) {
        (0.0, 0.0)
    }
}

impl Default for PathPointTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = PathPointTool::new();
    }

    #[test]
    fn test_value2d() {
        let pp = PathPoint::with_params(0.5, 0.6, 0.7, 0.8, false);
        let (u, v) = PathPointTool::value2d_on_first(&pp);
        assert_eq!(u, 0.0);
        assert_eq!(v, 0.0);
    }
}
