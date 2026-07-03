// FILE: geom_plates.rs

// occt: GeomPlate_BuildPlateSurface
/// Continuity / approximation order for plate surface constraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlateOrder {
    G0,
    G1,
    G2,
}

// occt: GeomPlate_PointConstraint
/// A single-point constraint used when building a plate surface.
pub struct PointConstraint {
    pub point: [f64; 3],
    pub order: PlateOrder,
}

impl PointConstraint {
    /// Create a new `PointConstraint` at position `p` with default order `G0`.
    pub fn new(p: [f64; 3]) -> Self {
        Self {
            point: p,
            order: PlateOrder::G0,
        }
    }

    /// Return a copy of the constraint with the given continuity order.
    pub fn with_order(mut self, o: PlateOrder) -> Self {
        self.order = o;
        self
    }
}

// occt: GeomPlate_BuildPlateSurface
/// A curve constraint (referenced by name) used when building a plate surface.
pub struct CurveConstraint {
    pub curve_name: String,
    pub order: PlateOrder,
}

impl CurveConstraint {
    /// Create a new `CurveConstraint` for the curve identified by `c` with default order `G0`.
    pub fn new(c: &str) -> Self {
        Self {
            curve_name: c.to_string(),
            order: PlateOrder::G0,
        }
    }

    /// Return a copy of the constraint with the given continuity order.
    pub fn with_order(mut self, o: PlateOrder) -> Self {
        self.order = o;
        self
    }
}

// occt: GeomPlate_BuildPlateSurface
/// Builder that accumulates point and curve constraints and produces a plate surface
/// approximated as a collection of 3-D sample points.
pub struct PlateSurfaceBuilder {
    pub point_constraints: Vec<PointConstraint>,
    pub curve_constraints: Vec<CurveConstraint>,
    pub tolerance: f64,
    is_done: bool,
}

impl PlateSurfaceBuilder {
    /// Create a new builder with the given fitting tolerance.
    pub fn new(tol: f64) -> Self {
        Self {
            point_constraints: Vec::new(),
            curve_constraints: Vec::new(),
            tolerance: tol,
            is_done: false,
        }
    }

    /// Add a point constraint to the builder.
    pub fn add_point(&mut self, p: PointConstraint) {
        self.point_constraints.push(p);
    }

    /// Add a curve constraint to the builder.
    pub fn add_curve(&mut self, c: CurveConstraint) {
        self.curve_constraints.push(c);
    }

    /// Attempt to build the plate surface.
    ///
    /// Stub implementation: marks the builder as done and returns the
    /// positions of all registered point constraints as sample output
    /// points.  In a full implementation this would solve the thin-plate
    /// spline system over the supplied constraints.
    pub fn build(&mut self) -> Vec<[f64; 3]> {
        self.is_done = true;
        self.point_constraints.iter().map(|pc| pc.point).collect()
    }

    /// Returns `true` after a successful call to [`build`].
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- PlateOrder -------------------------------------------------------

    #[test]
    fn plate_order_variants_are_distinct() {
        assert_ne!(PlateOrder::G0, PlateOrder::G1);
        assert_ne!(PlateOrder::G1, PlateOrder::G2);
        assert_ne!(PlateOrder::G0, PlateOrder::G2);
    }

    // ---- PointConstraint --------------------------------------------------

    #[test]
    fn point_constraint_new_defaults_to_g0() {
        let pc = PointConstraint::new([1.0, 2.0, 3.0]);
        assert_eq!(pc.point, [1.0, 2.0, 3.0]);
        assert_eq!(pc.order, PlateOrder::G0);
    }

    #[test]
    fn point_constraint_with_order() {
        let pc = PointConstraint::new([0.0, 0.0, 0.0]).with_order(PlateOrder::G2);
        assert_eq!(pc.order, PlateOrder::G2);
    }

    #[test]
    fn point_constraint_with_order_g1() {
        let pc = PointConstraint::new([1.0, 0.0, 0.0]).with_order(PlateOrder::G1);
        assert_eq!(pc.order, PlateOrder::G1);
        assert_eq!(pc.point[0], 1.0);
    }

    // ---- CurveConstraint --------------------------------------------------

    #[test]
    fn curve_constraint_new_stores_name() {
        let cc = CurveConstraint::new("edge_42");
        assert_eq!(cc.curve_name, "edge_42");
        assert_eq!(cc.order, PlateOrder::G0);
    }

    #[test]
    fn curve_constraint_with_order() {
        let cc = CurveConstraint::new("boundary").with_order(PlateOrder::G1);
        assert_eq!(cc.order, PlateOrder::G1);
    }

    #[test]
    fn curve_constraint_empty_name() {
        let cc = CurveConstraint::new("");
        assert_eq!(cc.curve_name, "");
    }

    // ---- PlateSurfaceBuilder ----------------------------------------------

    #[test]
    fn builder_new_starts_empty_and_not_done() {
        let b = PlateSurfaceBuilder::new(1e-4);
        assert!(b.point_constraints.is_empty());
        assert!(b.curve_constraints.is_empty());
        assert!(!b.is_done());
        assert!((b.tolerance - 1e-4).abs() < 1e-15);
    }

    #[test]
    fn builder_add_point_increments_count() {
        let mut b = PlateSurfaceBuilder::new(1e-4);
        b.add_point(PointConstraint::new([0.0, 0.0, 0.0]));
        b.add_point(PointConstraint::new([1.0, 0.0, 0.0]));
        assert_eq!(b.point_constraints.len(), 2);
    }

    #[test]
    fn builder_add_curve_increments_count() {
        let mut b = PlateSurfaceBuilder::new(1e-4);
        b.add_curve(CurveConstraint::new("c1"));
        b.add_curve(CurveConstraint::new("c2"));
        b.add_curve(CurveConstraint::new("c3"));
        assert_eq!(b.curve_constraints.len(), 3);
    }

    #[test]
    fn builder_build_sets_is_done() {
        let mut b = PlateSurfaceBuilder::new(1e-6);
        assert!(!b.is_done());
        b.build();
        assert!(b.is_done());
    }

    #[test]
    fn builder_build_returns_point_positions() {
        let mut b = PlateSurfaceBuilder::new(1e-4);
        b.add_point(PointConstraint::new([1.0, 2.0, 3.0]));
        b.add_point(PointConstraint::new([4.0, 5.0, 6.0]));
        let pts = b.build();
        assert_eq!(pts.len(), 2);
        assert_eq!(pts[0], [1.0, 2.0, 3.0]);
        assert_eq!(pts[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn builder_build_empty_returns_empty_vec() {
        let mut b = PlateSurfaceBuilder::new(1e-3);
        let pts = b.build();
        assert!(pts.is_empty());
        assert!(b.is_done());
    }

    #[test]
    fn builder_build_ignores_curve_constraints_in_output() {
        let mut b = PlateSurfaceBuilder::new(1e-4);
        b.add_curve(CurveConstraint::new("rim"));
        b.add_point(PointConstraint::new([0.0, 0.0, 1.0]));
        let pts = b.build();
        // Only the point constraint contributes to the stub output.
        assert_eq!(pts.len(), 1);
        assert_eq!(pts[0], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn builder_tolerance_stored_correctly() {
        let b = PlateSurfaceBuilder::new(0.001);
        assert!((b.tolerance - 0.001).abs() < 1e-15);
    }
}
