// FILE: rust/ironstream/crates/ironstream/src/geom_constraint.rs

//! Constraint types for plate-surface and general surface fitting.
//!
//! Mirrors the `GeomPlate` constraint hierarchy from OpenCascade:
//! `GeomPlate_BuildPlateSurface`, `GeomPlate_PointConstraint`, and
//! `GeomPlate_CurveConstraint`.  Zero third-party dependencies; only `std`.

// occt: GeomPlate_BuildPlateSurface constraint type — enum G0 (point on), G1 (tangent), G2 (curvature)
/// Geometric continuity order required at a constraint.
///
/// Mirrors the integer order used by `GeomPlate_BuildPlateSurface`:
/// `G0` = position only, `G1` = tangent continuity, `G2` = curvature continuity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeomConstraintType {
    /// Position constraint — the surface must pass through the locus.
    G0 = 0,
    /// Tangent constraint — the surface must be tangent-continuous at the locus.
    G1 = 1,
    /// Curvature constraint — the surface must be curvature-continuous at the locus.
    G2 = 2,
}

// occt: GeomPlate_PointConstraint — a point the surface must pass through
/// A single-point constraint for surface fitting.
///
/// Stores a 3D point and the required geometric continuity order, together with
/// an optional weight used by the plate solver when blending constraints.
#[derive(Clone, Debug, PartialEq)]
pub struct GeomPointConstraint {
    point: [f64; 3],
    constraint_type: GeomConstraintType,
    weight: f64,
}

impl GeomPointConstraint {
    /// Create a new point constraint with default weight `1.0`.
    ///
    /// `point` — the 3-D location `[x, y, z]` the surface must satisfy.
    /// `constraint_type` — continuity order required at this point.
    pub fn new(point: [f64; 3], constraint_type: GeomConstraintType) -> Self {
        Self {
            point,
            constraint_type,
            weight: 1.0,
        }
    }

    /// Return the constrained point location.
    #[inline]
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the continuity order for this constraint.
    #[inline]
    pub fn constraint_type(&self) -> GeomConstraintType {
        self.constraint_type
    }

    /// Return the solver weight for this constraint.
    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Set the solver weight for this constraint.
    #[inline]
    pub fn set_weight(&mut self, w: f64) {
        self.weight = w;
    }
}

// occt: GeomPlate_CurveConstraint — a curve the surface must contain
/// A curve-based constraint for surface fitting.
///
/// Rather than storing a full curve object (which would require a trait object
/// or enum), this type records the discretisation count (`nb_points`), the
/// continuity order, and an optional blending weight.  The actual curve
/// geometry is managed by the caller and passed to the solver separately,
/// matching the pattern used in `GeomPlate_CurveConstraint`.
#[derive(Clone, Debug, PartialEq)]
pub struct GeomCurveConstraint {
    nb_points: usize,
    constraint_type: GeomConstraintType,
    order: u32,
    weight: f64,
}

impl GeomCurveConstraint {
    /// Create a new curve constraint with default weight `1.0`.
    ///
    /// `nb_points` — number of sample points used to discretise the curve.
    /// `constraint_type` — continuity order required along the curve.
    /// `order` — polynomial order hint for the plate solver.
    pub fn new(nb_points: usize, constraint_type: GeomConstraintType, order: u32) -> Self {
        Self {
            nb_points,
            constraint_type,
            order,
            weight: 1.0,
        }
    }

    /// Return the number of discretisation sample points.
    #[inline]
    pub fn nb_points(&self) -> usize {
        self.nb_points
    }

    /// Return the continuity order for this constraint.
    #[inline]
    pub fn constraint_type(&self) -> GeomConstraintType {
        self.constraint_type
    }

    /// Return the polynomial order hint.
    #[inline]
    pub fn order(&self) -> u32 {
        self.order
    }

    /// Return the solver weight for this constraint.
    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Set the solver weight for this constraint.
    #[inline]
    pub fn set_weight(&mut self, w: f64) {
        self.weight = w;
    }
}

// occt: container for constraints
/// A container that holds point and curve constraints for a surface solver.
///
/// Mirrors the role of the constraint lists maintained inside
/// `GeomPlate_BuildPlateSurface`.  Constraints are appended and accessed
/// by index, matching the OCCT `NbPoints` / `NbCurves` / `Point(i)` /
/// `Curve(i)` API shape.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeomConstraintSet {
    point_constraints: Vec<GeomPointConstraint>,
    curve_constraints: Vec<GeomCurveConstraint>,
}

impl GeomConstraintSet {
    /// Create an empty constraint set.
    pub fn new() -> Self {
        Self {
            point_constraints: Vec::new(),
            curve_constraints: Vec::new(),
        }
    }

    /// Append a point constraint.
    pub fn add_point(&mut self, c: GeomPointConstraint) {
        self.point_constraints.push(c);
    }

    /// Append a curve constraint.
    pub fn add_curve(&mut self, c: GeomCurveConstraint) {
        self.curve_constraints.push(c);
    }

    /// Return the number of point constraints.
    #[inline]
    pub fn nb_points(&self) -> usize {
        self.point_constraints.len()
    }

    /// Return the number of curve constraints.
    #[inline]
    pub fn nb_curves(&self) -> usize {
        self.curve_constraints.len()
    }

    /// Return a reference to the point constraint at index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= self.nb_points()`.
    #[inline]
    pub fn point(&self, i: usize) -> &GeomPointConstraint {
        &self.point_constraints[i]
    }

    /// Return a reference to the curve constraint at index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= self.nb_curves()`.
    #[inline]
    pub fn curve(&self, i: usize) -> &GeomCurveConstraint {
        &self.curve_constraints[i]
    }
}
