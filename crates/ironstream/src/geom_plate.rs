// FILE: geom_plate.rs

// occt-ref: GeomPlate_BuildPlateSurface // continuity
/// Continuity order for plate surface constraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlateContinuity {
    G0 = 0,
    G1 = 1,
    G2 = 2,
}

// occt: GeomPlate_CurveConstraint
/// A curve constraint for plate surface construction.
pub struct PlateCurveConstraint {
    pub continuity: PlateContinuity,
    pub nb_points: usize,
}

impl PlateCurveConstraint {
    pub fn new(continuity: PlateContinuity, nb_points: usize) -> Self {
        Self { continuity, nb_points }
    }
}

// occt-ref: GeomPlate_PointConstraint
/// A point constraint for plate surface construction.
pub struct PlatePointConstraint {
    pub point: [f64; 3],
    pub continuity: PlateContinuity,
}

impl PlatePointConstraint {
    pub fn new(point: [f64; 3], continuity: PlateContinuity) -> Self {
        Self { point, continuity }
    }
}

/// Builder configuration for plate surface construction.
pub struct PlateBuildParams {
    pub degree: u32,
    pub max_degree: u32,
    pub max_segments: u32,
    pub tolerance: f64,
    pub nb_iter: u32,
}

impl PlateBuildParams {
    pub fn default() -> Self {
        Self {
            degree: 3,
            max_degree: 8,
            max_segments: 10,
            tolerance: 1e-4,
            nb_iter: 3,
        }
    }
}

// occt-ref: GeomPlate_BuildPlateSurface
/// Stub for OCCT's GeomPlate_BuildPlateSurface — constructs a plate surface
/// from a set of curve and point constraints.
pub struct GeomPlateBuildPlateSurface {
    params: PlateBuildParams,
    curve_constraints: Vec<PlateCurveConstraint>,
    point_constraints: Vec<PlatePointConstraint>,
    is_done: bool,
}

impl GeomPlateBuildPlateSurface {
    pub fn new(params: PlateBuildParams) -> Self {
        Self {
            params,
            curve_constraints: Vec::new(),
            point_constraints: Vec::new(),
            is_done: false,
        }
    }

    pub fn add_curve_constraint(&mut self, c: PlateCurveConstraint) {
        self.curve_constraints.push(c);
    }

    pub fn add_point_constraint(&mut self, c: PlatePointConstraint) {
        self.point_constraints.push(c);
    }

    pub fn nb_curve_constraints(&self) -> usize {
        self.curve_constraints.len()
    }

    pub fn nb_point_constraints(&self) -> usize {
        self.point_constraints.len()
    }

    /// Solve the plate equations (stub: marks is_done = true).
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// G0 approximation error (stub).
    pub fn g0_error(&self) -> f64 {
        0.0
    }

    /// G1 approximation error (stub).
    pub fn g1_error(&self) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params() {
        let p = PlateBuildParams::default();
        assert_eq!(p.degree, 3);
        assert_eq!(p.max_degree, 8);
        assert_eq!(p.max_segments, 10);
        assert!((p.tolerance - 1e-4).abs() < 1e-12);
        assert_eq!(p.nb_iter, 3);
    }

    #[test]
    fn builder_starts_empty_and_not_done() {
        let b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
        assert_eq!(b.nb_curve_constraints(), 0);
        assert_eq!(b.nb_point_constraints(), 0);
        assert!(!b.is_done());
    }

    #[test]
    fn perform_sets_is_done() {
        let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
        b.perform();
        assert!(b.is_done());
    }

    #[test]
    fn add_curve_constraint_increments_count() {
        let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
        b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G0, 10));
        b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G1, 20));
        assert_eq!(b.nb_curve_constraints(), 2);
    }

    #[test]
    fn add_point_constraint_increments_count() {
        let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
        b.add_point_constraint(PlatePointConstraint::new([1.0, 2.0, 3.0], PlateContinuity::G0));
        assert_eq!(b.nb_point_constraints(), 1);
    }

    #[test]
    fn error_stubs_return_zero() {
        let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
        b.perform();
        assert_eq!(b.g0_error(), 0.0);
        assert_eq!(b.g1_error(), 0.0);
    }

    #[test]
    fn continuity_ordering() {
        assert!(PlateContinuity::G0 < PlateContinuity::G1);
        assert!(PlateContinuity::G1 < PlateContinuity::G2);
    }
}
