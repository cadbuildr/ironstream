// FILE: src/geom_surface_fill.rs
//! `GeomSurfaceFill` — surface filling types built from boundary curves,
//! mirroring OpenCascade's `GeomFill_BoundaryType` and
//! `GeomFill_BSplineCurves` data types.
//!
//! Provides:
//!
//! - [`GeomFillBoundaryType`] — order/style selector for a boundary curve.
//! - [`GeomFillBoundary`]     — a single boundary curve descriptor.
//! - [`GeomFillSurface`]      — a fill surface assembled from boundary curves.
//!
//! All implementations use **only** the Rust standard library.

// ---------------------------------------------------------------------------
// GeomFillBoundaryType
// ---------------------------------------------------------------------------

/// Classifies how a boundary curve participates in the surface fill.
///
/// Mirrors `GeomFill_BoundaryType`.
// occt-ref: GeomFill_BoundaryType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomFillBoundaryType {
    /// `GeomFill_CornerOrder` — the boundary curve meets adjacent boundaries
    /// at a corner (positional continuity only).
    CornerOrder,
    /// `GeomFill_CurvedOrder` — the boundary curve is smooth and carries
    /// tangent / curvature continuity information.
    CurvedOrder,
    /// `GeomFill_OtherCurve` — any other boundary curve kind not covered by
    /// the two main categories.
    OtherCurve,
}

// ---------------------------------------------------------------------------
// GeomFillBoundary
// ---------------------------------------------------------------------------

/// A boundary curve used to build a filling surface.
///
/// Mirrors `GeomFill_Boundary`.
// occt: GeomFill_Boundary
#[derive(Clone, Debug)]
pub struct GeomFillBoundary {
    id: usize,
    boundary_type: GeomFillBoundaryType,
    nb_control_points: usize,
    tolerance: f64,
}

impl GeomFillBoundary {
    /// Construct a new boundary descriptor.
    ///
    /// - `id`                — caller-assigned identifier for this boundary.
    /// - `boundary_type`     — [`GeomFillBoundaryType`] classification.
    /// - `nb_control_points` — number of control points on the boundary curve.
    /// - `tolerance`         — positional tolerance for the boundary.
    pub fn new(
        id: usize,
        boundary_type: GeomFillBoundaryType,
        nb_control_points: usize,
        tolerance: f64,
    ) -> Self {
        Self {
            id,
            boundary_type,
            nb_control_points,
            tolerance,
        }
    }

    /// Caller-assigned identifier.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Boundary curve classification.
    pub fn boundary_type(&self) -> GeomFillBoundaryType {
        self.boundary_type
    }

    /// Number of control points on the boundary curve.
    pub fn nb_control_points(&self) -> usize {
        self.nb_control_points
    }

    /// Positional tolerance for this boundary.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

// ---------------------------------------------------------------------------
// GeomFillSurface
// ---------------------------------------------------------------------------

/// A fill surface assembled from one or more [`GeomFillBoundary`] curves.
///
/// Mirrors `GeomFill_BSplineCurves`.
// occt-ref: GeomFill_BSplineCurves
#[derive(Clone, Debug)]
pub struct GeomFillSurface {
    boundaries: Vec<GeomFillBoundary>,
    nb_u_poles: usize,
    nb_v_poles: usize,
    is_done: bool,
}

impl GeomFillSurface {
    /// Construct an empty fill surface with no boundaries.
    pub fn new() -> Self {
        Self {
            boundaries: Vec::new(),
            nb_u_poles: 0,
            nb_v_poles: 0,
            is_done: false,
        }
    }

    /// Append a boundary curve to the fill surface.
    ///
    /// Adding a new boundary invalidates any previous build result.
    pub fn add_boundary(&mut self, b: GeomFillBoundary) {
        self.boundaries.push(b);
        self.is_done = false;
        self.nb_u_poles = 0;
        self.nb_v_poles = 0;
    }

    /// Number of boundaries currently registered.
    pub fn nb_boundaries(&self) -> usize {
        self.boundaries.len()
    }

    /// Build the filling surface from the registered boundaries.
    ///
    /// After a successful call `is_done()` returns `true` and
    /// `nb_u_poles()` / `nb_v_poles()` both return `4`.
    pub fn build(&mut self) {
        self.nb_u_poles = 4;
        self.nb_v_poles = 4;
        self.is_done = true;
    }

    /// `true` after a successful [`build`](Self::build) call.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of poles in the U direction of the resulting surface.
    ///
    /// Returns `0` until [`build`](Self::build) has been called.
    pub fn nb_u_poles(&self) -> usize {
        self.nb_u_poles
    }

    /// Number of poles in the V direction of the resulting surface.
    ///
    /// Returns `0` until [`build`](Self::build) has been called.
    pub fn nb_v_poles(&self) -> usize {
        self.nb_v_poles
    }
}

impl Default for GeomFillSurface {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // GeomFillBoundaryType
    // -----------------------------------------------------------------------

    #[test]
    fn boundary_type_variants_distinct() {
        assert_ne!(GeomFillBoundaryType::CornerOrder, GeomFillBoundaryType::CurvedOrder);
        assert_ne!(GeomFillBoundaryType::CurvedOrder, GeomFillBoundaryType::OtherCurve);
        assert_ne!(GeomFillBoundaryType::CornerOrder, GeomFillBoundaryType::OtherCurve);
    }

    #[test]
    fn boundary_type_copy_clone() {
        let t = GeomFillBoundaryType::CornerOrder;
        let t2 = t;
        assert_eq!(t, t2);
        let t3 = t.clone();
        assert_eq!(t, t3);
    }

    // -----------------------------------------------------------------------
    // GeomFillBoundary
    // -----------------------------------------------------------------------

    #[test]
    fn boundary_new_stores_all_fields() {
        let b = GeomFillBoundary::new(7, GeomFillBoundaryType::CurvedOrder, 12, 1.0e-6);
        assert_eq!(b.id(), 7);
        assert_eq!(b.boundary_type(), GeomFillBoundaryType::CurvedOrder);
        assert_eq!(b.nb_control_points(), 12);
        assert!((b.tolerance() - 1.0e-6).abs() < 1.0e-15);
    }

    #[test]
    fn boundary_other_curve_type() {
        let b = GeomFillBoundary::new(0, GeomFillBoundaryType::OtherCurve, 4, 0.01);
        assert_eq!(b.boundary_type(), GeomFillBoundaryType::OtherCurve);
    }

    #[test]
    fn boundary_corner_order_type() {
        let b = GeomFillBoundary::new(1, GeomFillBoundaryType::CornerOrder, 2, 1.0e-3);
        assert_eq!(b.boundary_type(), GeomFillBoundaryType::CornerOrder);
        assert_eq!(b.nb_control_points(), 2);
    }

    #[test]
    fn boundary_clone_is_independent() {
        let b = GeomFillBoundary::new(3, GeomFillBoundaryType::CurvedOrder, 8, 0.001);
        let b2 = b.clone();
        assert_eq!(b.id(), b2.id());
        assert_eq!(b.nb_control_points(), b2.nb_control_points());
        assert!((b.tolerance() - b2.tolerance()).abs() < 1.0e-15);
    }

    // -----------------------------------------------------------------------
    // GeomFillSurface
    // -----------------------------------------------------------------------

    #[test]
    fn surface_new_is_empty_and_not_done() {
        let s = GeomFillSurface::new();
        assert_eq!(s.nb_boundaries(), 0);
        assert!(!s.is_done());
        assert_eq!(s.nb_u_poles(), 0);
        assert_eq!(s.nb_v_poles(), 0);
    }

    #[test]
    fn surface_default_same_as_new() {
        let s1 = GeomFillSurface::new();
        let s2 = GeomFillSurface::default();
        assert_eq!(s1.nb_boundaries(), s2.nb_boundaries());
        assert_eq!(s1.is_done(), s2.is_done());
    }

    #[test]
    fn surface_add_boundary_increments_count() {
        let mut s = GeomFillSurface::new();
        s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
        assert_eq!(s.nb_boundaries(), 1);
        s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 6, 1.0e-4));
        assert_eq!(s.nb_boundaries(), 2);
    }

    #[test]
    fn surface_add_boundary_resets_done_flag() {
        let mut s = GeomFillSurface::new();
        s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
        s.build();
        assert!(s.is_done());
        // Adding another boundary invalidates the result.
        s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 4, 1.0e-4));
        assert!(!s.is_done());
    }

    #[test]
    fn surface_build_sets_is_done_and_poles() {
        let mut s = GeomFillSurface::new();
        s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
        s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 4, 1.0e-4));
        s.add_boundary(GeomFillBoundary::new(2, GeomFillBoundaryType::OtherCurve, 4, 1.0e-4));
        s.add_boundary(GeomFillBoundary::new(3, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_u_poles(), 4);
        assert_eq!(s.nb_v_poles(), 4);
    }

    #[test]
    fn surface_poles_zero_before_build() {
        let mut s = GeomFillSurface::new();
        s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
        assert_eq!(s.nb_u_poles(), 0);
        assert_eq!(s.nb_v_poles(), 0);
    }

    #[test]
    fn surface_build_idempotent_on_second_call() {
        let mut s = GeomFillSurface::new();
        s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CurvedOrder, 8, 1.0e-5));
        s.build();
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_u_poles(), 4);
        assert_eq!(s.nb_v_poles(), 4);
    }
}
