// FILE: step_geom_surface_boundary.rs
// occt: StepGeom_SurfaceBoundary

/// Represents a boundary of a surface
pub struct StepGeomSurfaceBoundary {
    /// Boundary curve ID
    curve_id: i32,
    /// Whether boundary is outer (true) or inner (false) loop
    is_outer: bool,
}

impl StepGeomSurfaceBoundary {
    pub fn new(curve_id: i32, is_outer: bool) -> Self {
        StepGeomSurfaceBoundary {
            curve_id,
            is_outer,
        }
    }

    pub fn curve_id(&self) -> i32 {
        self.curve_id
    }

    pub fn is_outer(&self) -> bool {
        self.is_outer
    }

    pub fn is_inner(&self) -> bool {
        !self.is_outer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outer_boundary() {
        let boundary = StepGeomSurfaceBoundary::new(1, true);
        assert!(boundary.is_outer());
        assert!(!boundary.is_inner());
    }

    #[test]
    fn test_inner_boundary() {
        let boundary = StepGeomSurfaceBoundary::new(2, false);
        assert!(!boundary.is_outer());
        assert!(boundary.is_inner());
    }
}
