// FILE: brep_topology.rs

// ---------------------------------------------------------------------------
// BRepOrientation — edge orientation in a wire
// ---------------------------------------------------------------------------

/// Orientation of an edge within a wire.
///
/// Re-exported here from `top_abs` for caller convenience; the underlying
/// semantics are identical to `top_abs::Orientation`.
// occt-ref: TopAbs_Orientation // (re-export as BRepOrientation)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

// ---------------------------------------------------------------------------
// BRepCurveKind — how a curve is attached to an edge
// ---------------------------------------------------------------------------

/// Discriminant for how a curve is represented on an edge.
// occt-ref: BRep_CurveRepresentation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepCurveKind {
    Curve3d,
    CurveOnSurface,
    Polygon3d,
    PolygonOnSurface,
    Undefined,
}

// ---------------------------------------------------------------------------
// BRepPointRepresentation — vertex position data
// ---------------------------------------------------------------------------

/// Stores the 3-D position and tolerance of a vertex as it appears in BRep
/// data.  A vertex may have multiple representations (one per edge it belongs
/// to) with different tolerances; this struct holds one such record.
// occt: BRep_PointRepresentation
#[derive(Clone, Debug)]
pub struct BRepPointRepresentation {
    pub tolerance: f64,
    pub point: [f64; 3],
}

impl BRepPointRepresentation {
    pub fn new(x: f64, y: f64, z: f64, tolerance: f64) -> Self {
        Self { tolerance, point: [x, y, z] }
    }
}

// ---------------------------------------------------------------------------
// BRepEdgeData — simplified BRep_TEdge
// ---------------------------------------------------------------------------

/// Per-edge BRep data: tolerance and flags that govern how the edge relates
/// to its curve representation.
// occt-ref: BRep_TEdge
pub struct BRepEdgeData {
    pub tolerance: f64,
    pub same_parameter: bool,
    pub same_range: bool,
    pub degenerated: bool,
    pub curve_kind: BRepCurveKind,
}

impl BRepEdgeData {
    pub fn new() -> Self {
        Self {
            tolerance: 1e-7,
            same_parameter: true,
            same_range: true,
            degenerated: false,
            curve_kind: BRepCurveKind::Undefined,
        }
    }
}

impl Default for BRepEdgeData {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BRepFaceData — simplified BRep_TFace
// ---------------------------------------------------------------------------

/// Per-face BRep data: tolerance, whether the parametric domain exactly
/// matches the surface's natural bounds, and an optional location index.
// occt: BRep_TFace
pub struct BRepFaceData {
    pub tolerance: f64,
    pub natural_restriction: bool,
    pub location_index: u32,
}

impl BRepFaceData {
    pub fn new() -> Self {
        Self {
            tolerance: 1e-7,
            natural_restriction: false,
            location_index: 0,
        }
    }
}

impl Default for BRepFaceData {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BRepBuilder — simplified BRep_Builder
// ---------------------------------------------------------------------------

/// Low-level BRep builder that mutates edge/face/vertex data records.
///
/// `BRepBuilder` is stateless; all methods take `&self` and operate on the
/// provided data reference.  This mirrors OCCT's `BRep_Builder`, which acts
/// as a thin façade over the `BRep_T*` inner data classes.
// occt-ref: BRep_Builder
pub struct BRepBuilder;

impl BRepBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Grow `e.tolerance` to at least `tol`.  Never shrinks the tolerance.
    pub fn update_tolerance(&self, e: &mut BRepEdgeData, tol: f64) {
        e.tolerance = tol.max(e.tolerance);
    }

    /// Grow `f.tolerance` to at least `tol`.  Never shrinks the tolerance.
    pub fn update_face_tolerance(&self, f: &mut BRepFaceData, tol: f64) {
        f.tolerance = tol.max(f.tolerance);
    }

    /// Grow `p.tolerance` to at least `tol`.  Never shrinks the tolerance.
    pub fn update_vertex_tolerance(&self, p: &mut BRepPointRepresentation, tol: f64) {
        p.tolerance = tol.max(p.tolerance);
    }

    pub fn set_same_parameter(&self, e: &mut BRepEdgeData, v: bool) {
        e.same_parameter = v;
    }

    pub fn set_same_range(&self, e: &mut BRepEdgeData, v: bool) {
        e.same_range = v;
    }

    pub fn set_degenerated(&self, e: &mut BRepEdgeData, v: bool) {
        e.degenerated = v;
    }
}

impl Default for BRepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_data_defaults() {
        let e = BRepEdgeData::new();
        assert!((e.tolerance - 1e-7).abs() < 1e-20);
        assert!(e.same_parameter);
        assert!(e.same_range);
        assert!(!e.degenerated);
        assert_eq!(e.curve_kind, BRepCurveKind::Undefined);
    }

    #[test]
    fn face_data_defaults() {
        let f = BRepFaceData::new();
        assert!((f.tolerance - 1e-7).abs() < 1e-20);
        assert!(!f.natural_restriction);
        assert_eq!(f.location_index, 0);
    }

    #[test]
    fn point_repr_stores_coords() {
        let p = BRepPointRepresentation::new(1.0, 2.0, 3.0, 1e-6);
        assert_eq!(p.point, [1.0, 2.0, 3.0]);
        assert!((p.tolerance - 1e-6).abs() < 1e-20);
    }

    #[test]
    fn update_tolerance_grows_edge() {
        let b = BRepBuilder::new();
        let mut e = BRepEdgeData::new();
        b.update_tolerance(&mut e, 1e-4);
        assert!((e.tolerance - 1e-4).abs() < 1e-20);
    }

    #[test]
    fn update_tolerance_does_not_shrink_edge() {
        let b = BRepBuilder::new();
        let mut e = BRepEdgeData::new();
        e.tolerance = 1e-3;
        b.update_tolerance(&mut e, 1e-7);
        assert!((e.tolerance - 1e-3).abs() < 1e-20);
    }

    #[test]
    fn update_face_tolerance_grows() {
        let b = BRepBuilder::new();
        let mut f = BRepFaceData::new();
        b.update_face_tolerance(&mut f, 5e-5);
        assert!((f.tolerance - 5e-5).abs() < 1e-20);
    }

    #[test]
    fn update_vertex_tolerance_grows() {
        let b = BRepBuilder::new();
        let mut p = BRepPointRepresentation::new(0.0, 0.0, 0.0, 1e-7);
        b.update_vertex_tolerance(&mut p, 1e-4);
        assert!((p.tolerance - 1e-4).abs() < 1e-20);
    }

    #[test]
    fn set_flags_on_edge() {
        let b = BRepBuilder::new();
        let mut e = BRepEdgeData::new();
        b.set_same_parameter(&mut e, false);
        b.set_same_range(&mut e, false);
        b.set_degenerated(&mut e, true);
        assert!(!e.same_parameter);
        assert!(!e.same_range);
        assert!(e.degenerated);
    }

    #[test]
    fn brep_orientation_variants_distinct() {
        assert_ne!(BRepOrientation::Forward, BRepOrientation::Reversed);
        assert_ne!(BRepOrientation::Internal, BRepOrientation::External);
    }

    #[test]
    fn curve_kind_variants_distinct() {
        assert_ne!(BRepCurveKind::Curve3d, BRepCurveKind::CurveOnSurface);
        assert_eq!(BRepCurveKind::Undefined, BRepCurveKind::Undefined);
    }
}
