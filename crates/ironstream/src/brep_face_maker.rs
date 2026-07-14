// FILE: rust/ironstream/crates/ironstream/src/brep_face_maker.rs
//! `BRepBuilderAPI_MakeFace` — wire-centric face construction stubs, mirroring
//! the corresponding OCCT builder class with planar, cylindrical and
//! wire-bounded surface support.

// ---------------------------------------------------------------------------
// FaceError
// ---------------------------------------------------------------------------

// occt-ref: BRepBuilderAPI_FaceError // — error codes returned by BRepBuilderAPI_MakeFace::Error()
/// Error codes produced by [`FaceMaker`].
///
/// Mirrors the `BRepBuilderAPI_FaceError` enumeration from OCCT:
///
/// | Variant                  | OCCT equivalent                                           |
/// |--------------------------|-----------------------------------------------------------|
/// | `NoFace`                 | `BRepBuilderAPI_NoFace`                                   |
/// | `NotPlanar`              | `BRepBuilderAPI_NotPlanar`                                |
/// | `CurveProjectionFailed`  | `BRepBuilderAPI_CurveProjectionFailed`                    |
/// | `ParametersOutOfRange`   | `BRepBuilderAPI_ParametersOutOfRange`                     |
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaceError {
    // occt-note: BRepBuilderAPI_NoFace — no face has been built yet
    /// No face has been constructed.
    NoFace,
    // occt-note: BRepBuilderAPI_NotPlanar — the wire does not lie on a plane
    /// The wire does not define a planar contour.
    NotPlanar,
    // occt-note: BRepBuilderAPI_CurveProjectionFailed — PCurve computation failed
    /// Projection of a curve onto the surface failed.
    CurveProjectionFailed,
    // occt-note: BRepBuilderAPI_ParametersOutOfRange — UV bounds are invalid
    /// The supplied UV parameter bounds are out of range.
    ParametersOutOfRange,
}

// ---------------------------------------------------------------------------
// FaceMaker
// ---------------------------------------------------------------------------

// occt-ref: BRepBuilderAPI_MakeFace
/// Stub builder mirroring `BRepBuilderAPI_MakeFace`.
///
/// Constructs a topological face from a wire boundary, a planar surface
/// defined by an origin and normal vector, or a cylindrical surface defined
/// by a radius and height.  The `face` field carries a human-readable label
/// describing the resulting face; `done` is set to `true` when construction
/// succeeds.
#[derive(Debug, Clone)]
pub struct FaceMaker {
    // occt-ref: BRepBuilderAPI_MakeFace // — resulting TopoDS_Face handle (stub: label string)
    pub face: String,
    // occt-ref: BRepBuilderAPI_MakeFace // ::Error
    pub error: Option<FaceError>,
    // occt-ref: BRepBuilderAPI_MakeFace // ::IsDone
    pub done: bool,
}

impl FaceMaker {
    /// Construct a face from a closed wire boundary.
    ///
    /// OCCT attempts to find a supporting surface automatically; if the wire
    /// is planar, a `Geom_Plane` is used.  This stub succeeds for any
    /// non-empty wire label.
    ///
    /// Mirrors `BRepBuilderAPI_MakeFace(const TopoDS_Wire&, Standard_Boolean)`.
    // occt-note: BRepBuilderAPI_MakeFace(const TopoDS_Wire& W, Standard_Boolean OnlyPlane)
    pub fn from_wire(wire: &str) -> Self {
        if wire.is_empty() {
            return Self {
                face: String::new(),
                error: Some(FaceError::NoFace),
                done: false,
            };
        }
        Self {
            face: format!("face(wire={})", wire),
            error: None,
            done: true,
        }
    }

    /// Construct a face from a planar surface defined by an origin point and a
    /// unit normal vector.
    ///
    /// Mirrors `BRepBuilderAPI_MakeFace(const gp_Pln&)` where the plane is
    /// built from the supplied `origin` and `normal`.
    // occt-note: BRepBuilderAPI_MakeFace(const gp_Pln& P)
    pub fn from_plane(origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self {
            face: format!(
                "face(plane(origin=({},{},{}),normal=({},{},{})))",
                origin[0], origin[1], origin[2],
                normal[0], normal[1], normal[2]
            ),
            error: None,
            done: true,
        }
    }

    /// Construct a face from a cylindrical surface defined by a radius `r` and
    /// height `h`.
    ///
    /// The cylinder is assumed to be axis-aligned along Z with its base at the
    /// origin.  Mirrors `BRepBuilderAPI_MakeFace(const gp_Cylinder&, Standard_Real, Standard_Real)`.
    // occt-note: BRepBuilderAPI_MakeFace(const gp_Cylinder& C, Standard_Real P1, Standard_Real P2)
    pub fn from_cylinder(r: f64, h: f64) -> Self {
        if r <= 0.0 || h <= 0.0 {
            return Self {
                face: String::new(),
                error: Some(FaceError::ParametersOutOfRange),
                done: false,
            };
        }
        Self {
            face: format!("face(cylinder(r={},h={}))", r, h),
            error: None,
            done: true,
        }
    }

    /// Add an interior wire (hole) to the face, returning a mutable reference
    /// to `self` for method chaining.
    ///
    /// Mirrors `BRepBuilderAPI_MakeFace::Add(const TopoDS_Wire&)`.
    // occt-ref: BRepBuilderAPI_MakeFace // ::Add(const TopoDS_Wire& W)
    pub fn add_wire(&mut self, wire: &str) -> &mut Self {
        if self.done && !wire.is_empty() {
            let existing = self.face.trim_end_matches(')').to_string();
            self.face = format!("{},inner={})", existing, wire);
        }
        self
    }

    /// Return a reference to the label of the constructed face.
    ///
    /// Mirrors `BRepBuilderAPI_MakeFace::Face()`.
    // occt-ref: BRepBuilderAPI_MakeFace // ::Face() -> const TopoDS_Face&
    pub fn face(&self) -> &str {
        &self.face
    }

    /// Return `true` when the face was successfully constructed.
    ///
    /// Mirrors `BRepBuilderAPI_MakeFace::IsDone()`.
    // occt-ref: BRepBuilderAPI_MakeFace // ::IsDone() -> Standard_Boolean
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Return the error code if construction failed, or `None` on success.
    ///
    /// Mirrors `BRepBuilderAPI_MakeFace::Error()`.
    // occt-ref: BRepBuilderAPI_MakeFace // ::Error() -> BRepBuilderAPI_FaceError
    pub fn error(&self) -> Option<&FaceError> {
        self.error.as_ref()
    }
}

// ---------------------------------------------------------------------------
// Free-function helpers
// ---------------------------------------------------------------------------

/// Construct a planar face label from a closed wire.
///
/// Convenience wrapper around [`FaceMaker::from_wire`] that returns the face
/// label directly, mirroring typical single-call usage of
/// `BRepBuilderAPI_MakeFace(wire, Standard_True).Face()`.
// occt-note: BRepBuilderAPI_MakeFace(const TopoDS_Wire&, Standard_Boolean) — convenience
pub fn make_planar_face(wire: &str) -> String {
    FaceMaker::from_wire(wire).face
}

/// Construct a parametrically bounded planar face.
///
/// Builds a `Geom_Plane` from `origin` and `normal`, then trims it to the
/// rectangle `[u1, u2] × [v1, v2]` in the plane's own UV coordinate system.
///
/// Mirrors `BRepBuilderAPI_MakeFace(const gp_Pln&, u1, u2, v1, v2)`.
// occt-note: BRepBuilderAPI_MakeFace(const gp_Pln& P, Standard_Real u1, Standard_Real u2, Standard_Real v1, Standard_Real v2)
pub fn make_face_from_plane(
    origin: [f64; 3],
    normal: [f64; 3],
    u1: f64,
    u2: f64,
    v1: f64,
    v2: f64,
) -> String {
    if u1 >= u2 || v1 >= v2 {
        return String::new();
    }
    format!(
        "face(plane(origin=({},{},{}),normal=({},{},{})),u=[{},{}],v=[{},{}])",
        origin[0], origin[1], origin[2],
        normal[0], normal[1], normal[2],
        u1, u2, v1, v2
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_wire_is_done() {
        let m = FaceMaker::from_wire("wire_1");
        assert!(m.is_done());
        assert!(m.face().contains("wire_1"));
        assert!(m.error().is_none());
    }

    #[test]
    fn from_wire_empty_fails() {
        let m = FaceMaker::from_wire("");
        assert!(!m.is_done());
        assert_eq!(m.error(), Some(&FaceError::NoFace));
    }

    #[test]
    fn from_plane_is_done() {
        let m = FaceMaker::from_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(m.is_done());
        assert!(m.face().contains("plane"));
        assert!(m.error().is_none());
    }

    #[test]
    fn from_cylinder_is_done() {
        let m = FaceMaker::from_cylinder(3.0, 10.0);
        assert!(m.is_done());
        assert!(m.face().contains("cylinder"));
        assert!(m.error().is_none());
    }

    #[test]
    fn from_cylinder_bad_params() {
        let m = FaceMaker::from_cylinder(-1.0, 5.0);
        assert!(!m.is_done());
        assert_eq!(m.error(), Some(&FaceError::ParametersOutOfRange));
    }

    #[test]
    fn add_wire_appends_inner() {
        let mut m = FaceMaker::from_wire("outer");
        m.add_wire("hole_1");
        assert!(m.face().contains("hole_1"));
    }

    #[test]
    fn add_wire_chaining() {
        let mut m = FaceMaker::from_wire("outer");
        m.add_wire("hole_a").add_wire("hole_b");
        let f = m.face();
        assert!(f.contains("hole_a"));
        assert!(f.contains("hole_b"));
    }

    #[test]
    fn make_planar_face_helper() {
        let label = make_planar_face("rect_wire");
        assert!(label.contains("wire=rect_wire"));
    }

    #[test]
    fn make_planar_face_empty_wire() {
        let label = make_planar_face("");
        assert!(label.is_empty());
    }

    #[test]
    fn make_face_from_plane_helper() {
        let label = make_face_from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            -5.0, 5.0,
            -5.0, 5.0,
        );
        assert!(label.contains("plane"));
        assert!(label.contains("u=[-5,5]"));
        assert!(label.contains("v=[-5,5]"));
    }

    #[test]
    fn make_face_from_plane_invalid_range() {
        let label = make_face_from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0, 1.0,
            0.0, 1.0,
        );
        assert!(label.is_empty());
    }

    #[test]
    fn face_error_variants_are_distinct() {
        assert_ne!(FaceError::NoFace, FaceError::NotPlanar);
        assert_ne!(FaceError::CurveProjectionFailed, FaceError::ParametersOutOfRange);
    }
}
