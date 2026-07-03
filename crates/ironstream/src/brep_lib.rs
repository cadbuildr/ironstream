// FILE: brep_lib.rs
//! `BRepLib` — utility library for boundary-representation geometry, mirroring
//! OpenCascade's `BRepLib`, `BRepLib_MakeEdge`, `BRepLib_MakeWire`, and
//! `BRepLib_MakeFace`.
//!
//! These builders construct topological shapes (edges, wires, faces) from
//! analytic geometry descriptors (lines, circles, planes, points).  The
//! IronStream tessellating kernel represents edges as sequences of sample
//! points and faces as planar polygonal regions; builders sample analytic
//! geometry to the requested or default precision.
//!
//! # Key types
//!
//! * [`BRepLib`] — global utility: orientation fixup, tolerance queries,
//!   surface/edge checks.
//! * [`BRepLib_MakeEdge`] / [`MakeEdge`] — build a topological edge from a
//!   curve (line, circle) and optional endpoint parameters.
//! * [`BRepLib_MakeWire`] / [`MakeWire`] — compose a closed wire from a
//!   sequence of edges.
//! * [`BRepLib_MakeFace`] / [`MakeFace`] — build a planar face from a wire or
//!   analytic surface + wire.

use crate::gp::{Ax3, EPS};
use crate::gp::{Pnt, Vec3};
use crate::gp_prim::{Circ, Lin};
use crate::topods::{Face, Vertex, Wire};
use std::f64::consts::TAU;

// ─────────────────────────── error type ──────────────────────────────────────

/// Error kinds that a `BRepLib` builder can produce, mirroring OCCT's
/// `BRepLib_MakeShapeError` / `BRepBuilderAPI_EdgeError` etc.
// occt: BRepLib_MakeShapeError, BRepBuilderAPI_EdgeError, BRepBuilderAPI_FaceError
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepLibError {
    /// The builder has not been invoked yet or holds no result.
    NotDone,
    /// Two coincident points were supplied where distinct points are required.
    CoincidentPoints,
    /// A curve parameter range is degenerate (first >= last after normalisation).
    ParameterOutOfRange,
    /// The supplied wire is not planar (for `MakeFace`).
    NonPlanarWire,
    /// The wire is invalid (fewer than 3 distinct vertices, etc.).
    InvalidWire,
    /// A supplied face or surface could not be oriented/validated.
    SurfaceNotC2,
    /// The circle radius is negative or zero.
    NegativeRadius,
    /// Internal construction failure (e.g. near-degenerate geometry).
    ConstructionError,
}

impl std::fmt::Display for BRepLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotDone => write!(f, "BRepLib: builder not done"),
            Self::CoincidentPoints => write!(f, "BRepLib: coincident points"),
            Self::ParameterOutOfRange => write!(f, "BRepLib: parameter out of range"),
            Self::NonPlanarWire => write!(f, "BRepLib: non-planar wire"),
            Self::InvalidWire => write!(f, "BRepLib: invalid wire"),
            Self::SurfaceNotC2 => write!(f, "BRepLib: surface not C2"),
            Self::NegativeRadius => write!(f, "BRepLib: negative radius"),
            Self::ConstructionError => write!(f, "BRepLib: construction error"),
        }
    }
}

// ─────────────────────────── TopoDS::Edge ────────────────────────────────────

// NOTE: the crate's `topods` module currently only exports `Vertex`, `Wire`,
// `Face`, `Shell`, `Solid`, `Compound`.  We therefore alias `Edge` = `Wire`
// for the purpose of this module; in OCCT an edge is a trimmed 1-manifold, and
// in IronStream a `Wire` already carries an ordered sequence of sample points
// which serves that role for a single-edge strip.

/// A topological edge — a trimmed 1-D curve represented as an ordered sequence
/// of sample points.  Mirrors `TopoDS_Edge`.
// occt: TopoDS_Edge
pub use crate::topods::Wire as TopoEdge;

// ─────────────────────────── BRepLib ─────────────────────────────────────────

/// Global BRepLib utilities: orientation checking, tolerance constants, and
/// shape fixup helpers.
// occt: BRepLib
pub struct BRepLib;

impl BRepLib {
    /// Default linear tolerance used in shape checks (mirrors
    /// `Precision::Confusion()` = 1e-7 in OCCT).
    pub const TOLERANCE: f64 = 1.0e-7;

    /// Default angular tolerance in radians (mirrors
    /// `Precision::Angular()` = 1e-12 in OCCT).
    pub const ANGULAR_TOLERANCE: f64 = 1.0e-12;

    /// Test whether a wire is planar within `tol`.  Returns the plane normal if
    /// it is, or `None` if the wire is not planar.
    ///
    /// Mirrors `BRepLib::OrientClosedSolid` / `BRepLib::FindValidRange` notion
    /// of planarity.
    pub fn wire_is_planar(wire: &Wire, tol: f64) -> Option<Vec3> {
        let pts = &wire.pts;
        if pts.len() < 3 {
            return None;
        }
        // Compute normal via Newell's method.
        let n = wire.normal();
        if n.norm() < EPS {
            return None;
        }
        // Check all points lie on the plane through pts[0] with that normal.
        let origin = pts[0];
        for p in pts.iter().skip(1) {
            let dist = (*p - origin).dot(n).abs();
            if dist > tol {
                return None;
            }
        }
        Some(n)
    }

    /// Compute the signed area of a planar wire using the shoelace formula
    /// projected onto the wire's best-fit plane.  Positive means CCW about the
    /// wire normal.
    pub fn wire_area(wire: &Wire) -> f64 {
        let pts = &wire.pts;
        let n = pts.len();
        if n < 3 {
            return 0.0;
        }
        let normal = wire.normal();
        // Project onto a local 2-D frame.
        let x_ax = (pts[1] - pts[0]).normalized();
        let y_ax = normal.cross(x_ax).normalized();
        let origin = pts[0];
        let proj: Vec<(f64, f64)> = pts
            .iter()
            .map(|p| {
                let r = *p - origin;
                (r.dot(x_ax), r.dot(y_ax))
            })
            .collect();
        // Shoelace.
        let mut area = 0.0_f64;
        for i in 0..n {
            let j = (i + 1) % n;
            area += proj[i].0 * proj[j].1 - proj[j].0 * proj[i].1;
        }
        area * 0.5
    }

    /// Reorient a wire so that its vertices appear CCW when viewed from
    /// `desired_normal`.  Returns a new `Wire` with points reversed if needed.
    pub fn orient_wire_ccw(wire: &Wire, desired_normal: Vec3) -> Wire {
        let normal = wire.normal();
        if normal.dot(desired_normal) < 0.0 {
            let mut pts = wire.pts.clone();
            pts.reverse();
            Wire::new(pts)
        } else {
            wire.clone()
        }
    }

    /// Check that a `Face` is valid: the outer wire must be planar, have at
    /// least 3 vertices, and each hole must also be planar on the same plane.
    pub fn face_is_valid(face: &Face, tol: f64) -> bool {
        if !face.outer.is_valid() {
            return false;
        }
        if BRepLib::wire_is_planar(&face.outer, tol).is_none() {
            return false;
        }
        let normal = face.outer.normal();
        let origin = face.outer.pts[0];
        for hole in &face.holes {
            if !hole.is_valid() {
                return false;
            }
            for p in &hole.pts {
                if (*p - origin).dot(normal).abs() > tol {
                    return false;
                }
            }
        }
        true
    }
}

// ─────────────────────────── BRepLib_MakeEdge ────────────────────────────────

/// Build a topological edge from an analytic curve (line segment or circular
/// arc), optionally trimmed by parameter values or endpoint points.
///
/// Mirrors `BRepLib_MakeEdge` from OCCT.  The resulting "edge" is represented
/// as a [`Wire`] whose `pts` hold the discretized sample points.
// occt: BRepLib_MakeEdge
pub struct BRepLibMakeEdge {
    result: Result<Wire, BRepLibError>,
}

/// Alias following OCCT naming conventions (`BRepLib_MakeEdge`).
pub type MakeEdge = BRepLibMakeEdge;

/// Number of interior sample points used when discretizing a circular arc.
const ARC_SAMPLES: usize = 64;

impl BRepLibMakeEdge {
    // ── constructors ──────────────────────────────────────────────────────────

    /// Build an edge that is a straight line segment from `p1` to `p2`.
    ///
    /// Mirrors `BRepLib_MakeEdge(const gp_Pnt&, const gp_Pnt&)`.
    pub fn from_two_points(p1: Pnt, p2: Pnt) -> Self {
        if p1.distance(p2) < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::CoincidentPoints),
            };
        }
        Self {
            result: Ok(Wire::new(vec![p1, p2])),
        }
    }

    /// Build an edge from a `gp_Lin` and a parameter range `[u1, u2]`.
    ///
    /// The line is parameterized as `P(u) = loc + u * dir`.
    ///
    /// Mirrors `BRepLib_MakeEdge(const gp_Lin&, const Standard_Real, const
    /// Standard_Real)`.
    pub fn from_lin_params(lin: Lin, u1: f64, u2: f64) -> Self {
        let (u_min, u_max) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
        if (u_max - u_min).abs() < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::ParameterOutOfRange),
            };
        }
        let loc = lin.location();
        let dir = lin.direction();
        let p1 = loc + dir * u_min;
        let p2 = loc + dir * u_max;
        if p1.distance(p2) < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::CoincidentPoints),
            };
        }
        Self {
            result: Ok(Wire::new(vec![p1, p2])),
        }
    }

    /// Build an edge from a `gp_Lin` trimmed to the segment `[p1, p2]`.
    ///
    /// The points are projected onto the line; the trimmed portion becomes the
    /// edge.
    ///
    /// Mirrors `BRepLib_MakeEdge(const gp_Lin&, const gp_Pnt&, const gp_Pnt&)`.
    pub fn from_lin_two_points(lin: Lin, p1: Pnt, p2: Pnt) -> Self {
        let loc = lin.location();
        let dir = lin.direction();
        // Project p1 and p2 onto the line.
        let u1 = (p1 - loc).dot(dir);
        let u2 = (p2 - loc).dot(dir);
        Self::from_lin_params(lin, u1, u2)
    }

    /// Build a full circle edge (`0..2π`).
    ///
    /// Mirrors `BRepLib_MakeEdge(const gp_Circ&)`.
    pub fn from_circ(circ: Circ) -> Self {
        Self::from_circ_params(circ, 0.0, TAU)
    }

    /// Build a circular arc edge for the parameter range `[u1, u2]` (radians).
    ///
    /// Mirrors `BRepLib_MakeEdge(const gp_Circ&, Standard_Real, Standard_Real)`.
    pub fn from_circ_params(circ: Circ, u1: f64, u2: f64) -> Self {
        let r = circ.radius();
        if r < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::NegativeRadius),
            };
        }
        // Normalise angle range to [0, 2π).
        let span = {
            let raw = u2 - u1;
            if raw.abs() < BRepLib::TOLERANCE {
                return Self {
                    result: Err(BRepLibError::ParameterOutOfRange),
                };
            }
            raw
        };
        let is_full = (span.abs() - TAU).abs() < BRepLib::TOLERANCE || span.abs() >= TAU;

        let n_pts = if is_full {
            ARC_SAMPLES
        } else {
            // Use at least 3 sample points for any arc.
            let frac = span.abs() / TAU;
            ((ARC_SAMPLES as f64 * frac).ceil() as usize).max(3)
        };

        let placement = circ.position();
        let pts: Vec<Pnt> = (0..=n_pts)
            .map(|i| {
                let t = if is_full && i == n_pts {
                    u1 // close the circle
                } else {
                    u1 + span * i as f64 / n_pts as f64
                };
                sample_circle(&placement, r, t)
            })
            .collect();

        let wire = if is_full {
            // Drop the repeated closing point – Wire::new deduplicates it.
            Wire::new(pts)
        } else {
            Wire::new(pts)
        };

        Self { result: Ok(wire) }
    }

    /// Build a circular arc from `p1` to `p2` on `circ` using the shorter arc.
    ///
    /// Mirrors `BRepLib_MakeEdge(const gp_Circ&, const gp_Pnt&, const gp_Pnt&)`.
    pub fn from_circ_two_points(circ: Circ, p1: Pnt, p2: Pnt) -> Self {
        let r = circ.radius();
        if r < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::NegativeRadius),
            };
        }
        let placement = circ.position();
        // Project p1 and p2 onto the circle plane and find their angles.
        let u1 = project_to_circle_angle(&placement, r, p1);
        let u2 = project_to_circle_angle(&placement, r, p2);
        let span = angle_span_ccw(u1, u2);
        if span.abs() < BRepLib::TOLERANCE || (span - TAU).abs() < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::ParameterOutOfRange),
            };
        }
        Self::from_circ_params(circ, u1, u1 + span)
    }

    // ── accessors ─────────────────────────────────────────────────────────────

    /// Returns `true` if the edge was built successfully.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the built edge (a discretized `Wire`).
    ///
    /// # Panics
    /// Panics if `!is_done()`.
    pub fn wire(&self) -> &Wire {
        self.result.as_ref().unwrap()
    }

    /// Return the result by value, consuming the builder.
    pub fn into_wire(self) -> Result<Wire, BRepLibError> {
        self.result
    }

    /// Mirrors OCCT's `BRepLib_MakeShape::IsDone()` + error accessor pattern.
    pub fn result(&self) -> Result<Wire, BRepLibError> {
        self.result.clone()
    }

    /// Return the start and end vertices of this edge.
    pub fn vertices(&self) -> Option<(Vertex, Vertex)> {
        let w = self.result.as_ref().ok()?;
        let start = *w.pts.first()?;
        let end = *w.pts.last()?;
        Some((Vertex(start), Vertex(end)))
    }
}

// ─────────────────────────── BRepLib_MakeWire ────────────────────────────────

/// Build a closed wire from one or more edges (each represented as a [`Wire`]
/// of sample points).  Edges are chained head-to-tail within a positional
/// tolerance; the final wire is closed.
///
/// Mirrors `BRepLib_MakeWire` from OCCT.
// occt: BRepLib_MakeWire
pub struct BRepLibMakeWire {
    pts: Vec<Pnt>,
    error: Option<BRepLibError>,
    tol: f64,
}

/// Alias following OCCT naming conventions (`BRepLib_MakeWire`).
pub type MakeWire = BRepLibMakeWire;

impl BRepLibMakeWire {
    /// Create an empty wire builder with the default connection tolerance.
    pub fn new() -> Self {
        Self {
            pts: Vec::new(),
            error: None,
            tol: BRepLib::TOLERANCE,
        }
    }

    /// Set the connection tolerance (default: [`BRepLib::TOLERANCE`]).
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tol = tol;
        self
    }

    /// Add an edge (a `Wire` of ordered sample points) to the wire being built.
    ///
    /// On the first call the edge is accepted as-is.  On subsequent calls the
    /// start or end of the new edge must be within `tol` of the current last
    /// accumulated point; if not, the edge is reversed before appending.  If
    /// neither endpoint connects, the builder sets an error.
    ///
    /// Mirrors `BRepLib_MakeWire::Add(const TopoDS_Edge&)`.
    pub fn add_edge(&mut self, edge: &Wire) {
        if self.error.is_some() {
            return;
        }
        let edge_pts = &edge.pts;
        if edge_pts.is_empty() {
            self.error = Some(BRepLibError::InvalidWire);
            return;
        }
        if self.pts.is_empty() {
            // First edge: append all sample points.
            self.pts.extend_from_slice(edge_pts);
            return;
        }

        let last = *self.pts.last().unwrap();
        let e_start = edge_pts[0];
        let e_end = *edge_pts.last().unwrap();

        if last.distance(e_start) <= self.tol {
            // Forward connection: skip the duplicate connector point.
            self.pts.extend_from_slice(&edge_pts[1..]);
        } else if last.distance(e_end) <= self.tol {
            // Reverse the edge and skip the duplicate connector.
            let reversed: Vec<Pnt> = edge_pts.iter().rev().copied().collect();
            self.pts.extend_from_slice(&reversed[1..]);
        } else {
            // Snap the gap if it's within a relaxed tolerance (10×).
            let relaxed = self.tol * 10.0;
            if last.distance(e_start) <= relaxed {
                self.pts.extend_from_slice(&edge_pts[1..]);
            } else if last.distance(e_end) <= relaxed {
                let reversed: Vec<Pnt> = edge_pts.iter().rev().copied().collect();
                self.pts.extend_from_slice(&reversed[1..]);
            } else {
                self.error = Some(BRepLibError::ConstructionError);
            }
        }
    }

    /// Convenience: add multiple edges in order.
    pub fn add_edges(&mut self, edges: &[Wire]) {
        for e in edges {
            self.add_edge(e);
        }
    }

    /// Build the final closed `Wire`.
    ///
    /// Requires at least 3 distinct points; the wire is automatically closed
    /// (if the last point coincides with the first it is deduplicated by
    /// [`Wire::new`]).
    pub fn wire(&self) -> Result<Wire, BRepLibError> {
        if let Some(e) = self.error {
            return Err(e);
        }
        if self.pts.len() < 3 {
            return Err(BRepLibError::InvalidWire);
        }
        Ok(Wire::new(self.pts.clone()))
    }

    /// Returns `true` if no error has been recorded yet.
    pub fn is_done(&self) -> bool {
        self.error.is_none() && self.pts.len() >= 3
    }
}

impl Default for BRepLibMakeWire {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── BRepLib_MakeFace ────────────────────────────────

/// Build a planar topological face from a wire (outer boundary) plus optional
/// inner wires (holes).
///
/// Mirrors `BRepLib_MakeFace` from OCCT.
// occt: BRepLib_MakeFace
pub struct BRepLibMakeFace {
    result: Result<Face, BRepLibError>,
}

/// Alias following OCCT naming conventions (`BRepLib_MakeFace`).
pub type MakeFace = BRepLibMakeFace;

impl BRepLibMakeFace {
    // ── constructors ──────────────────────────────────────────────────────────

    /// Build a planar face from the outer wire.
    ///
    /// Mirrors `BRepLib_MakeFace(const TopoDS_Wire&)`.
    pub fn from_wire(wire: Wire) -> Self {
        Self::from_wire_tol(wire, BRepLib::TOLERANCE)
    }

    /// Build a planar face from the outer wire with an explicit planarity
    /// tolerance.
    pub fn from_wire_tol(wire: Wire, tol: f64) -> Self {
        if !wire.is_valid() {
            return Self {
                result: Err(BRepLibError::InvalidWire),
            };
        }
        if BRepLib::wire_is_planar(&wire, tol).is_none() {
            return Self {
                result: Err(BRepLibError::NonPlanarWire),
            };
        }
        Self {
            result: Ok(Face::new(wire)),
        }
    }

    /// Build a face from the outer wire plus a list of hole wires.
    ///
    /// Mirrors `BRepLib_MakeFace(wire).Add(hole_wire)` idiom.
    pub fn from_wire_with_holes(outer: Wire, holes: Vec<Wire>) -> Self {
        if !outer.is_valid() {
            return Self {
                result: Err(BRepLibError::InvalidWire),
            };
        }
        if BRepLib::wire_is_planar(&outer, BRepLib::TOLERANCE).is_none() {
            return Self {
                result: Err(BRepLibError::NonPlanarWire),
            };
        }
        for h in &holes {
            if !h.is_valid() {
                return Self {
                    result: Err(BRepLibError::InvalidWire),
                };
            }
        }
        Self {
            result: Ok(Face::with_holes(outer, holes)),
        }
    }

    /// Build a rectangular face in the XY plane of `placement`, spanning
    /// `[u_min, u_max] × [v_min, v_max]`.
    ///
    /// Mirrors `BRepLib_MakeFace(const gp_Pln&, u1, u2, v1, v2)`.
    pub fn from_plane_bounds(placement: Ax3, u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        if u_max - u_min < BRepLib::TOLERANCE || v_max - v_min < BRepLib::TOLERANCE {
            return Self {
                result: Err(BRepLibError::ParameterOutOfRange),
            };
        }
        // Corner points of the rectangle in world coordinates.
        let p00 = placement.to_world(u_min, v_min, 0.0);
        let p10 = placement.to_world(u_max, v_min, 0.0);
        let p11 = placement.to_world(u_max, v_max, 0.0);
        let p01 = placement.to_world(u_min, v_max, 0.0);
        let wire = Wire::new(vec![p00, p10, p11, p01]);
        Self {
            result: Ok(Face::new(wire)),
        }
    }

    /// Build a planar disk face from a circle (the planar region enclosed by
    /// the circle boundary).
    ///
    /// Mirrors `BRepLib_MakeFace(const gp_Circ&)`.
    pub fn from_circle(circ: Circ) -> Self {
        let edge = BRepLibMakeEdge::from_circ(circ);
        match edge.into_wire() {
            Ok(wire) => Self {
                result: Ok(Face::new(wire)),
            },
            Err(e) => Self { result: Err(e) },
        }
    }

    /// Add a hole to an existing face (builder-style).
    ///
    /// Mirrors `BRepLib_MakeFace::Add(const TopoDS_Wire&)`.
    pub fn add_hole(mut self, hole: Wire) -> Self {
        if let Ok(ref mut face) = self.result {
            face.holes.push(hole);
        }
        self
    }

    // ── accessors ─────────────────────────────────────────────────────────────

    /// Returns `true` if the face was built successfully.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns a reference to the built face.
    ///
    /// # Panics
    /// Panics if `!is_done()`.
    pub fn face(&self) -> &Face {
        self.result.as_ref().unwrap()
    }

    /// Consume the builder and return the face, or an error.
    pub fn into_face(self) -> Result<Face, BRepLibError> {
        self.result
    }

    /// Mirrors OCCT's two-step `IsDone()` + `Shape()` accessor pattern.
    pub fn result(&self) -> Result<Face, BRepLibError> {
        self.result.clone()
    }

    /// The plane normal of the face (derived from the outer wire).
    pub fn normal(&self) -> Option<Vec3> {
        self.result.as_ref().ok().map(|f| f.normal())
    }

    /// Triangulate the face (delegates to `topods::Face::triangulate`).
    pub fn triangulate(&self) -> Option<Vec<[Pnt; 3]>> {
        self.result.as_ref().ok().map(|f| f.triangulate())
    }
}

// ─────────────────────────── free helpers ────────────────────────────────────

/// Sample the point on a circle (defined by `placement` and `radius`) at
/// parameter `u` (radians).
///
/// `P(u) = center + r * cos(u) * x_dir + r * sin(u) * y_dir`
fn sample_circle(placement: &Ax3, radius: f64, u: f64) -> Pnt {
    let (c, s) = (u.cos(), u.sin());
    placement.location + placement.x_dir * (radius * c) + placement.y_dir * (radius * s)
}

/// Project a world point onto the circle plane and return its angle in
/// `[0, 2π)`.
fn project_to_circle_angle(placement: &Ax3, _radius: f64, p: Pnt) -> f64 {
    let rel = p - placement.location;
    let lx = rel.dot(placement.x_dir);
    let ly = rel.dot(placement.y_dir);
    let a = ly.atan2(lx);
    if a < 0.0 {
        a + TAU
    } else {
        a
    }
}

/// Return the CCW angular span from angle `a` to angle `b`, always in `(0, 2π]`.
fn angle_span_ccw(a: f64, b: f64) -> f64 {
    let span = (b - a).rem_euclid(TAU);
    if span < EPS {
        TAU
    } else {
        span
    }
}

// ─────────────────────────── unit tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Pnt};
    use crate::gp_prim::{Circ, Lin};
    use std::f64::consts::PI;

    const TOL: f64 = 1e-9;

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ≈ {b} within {tol}, diff={}",
            (a - b).abs()
        );
    }

    // ── BRepLib utility ───────────────────────────────────────────────────────

    #[test]
    fn brep_lib_wire_is_planar_xy_plane() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let n = BRepLib::wire_is_planar(&w, 1e-7);
        assert!(n.is_some(), "square in XY plane must be planar");
        let n = n.unwrap();
        // Normal should be near +Z or -Z.
        assert!(n.z.abs() > 0.9, "normal should be along Z, got {:?}", n);
    }

    #[test]
    fn brep_lib_wire_is_planar_non_planar() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 1.0), // lifted off XY
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        // With a tight tolerance the non-planar point should fail.
        let n = BRepLib::wire_is_planar(&w, 1e-7);
        assert!(n.is_none(), "non-planar wire should return None");
    }

    #[test]
    fn brep_lib_wire_area_unit_square() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let area = BRepLib::wire_area(&w).abs();
        near(area, 1.0, 1e-9);
    }

    #[test]
    fn brep_lib_orient_wire_ccw() {
        // CW square in XY plane.
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ]);
        let desired = Pnt::new(0.0, 0.0, 1.0); // +Z normal
        let oriented = BRepLib::orient_wire_ccw(&w, desired);
        // After orientation the first point should be the old last point.
        let area = BRepLib::wire_area(&oriented);
        assert!(area > 0.0, "oriented wire must have positive area, got {area}");
    }

    // ── BRepLib_MakeEdge ──────────────────────────────────────────────────────

    #[test]
    fn make_edge_two_points_segment() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 4.0, 0.0);
        let e = BRepLibMakeEdge::from_two_points(p1, p2);
        assert!(e.is_done());
        let w = e.wire();
        assert_eq!(w.pts.len(), 2);
        near(w.pts[0].distance(p1), 0.0, TOL);
        near(w.pts[1].distance(p2), 0.0, TOL);
    }

    #[test]
    fn make_edge_coincident_points_error() {
        let p = Pnt::new(1.0, 2.0, 3.0);
        let e = BRepLibMakeEdge::from_two_points(p, p);
        assert!(!e.is_done());
        assert_eq!(e.result().unwrap_err(), BRepLibError::CoincidentPoints);
    }

    #[test]
    fn make_edge_from_lin_params_length() {
        let lin = Lin::new_pd(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let e = BRepLibMakeEdge::from_lin_params(lin, 1.0, 4.0);
        assert!(e.is_done());
        let w = e.wire();
        near(w.pts[0].x, 1.0, TOL);
        near(w.pts[1].x, 4.0, TOL);
    }

    #[test]
    fn make_edge_full_circle_closes() {
        let ax3 = Ax3::identity();
        let circ = Circ::new(ax3, 1.0);
        let e = BRepLibMakeEdge::from_circ(circ);
        assert!(e.is_done());
        let w = e.wire();
        // A closed circle wire should have first ≈ last (Wire::new deduplicates).
        // All sample points should be at distance 1 from origin.
        for p in &w.pts {
            near(p.distance(Pnt::origin()), 1.0, 1e-12);
        }
    }

    #[test]
    fn make_edge_quarter_arc_endpoint() {
        let ax3 = Ax3::identity();
        let circ = Circ::new(ax3, 2.0);
        // Quarter arc from 0 to π/2.
        let e = BRepLibMakeEdge::from_circ_params(circ, 0.0, PI / 2.0);
        assert!(e.is_done());
        let w = e.wire();
        let start = w.pts[0];
        let end = *w.pts.last().unwrap();
        near(start.distance(Pnt::new(2.0, 0.0, 0.0)), 0.0, 1e-10);
        near(end.distance(Pnt::new(0.0, 2.0, 0.0)), 0.0, 1e-10);
    }

    // ── BRepLib_MakeWire ──────────────────────────────────────────────────────

    #[test]
    fn make_wire_from_three_edges_forms_triangle() {
        // Three edges forming a right triangle.
        let e1 = BRepLibMakeEdge::from_two_points(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        )
        .into_wire()
        .unwrap();
        let e2 = BRepLibMakeEdge::from_two_points(
            Pnt::new(3.0, 0.0, 0.0),
            Pnt::new(0.0, 4.0, 0.0),
        )
        .into_wire()
        .unwrap();
        let e3 = BRepLibMakeEdge::from_two_points(
            Pnt::new(0.0, 4.0, 0.0),
            Pnt::new(0.0, 0.0, 0.0),
        )
        .into_wire()
        .unwrap();

        let mut builder = BRepLibMakeWire::new();
        builder.add_edges(&[e1, e2, e3]);
        let wire = builder.wire().expect("triangle wire must build");
        assert!(wire.is_valid(), "triangle wire must be valid");
    }

    #[test]
    fn make_wire_disconnected_edges_error() {
        let e1 = BRepLibMakeEdge::from_two_points(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        )
        .into_wire()
        .unwrap();
        // This edge starts far from where e1 ends.
        let e2 = BRepLibMakeEdge::from_two_points(
            Pnt::new(100.0, 0.0, 0.0),
            Pnt::new(101.0, 0.0, 0.0),
        )
        .into_wire()
        .unwrap();

        let mut builder = BRepLibMakeWire::new();
        builder.add_edge(&e1);
        builder.add_edge(&e2);
        assert!(!builder.is_done(), "disconnected edges must produce an error");
    }

    // ── BRepLib_MakeFace ──────────────────────────────────────────────────────

    #[test]
    fn make_face_from_wire_is_valid() {
        let wire = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let mf = BRepLibMakeFace::from_wire(wire);
        assert!(mf.is_done());
        assert!(BRepLib::face_is_valid(mf.face(), 1e-7));
    }

    #[test]
    fn make_face_from_plane_bounds_corner_points() {
        let placement = Ax3::identity();
        let mf = BRepLibMakeFace::from_plane_bounds(placement, -2.0, 2.0, -3.0, 3.0);
        assert!(mf.is_done());
        let face = mf.face();
        // All 4 corner points should be on the Z=0 plane.
        for p in &face.outer.pts {
            near(p.z, 0.0, TOL);
        }
    }

    #[test]
    fn make_face_from_circle_all_points_at_radius() {
        let ax3 = Ax3::identity();
        let circ = Circ::new(ax3, 5.0);
        let mf = BRepLibMakeFace::from_circle(circ);
        assert!(mf.is_done());
        for p in &mf.face().outer.pts {
            near(p.distance(Pnt::origin()), 5.0, 1e-10);
        }
    }

    #[test]
    fn make_face_non_planar_wire_error() {
        let wire = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 1.0), // off-plane
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let mf = BRepLibMakeFace::from_wire_tol(wire, 1e-7);
        assert!(!mf.is_done());
        assert_eq!(
            mf.into_face().unwrap_err(),
            BRepLibError::NonPlanarWire
        );
    }

    #[test]
    fn make_face_with_hole() {
        let outer = Wire::new(vec![
            Pnt::new(-2.0, -2.0, 0.0),
            Pnt::new(2.0, -2.0, 0.0),
            Pnt::new(2.0, 2.0, 0.0),
            Pnt::new(-2.0, 2.0, 0.0),
        ]);
        let hole = Wire::new(vec![
            Pnt::new(-0.5, -0.5, 0.0),
            Pnt::new(0.5, -0.5, 0.0),
            Pnt::new(0.5, 0.5, 0.0),
            Pnt::new(-0.5, 0.5, 0.0),
        ]);
        let mf = BRepLibMakeFace::from_wire_with_holes(outer, vec![hole]);
        assert!(mf.is_done());
        assert_eq!(mf.face().holes.len(), 1);
    }
}
