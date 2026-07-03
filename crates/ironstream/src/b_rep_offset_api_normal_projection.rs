// FILE: b_rep_offset_api_normal_projection.rs
// occt: BRepOffsetAPI_NormalProjection

/// Computes the projection of a shape onto a surface along surface normals.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiNormalProjection {
    /// Whether the operation was successful
    is_done: bool,
    /// Tolerance for projection
    tolerance: f64,
}

impl BRepOffsetApiNormalProjection {
    /// Create a new NormalProjection builder
    pub fn new() -> Self {
        Self {
            is_done: false,
            tolerance: 1e-7,
        }
    }

    /// Check if the operation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark operation as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }

    /// Set tolerance
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Get tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

impl Default for BRepOffsetApiNormalProjection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let proj = BRepOffsetApiNormalProjection::new();
        assert!(!proj.is_done());
        assert!(proj.tolerance() > 0.0);
    }

    #[test]
    fn test_set_tolerance() {
        let mut proj = BRepOffsetApiNormalProjection::new();
        proj.set_tolerance(0.001);
        assert_eq!(proj.tolerance(), 0.001);
    }

    #[test]
    fn test_set_done() {
        let mut proj = BRepOffsetApiNormalProjection::new();
        proj.set_done();
        assert!(proj.is_done());
    }

    #[test]
    fn test_default() {
        let proj = BRepOffsetApiNormalProjection::default();
        assert!(!proj.is_done());
    }
}
