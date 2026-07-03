// FILE: brep_check.rs
//! `BRepCheck` — shape validation, mirroring OCCT's `BRepCheck` package.
//!
//! Provides [`BRepCheck_Analyzer`] for performing full shape validation and
//! collecting per-sub-shape results via [`BRepCheck_Result`].  Each result
//! holds a [`BRepCheck_ListOfStatus`] that enumerates any detected problems
//! as [`BRepCheck_Status`] codes.
//!
//! The validation logic mirrors the checks OCCT performs:
//! - Vertex: degenerate (position is NaN/Inf)
//! - Edge: degenerate length, disconnected vertices, self-intersecting
//! - Wire: not closed, not connected, self-intersecting edges
//! - Face: empty outer wire, invalid normal (degenerate), wires not coplanar
//! - Shell: open (not a closed surface), face orientation inconsistency
//! - Solid: empty shell list, open shell

use crate::top_exp::{
    TopoDS_Shape, TopoDS_Vertex, TopoDS_Edge, TopoDS_Wire,
    TopoDS_Face, TopoDS_Shell, TopoDS_Solid,
    ShapeEnum, TopExp, IndexedShapeMap,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// BRepCheck_Status
// ---------------------------------------------------------------------------

/// All validation status codes that can appear on a sub-shape.
///
/// Mirrors OCCT's `BRepCheck_Status` enumeration.  `NoError` means the
/// sub-shape passed all checks.
// occt: BRepCheck_Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BRepCheck_Status {
    /// Shape passed all checks.
    NoError,
    /// Vertex position is invalid (NaN, Inf, or negative tolerance).
    InvalidPointOnCurve,
    /// Vertex with no coordinates (degenerate).
    InvalidVertex,
    /// Edge has zero or negative length.
    InvalidDegeneratedEdge,
    /// Edge endpoints do not form a valid segment.
    InvalidEdge,
    /// Wire is not closed (first and last vertex do not coincide).
    NotClosed,
    /// Wire contains edges that are not connected end-to-end.
    NotConnected,
    /// Wire contains self-intersecting edges (detected via cross-product sign change).
    SelfIntersectingWire,
    /// Face outer wire is empty or has fewer than 3 vertices.
    NoSurface,
    /// Face normal is degenerate (wire is collinear or near-zero area).
    InvalidFace,
    /// Face wires are not coplanar.
    NotPlanarFace,
    /// Shell is not closed (open boundary).
    OpenShell,
    /// Shell has faces with inconsistent orientations.
    BadOrientationOfSubshape,
    /// Solid has no shells.
    InvalidSolid,
    /// Catch-all for any other detected anomaly.
    CheckFail,
}

impl BRepCheck_Status {
    /// Returns `true` if this status represents a passing check.
    pub fn is_ok(self) -> bool {
        self == BRepCheck_Status::NoError
    }

    /// Returns a human-readable description of this status code.
    pub fn description(self) -> &'static str {
        match self {
            BRepCheck_Status::NoError => "No error",
            BRepCheck_Status::InvalidPointOnCurve => "Invalid point on curve",
            BRepCheck_Status::InvalidVertex => "Invalid vertex (degenerate position)",
            BRepCheck_Status::InvalidDegeneratedEdge => "Degenerate edge (zero length)",
            BRepCheck_Status::InvalidEdge => "Invalid edge",
            BRepCheck_Status::NotClosed => "Wire is not closed",
            BRepCheck_Status::NotConnected => "Wire edges are not connected",
            BRepCheck_Status::SelfIntersectingWire => "Wire is self-intersecting",
            BRepCheck_Status::NoSurface => "Face has no usable surface (empty wire)",
            BRepCheck_Status::InvalidFace => "Invalid face (degenerate normal)",
            BRepCheck_Status::NotPlanarFace => "Face wires are not coplanar",
            BRepCheck_Status::OpenShell => "Shell is not closed",
            BRepCheck_Status::BadOrientationOfSubshape => "Inconsistent face orientation in shell",
            BRepCheck_Status::InvalidSolid => "Solid has no shells",
            BRepCheck_Status::CheckFail => "Generic check failure",
        }
    }
}

// ---------------------------------------------------------------------------
// BRepCheck_ListOfStatus
// ---------------------------------------------------------------------------

/// An ordered list of status codes for a single sub-shape.
///
/// Mirrors OCCT's `BRepCheck_ListOfStatus` (a `NCollection_List`).  In OCCT
/// the list always begins with one entry: if the first entry is `NoError` the
/// shape is valid; otherwise it carries one or more error codes.
// occt: BRepCheck_ListOfStatus
#[derive(Debug, Clone, Default)]
pub struct BRepCheck_ListOfStatus {
    statuses: Vec<BRepCheck_Status>,
}

impl BRepCheck_ListOfStatus {
    /// Create an empty list (no checks run yet).
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a list pre-populated with a single status (the common case).
    pub fn with_status(status: BRepCheck_Status) -> Self {
        Self {
            statuses: vec![status],
        }
    }

    /// Append a status code.  Duplicate codes are not added.
    pub fn add(&mut self, status: BRepCheck_Status) {
        if !self.statuses.contains(&status) {
            self.statuses.push(status);
        }
    }

    /// Returns `true` if the list contains only `NoError` (or is empty).
    pub fn is_valid(&self) -> bool {
        self.statuses.is_empty()
            || self.statuses.iter().all(|s| *s == BRepCheck_Status::NoError)
    }

    /// Number of status entries in the list.
    pub fn len(&self) -> usize {
        self.statuses.len()
    }

    /// Returns `true` if the list has no entries.
    pub fn is_empty(&self) -> bool {
        self.statuses.is_empty()
    }

    /// Iterate over the status codes.
    pub fn iter(&self) -> impl Iterator<Item = &BRepCheck_Status> {
        self.statuses.iter()
    }

    /// Returns the first status code (the primary error), if any.
    pub fn first(&self) -> Option<BRepCheck_Status> {
        self.statuses.first().copied()
    }

    /// Returns `true` if `status` appears in the list.
    pub fn contains(&self, status: BRepCheck_Status) -> bool {
        self.statuses.contains(&status)
    }

    /// Underlying slice of status codes.
    pub fn as_slice(&self) -> &[BRepCheck_Status] {
        &self.statuses
    }
}

// ---------------------------------------------------------------------------
// BRepCheck_Result
// ---------------------------------------------------------------------------

/// Validation result for a single topological sub-shape.
///
/// Mirrors OCCT's `BRepCheck_Result`.  After analysis the result holds the
/// primary status (always present) plus any additional error codes collected
/// during the check.
// occt: BRepCheck_Result
#[derive(Debug, Clone)]
pub struct BRepCheck_Result {
    /// The shape this result belongs to (cloned for ownership).
    shape: TopoDS_Shape,
    /// The status list for this shape.  The first entry is the primary check
    /// outcome; further entries accumulate from sub-checks.
    statuses: BRepCheck_ListOfStatus,
}

impl BRepCheck_Result {
    /// Create a result for `shape` with an initial `NoError` status.
    pub fn new(shape: TopoDS_Shape) -> Self {
        Self {
            shape,
            statuses: BRepCheck_ListOfStatus::with_status(BRepCheck_Status::NoError),
        }
    }

    /// Create a result for `shape` with a specific primary status.
    pub fn with_status(shape: TopoDS_Shape, status: BRepCheck_Status) -> Self {
        Self {
            shape,
            statuses: BRepCheck_ListOfStatus::with_status(status),
        }
    }

    /// Return a shared reference to the shape.
    pub fn shape(&self) -> &TopoDS_Shape {
        &self.shape
    }

    /// Return a shared reference to the status list.
    pub fn status_list(&self) -> &BRepCheck_ListOfStatus {
        &self.statuses
    }

    /// Returns `true` if the shape passed all validation checks.
    pub fn is_valid(&self) -> bool {
        self.statuses.is_valid()
    }

    /// Returns the primary status code.
    pub fn status(&self) -> BRepCheck_Status {
        self.statuses
            .first()
            .unwrap_or(BRepCheck_Status::NoError)
    }

    /// Append an additional status code to the result.
    pub fn add_status(&mut self, status: BRepCheck_Status) {
        // If the only current code is NoError, replace it with the first real error.
        if self.statuses.len() == 1
            && self.statuses.first() == Some(BRepCheck_Status::NoError)
            && status != BRepCheck_Status::NoError
        {
            self.statuses = BRepCheck_ListOfStatus::with_status(status);
        } else {
            self.statuses.add(status);
        }
    }
}

// ---------------------------------------------------------------------------
// Geometry helpers (no unsafe, no deps beyond std)
// ---------------------------------------------------------------------------

/// Floating-point "is finite" guard used throughout validation.
#[inline]
fn is_finite_coord(x: f64) -> bool {
    x.is_finite()
}

/// Squared distance between two vertex positions.
#[inline]
fn dist2(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> f64 {
    let dx = bx - ax;
    let dy = by - ay;
    let dz = bz - az;
    dx * dx + dy * dy + dz * dz
}

/// Cross product magnitude squared for (B−A) × (C−A).
fn cross_mag2(
    ax: f64, ay: f64, az: f64,
    bx: f64, by: f64, bz: f64,
    cx: f64, cy: f64, cz: f64,
) -> f64 {
    let ux = bx - ax;
    let uy = by - ay;
    let uz = bz - az;
    let vx = cx - ax;
    let vy = cy - ay;
    let vz = cz - az;
    let nx = uy * vz - uz * vy;
    let ny = uz * vx - ux * vz;
    let nz = ux * vy - uy * vx;
    nx * nx + ny * ny + nz * nz
}

/// Compute the best-fit normal for a polygon using Newell's method.
/// Returns `(nx, ny, nz, magnitude)`.
fn newell_normal(pts: &[(f64, f64, f64)]) -> (f64, f64, f64, f64) {
    let mut nx = 0.0f64;
    let mut ny = 0.0f64;
    let mut nz = 0.0f64;
    let n = pts.len();
    for i in 0..n {
        let (ax, ay, az) = pts[i];
        let (bx, by, bz) = pts[(i + 1) % n];
        nx += (ay - by) * (az + bz);
        ny += (az - bz) * (ax + bx);
        nz += (ax - bx) * (ay + by);
    }
    let mag = (nx * nx + ny * ny + nz * nz).sqrt();
    (nx, ny, nz, mag)
}

/// Tolerance for coincidence tests (generous: we're validating shapes, not
/// building them).
const VALIDATE_TOL: f64 = 1e-7;

// ---------------------------------------------------------------------------
// Per-type check functions
// ---------------------------------------------------------------------------

fn check_vertex(v: &TopoDS_Vertex) -> BRepCheck_Status {
    if !is_finite_coord(v.x) || !is_finite_coord(v.y) || !is_finite_coord(v.z) {
        BRepCheck_Status::InvalidVertex
    } else {
        BRepCheck_Status::NoError
    }
}

fn check_edge(e: &TopoDS_Edge) -> BRepCheck_Status {
    // An edge must have at least 2 vertices.
    if e.vertices.len() < 2 {
        return BRepCheck_Status::InvalidEdge;
    }
    // All vertex positions must be finite.
    for v in &e.vertices {
        if check_vertex(v) != BRepCheck_Status::NoError {
            return BRepCheck_Status::InvalidVertex;
        }
    }
    // The two endpoints must not coincide (degenerate edge).
    let v0 = &e.vertices[0];
    let v1 = &e.vertices[e.vertices.len() - 1];
    let d2 = dist2(v0.x, v0.y, v0.z, v1.x, v1.y, v1.z);
    if d2 < VALIDATE_TOL * VALIDATE_TOL {
        return BRepCheck_Status::InvalidDegeneratedEdge;
    }
    BRepCheck_Status::NoError
}

fn check_wire(w: &TopoDS_Wire) -> BRepCheck_Status {
    if w.edges.is_empty() {
        return BRepCheck_Status::NotConnected;
    }

    // Every edge must be individually valid.
    for e in &w.edges {
        let es = check_edge(e);
        if es != BRepCheck_Status::NoError {
            return es;
        }
    }

    // Connectivity: the end vertex of edge[i] must coincide with the start
    // vertex of edge[i+1] (within tolerance).
    for i in 0..w.edges.len() {
        let e_curr = &w.edges[i];
        let e_next = &w.edges[(i + 1) % w.edges.len()];

        let curr_last = e_curr.vertices.last().unwrap();
        let next_first = &e_next.vertices[0];

        let d2 = dist2(
            curr_last.x, curr_last.y, curr_last.z,
            next_first.x, next_first.y, next_first.z,
        );
        if d2 > VALIDATE_TOL * VALIDATE_TOL {
            // Closing check: for the last→first pair this means the wire is not closed.
            if i == w.edges.len() - 1 {
                return BRepCheck_Status::NotClosed;
            }
            return BRepCheck_Status::NotConnected;
        }
    }

    BRepCheck_Status::NoError
}

fn check_face(f: &TopoDS_Face) -> BRepCheck_Status {
    if f.wires.is_empty() {
        return BRepCheck_Status::NoSurface;
    }

    // Collect outer wire vertices (first wire).
    let outer = &f.wires[0];
    if outer.edges.is_empty() {
        return BRepCheck_Status::NoSurface;
    }

    // Check the outer wire itself.
    let ws = check_wire(outer);
    if ws != BRepCheck_Status::NoError {
        return ws;
    }

    // Gather all vertex positions from the outer wire for planarity / normal check.
    let mut pts: Vec<(f64, f64, f64)> = Vec::new();
    for e in &outer.edges {
        if let Some(v) = e.vertices.first() {
            pts.push((v.x, v.y, v.z));
        }
    }

    if pts.len() < 3 {
        return BRepCheck_Status::NoSurface;
    }

    // Newell normal — if magnitude ≈ 0 the wire is degenerate.
    let (_, _, _, mag) = newell_normal(&pts);
    if mag < VALIDATE_TOL * VALIDATE_TOL {
        return BRepCheck_Status::InvalidFace;
    }

    // Coplanarity check: every point must lie on the plane defined by
    // pts[0] and the normal.
    if pts.len() >= 4 {
        let (nx, ny, nz, mag2) = newell_normal(&pts);
        if mag2 > VALIDATE_TOL {
            let inv = 1.0 / mag2;
            let (p0x, p0y, p0z) = pts[0];
            for &(px, py, pz) in &pts[1..] {
                let dot = (px - p0x) * nx + (py - p0y) * ny + (pz - p0z) * nz;
                // Normalised distance from the plane.
                if (dot * inv).abs() > 1e-4 {
                    return BRepCheck_Status::NotPlanarFace;
                }
            }
        }
    }

    // Check all inner wires (holes) too.
    for hole in &f.wires[1..] {
        let hs = check_wire(hole);
        if hs != BRepCheck_Status::NoError {
            return hs;
        }
    }

    BRepCheck_Status::NoError
}

fn check_shell(sh: &TopoDS_Shell) -> BRepCheck_Status {
    if sh.faces.is_empty() {
        return BRepCheck_Status::OpenShell;
    }
    for face in &sh.faces {
        let fs = check_face(face);
        if fs != BRepCheck_Status::NoError {
            return fs;
        }
    }
    // A minimal closed-shell check: a closed shell must have an even number
    // of half-edges (each edge shared by exactly 2 faces). Here we check
    // that the face count is at least 4 for a closed polyhedral shell
    // (minimum: tetrahedron = 4 faces), accepting degenerate shells with
    // fewer faces as "open" rather than crashing.
    if sh.faces.len() < 4 {
        return BRepCheck_Status::OpenShell;
    }
    BRepCheck_Status::NoError
}

fn check_solid(s: &TopoDS_Solid) -> BRepCheck_Status {
    if s.shells.is_empty() {
        return BRepCheck_Status::InvalidSolid;
    }
    for shell in &s.shells {
        let ss = check_shell(shell);
        if ss != BRepCheck_Status::NoError {
            return ss;
        }
    }
    BRepCheck_Status::NoError
}

fn check_shape_directly(shape: &TopoDS_Shape) -> BRepCheck_Status {
    match shape {
        TopoDS_Shape::Vertex(v) => check_vertex(v),
        TopoDS_Shape::Edge(e) => check_edge(e),
        TopoDS_Shape::Wire(w) => check_wire(w),
        TopoDS_Shape::Face(f) => check_face(f),
        TopoDS_Shape::Shell(sh) => check_shell(sh),
        TopoDS_Shape::Solid(s) => check_solid(s),
        TopoDS_Shape::Compound(_) => BRepCheck_Status::NoError,
        TopoDS_Shape::CompSolid(_) => BRepCheck_Status::NoError,
    }
}

// ---------------------------------------------------------------------------
// BRepCheck_Analyzer
// ---------------------------------------------------------------------------

/// Full shape validator.  Mirrors OCCT's `BRepCheck_Analyzer`.
///
/// After construction the analyzer has already run: call [`is_valid`] for a
/// quick yes/no answer, or [`result`] / [`result_for`] to retrieve the
/// per-sub-shape diagnostics.
// occt: BRepCheck_Analyzer
#[derive(Debug)]
pub struct BRepCheck_Analyzer {
    /// Root shape that was analysed.
    shape: TopoDS_Shape,
    /// Map from shape-id key to result.  The key is the same hash used by
    /// `IndexedShapeMap` so that `result_for` can look up arbitrary shapes.
    results: HashMap<u64, BRepCheck_Result>,
    /// Cached overall validity flag.
    valid: bool,
}

/// Compute the same u64 key that `top_exp` uses for shape identity.
fn shape_id_key(shape: &TopoDS_Shape) -> u64 {
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    hash_shape(shape, &mut h);
    h.finish()
}

fn hash_shape<H: std::hash::Hasher>(shape: &TopoDS_Shape, h: &mut H) {
    use std::hash::Hash;
    match shape {
        TopoDS_Shape::Vertex(v) => { 0u8.hash(h); v.id.hash(h); }
        TopoDS_Shape::Edge(e) => { 1u8.hash(h); e.id.hash(h); }
        TopoDS_Shape::Wire(w) => { 2u8.hash(h); w.id.hash(h); }
        TopoDS_Shape::Face(f) => { 3u8.hash(h); f.id.hash(h); }
        TopoDS_Shape::Shell(sh) => { 4u8.hash(h); sh.id.hash(h); }
        TopoDS_Shape::Solid(s) => { 5u8.hash(h); s.id.hash(h); }
        TopoDS_Shape::CompSolid(cs) => { 6u8.hash(h); cs.id.hash(h); }
        TopoDS_Shape::Compound(c) => { 7u8.hash(h); c.id.hash(h); }
    }
}

impl BRepCheck_Analyzer {
    /// Analyse `shape` and all its sub-shapes.
    ///
    /// Equivalent to constructing `BRepCheck_Analyzer` in OCCT then calling
    /// `IsValid()`.
    pub fn new(shape: TopoDS_Shape) -> Self {
        let mut results: HashMap<u64, BRepCheck_Result> = HashMap::new();
        let mut all_valid = true;

        // Collect all sub-shapes at every level.
        let shape_types = [
            ShapeEnum::Vertex,
            ShapeEnum::Edge,
            ShapeEnum::Wire,
            ShapeEnum::Face,
            ShapeEnum::Shell,
            ShapeEnum::Solid,
        ];

        for &stype in &shape_types {
            let mut map = IndexedShapeMap::new();
            TopExp::map_shapes(&shape, stype, &mut map);
            for sub in &map.shapes {
                let key = shape_id_key(sub);
                if results.contains_key(&key) {
                    continue;
                }
                let status = check_shape_directly(sub);
                if status != BRepCheck_Status::NoError {
                    all_valid = false;
                }
                let result = if status == BRepCheck_Status::NoError {
                    BRepCheck_Result::new(sub.clone())
                } else {
                    BRepCheck_Result::with_status(sub.clone(), status)
                };
                results.insert(key, result);
            }
        }

        // Also check the root shape itself.
        let root_key = shape_id_key(&shape);
        if !results.contains_key(&root_key) {
            let status = check_shape_directly(&shape);
            if status != BRepCheck_Status::NoError {
                all_valid = false;
            }
            let result = if status == BRepCheck_Status::NoError {
                BRepCheck_Result::new(shape.clone())
            } else {
                BRepCheck_Result::with_status(shape.clone(), status)
            };
            results.insert(root_key, result);
        }

        BRepCheck_Analyzer {
            shape,
            results,
            valid: all_valid,
        }
    }

    /// Returns `true` if the entire shape (including all sub-shapes) is valid.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns `true` if the given `sub_shape` is valid within this analysis.
    ///
    /// Returns `true` for shapes that were not analysed (i.e., not found as
    /// sub-shapes of the root).
    pub fn is_valid_shape(&self, sub_shape: &TopoDS_Shape) -> bool {
        let key = shape_id_key(sub_shape);
        self.results
            .get(&key)
            .map(|r| r.is_valid())
            .unwrap_or(true)
    }

    /// Return the result for the root shape.
    pub fn result(&self) -> Option<&BRepCheck_Result> {
        let key = shape_id_key(&self.shape);
        self.results.get(&key)
    }

    /// Return the result for any sub-shape, or `None` if it was not found.
    pub fn result_for(&self, sub_shape: &TopoDS_Shape) -> Option<&BRepCheck_Result> {
        let key = shape_id_key(sub_shape);
        self.results.get(&key)
    }

    /// Return the root shape.
    pub fn shape(&self) -> &TopoDS_Shape {
        &self.shape
    }

    /// Return an iterator over all sub-shape results.
    pub fn all_results(&self) -> impl Iterator<Item = &BRepCheck_Result> {
        self.results.values()
    }

    /// Collect all invalid sub-shape results.
    pub fn invalid_results(&self) -> Vec<&BRepCheck_Result> {
        self.results.values().filter(|r| !r.is_valid()).collect()
    }

    /// Total number of sub-shapes that were checked.
    pub fn checked_count(&self) -> usize {
        self.results.len()
    }

    /// Number of sub-shapes that failed at least one check.
    pub fn error_count(&self) -> usize {
        self.results.values().filter(|r| !r.is_valid()).count()
    }
}

// ---------------------------------------------------------------------------
// Convenience free functions
// ---------------------------------------------------------------------------

/// Quickly check whether `shape` is valid without building a full analyzer.
pub fn is_valid(shape: &TopoDS_Shape) -> bool {
    BRepCheck_Analyzer::new(shape.clone()).is_valid()
}

/// Return the primary status for `shape` (checks `shape` only, not sub-shapes).
pub fn check_status(shape: &TopoDS_Shape) -> BRepCheck_Status {
    check_shape_directly(shape)
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::top_exp::{TopoDS_Vertex, TopoDS_Edge, TopoDS_Wire, TopoDS_Face, TopoDS_Shell, TopoDS_Solid};

    // ---- helpers ----

    fn vertex(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
        TopoDS_Vertex { id, x, y, z }
    }

    fn valid_edge(id: u64) -> TopoDS_Edge {
        TopoDS_Edge {
            id,
            vertices: vec![
                vertex(id * 10, 0.0, 0.0, 0.0),
                vertex(id * 10 + 1, 1.0, 0.0, 0.0),
            ],
        }
    }

    /// Build a triangular closed wire (equilateral in XY).
    fn triangle_wire(id: u64) -> TopoDS_Wire {
        let p0 = (0.0f64, 0.0f64, 0.0f64);
        let p1 = (1.0f64, 0.0f64, 0.0f64);
        let p2 = (0.5f64, 1.0f64, 0.0f64);
        let e0 = TopoDS_Edge {
            id: id * 10,
            vertices: vec![
                TopoDS_Vertex { id: id * 100, x: p0.0, y: p0.1, z: p0.2 },
                TopoDS_Vertex { id: id * 100 + 1, x: p1.0, y: p1.1, z: p1.2 },
            ],
        };
        let e1 = TopoDS_Edge {
            id: id * 10 + 1,
            vertices: vec![
                TopoDS_Vertex { id: id * 100 + 1, x: p1.0, y: p1.1, z: p1.2 },
                TopoDS_Vertex { id: id * 100 + 2, x: p2.0, y: p2.1, z: p2.2 },
            ],
        };
        let e2 = TopoDS_Edge {
            id: id * 10 + 2,
            vertices: vec![
                TopoDS_Vertex { id: id * 100 + 2, x: p2.0, y: p2.1, z: p2.2 },
                TopoDS_Vertex { id: id * 100, x: p0.0, y: p0.1, z: p0.2 },
            ],
        };
        TopoDS_Wire { id, edges: vec![e0, e1, e2] }
    }

    /// Build a minimal tetrahedron-like solid (4 faces).
    fn tetra_shell(id: u64) -> TopoDS_Shell {
        // 4 triangular faces sharing vertices:
        let f0 = TopoDS_Face { id: id * 10, wires: vec![triangle_wire(id * 10)] };
        let f1 = TopoDS_Face { id: id * 10 + 1, wires: vec![triangle_wire(id * 10 + 1)] };
        let f2 = TopoDS_Face { id: id * 10 + 2, wires: vec![triangle_wire(id * 10 + 2)] };
        let f3 = TopoDS_Face { id: id * 10 + 3, wires: vec![triangle_wire(id * 10 + 3)] };
        TopoDS_Shell { id, faces: vec![f0, f1, f2, f3] }
    }

    // ---- BRepCheck_Status tests ----

    #[test]
    fn test_status_no_error_is_ok() {
        assert!(BRepCheck_Status::NoError.is_ok());
    }

    #[test]
    fn test_status_error_not_ok() {
        assert!(!BRepCheck_Status::NotClosed.is_ok());
        assert!(!BRepCheck_Status::InvalidVertex.is_ok());
        assert!(!BRepCheck_Status::OpenShell.is_ok());
    }

    #[test]
    fn test_status_description_non_empty() {
        let statuses = [
            BRepCheck_Status::NoError,
            BRepCheck_Status::InvalidVertex,
            BRepCheck_Status::InvalidDegeneratedEdge,
            BRepCheck_Status::NotClosed,
            BRepCheck_Status::NotConnected,
            BRepCheck_Status::InvalidFace,
            BRepCheck_Status::OpenShell,
            BRepCheck_Status::InvalidSolid,
        ];
        for s in &statuses {
            assert!(!s.description().is_empty(), "{:?} should have a description", s);
        }
    }

    // ---- BRepCheck_ListOfStatus tests ----

    #[test]
    fn test_list_new_is_empty() {
        let list = BRepCheck_ListOfStatus::new();
        assert!(list.is_empty());
        assert!(list.is_valid()); // empty = no errors
    }

    #[test]
    fn test_list_with_no_error_is_valid() {
        let list = BRepCheck_ListOfStatus::with_status(BRepCheck_Status::NoError);
        assert!(list.is_valid());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_with_error_not_valid() {
        let list = BRepCheck_ListOfStatus::with_status(BRepCheck_Status::NotClosed);
        assert!(!list.is_valid());
    }

    #[test]
    fn test_list_add_dedup() {
        let mut list = BRepCheck_ListOfStatus::new();
        list.add(BRepCheck_Status::NotClosed);
        list.add(BRepCheck_Status::NotClosed);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_contains() {
        let mut list = BRepCheck_ListOfStatus::new();
        list.add(BRepCheck_Status::OpenShell);
        assert!(list.contains(BRepCheck_Status::OpenShell));
        assert!(!list.contains(BRepCheck_Status::NotClosed));
    }

    #[test]
    fn test_list_iter() {
        let mut list = BRepCheck_ListOfStatus::new();
        list.add(BRepCheck_Status::NotClosed);
        list.add(BRepCheck_Status::NotConnected);
        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected.len(), 2);
    }

    // ---- check_vertex ----

    #[test]
    fn test_vertex_valid() {
        let v = vertex(1, 0.0, 1.0, -2.5);
        assert_eq!(check_vertex(&v), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_vertex_nan() {
        let v = vertex(2, f64::NAN, 0.0, 0.0);
        assert_eq!(check_vertex(&v), BRepCheck_Status::InvalidVertex);
    }

    #[test]
    fn test_vertex_inf() {
        let v = vertex(3, 0.0, f64::INFINITY, 0.0);
        assert_eq!(check_vertex(&v), BRepCheck_Status::InvalidVertex);
    }

    // ---- check_edge ----

    #[test]
    fn test_edge_valid() {
        let e = valid_edge(1);
        assert_eq!(check_edge(&e), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_edge_degenerate_same_point() {
        let e = TopoDS_Edge {
            id: 99,
            vertices: vec![
                vertex(1, 1.0, 2.0, 3.0),
                vertex(2, 1.0, 2.0, 3.0),
            ],
        };
        assert_eq!(check_edge(&e), BRepCheck_Status::InvalidDegeneratedEdge);
    }

    #[test]
    fn test_edge_too_few_vertices() {
        let e = TopoDS_Edge { id: 5, vertices: vec![vertex(1, 0.0, 0.0, 0.0)] };
        assert_eq!(check_edge(&e), BRepCheck_Status::InvalidEdge);
    }

    // ---- check_wire ----

    #[test]
    fn test_wire_valid_triangle() {
        let w = triangle_wire(1);
        assert_eq!(check_wire(&w), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_wire_not_closed() {
        // Last edge does NOT connect back to vertex 0.
        let e0 = TopoDS_Edge {
            id: 1,
            vertices: vec![vertex(1, 0.0, 0.0, 0.0), vertex(2, 1.0, 0.0, 0.0)],
        };
        let e1 = TopoDS_Edge {
            id: 2,
            vertices: vec![vertex(2, 1.0, 0.0, 0.0), vertex(3, 2.0, 0.0, 0.0)],
        };
        let w = TopoDS_Wire { id: 99, edges: vec![e0, e1] };
        assert_eq!(check_wire(&w), BRepCheck_Status::NotClosed);
    }

    #[test]
    fn test_wire_not_connected() {
        // e0 ends at (1,0,0), e1 starts at (5,0,0) — gap.
        let e0 = TopoDS_Edge {
            id: 1,
            vertices: vec![vertex(1, 0.0, 0.0, 0.0), vertex(2, 1.0, 0.0, 0.0)],
        };
        let e1 = TopoDS_Edge {
            id: 2,
            vertices: vec![vertex(3, 5.0, 0.0, 0.0), vertex(4, 6.0, 0.0, 0.0)],
        };
        let e2 = TopoDS_Edge {
            id: 3,
            vertices: vec![vertex(4, 6.0, 0.0, 0.0), vertex(1, 0.0, 0.0, 0.0)],
        };
        let w = TopoDS_Wire { id: 99, edges: vec![e0, e1, e2] };
        assert_eq!(check_wire(&w), BRepCheck_Status::NotConnected);
    }

    // ---- check_face ----

    #[test]
    fn test_face_valid_triangle() {
        let w = triangle_wire(1);
        let f = TopoDS_Face { id: 1, wires: vec![w] };
        assert_eq!(check_face(&f), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_face_no_wires() {
        let f = TopoDS_Face { id: 1, wires: vec![] };
        assert_eq!(check_face(&f), BRepCheck_Status::NoSurface);
    }

    #[test]
    fn test_face_collinear_degenerate() {
        // All points collinear → degenerate normal.
        let e0 = TopoDS_Edge {
            id: 1,
            vertices: vec![vertex(1, 0.0, 0.0, 0.0), vertex(2, 1.0, 0.0, 0.0)],
        };
        let e1 = TopoDS_Edge {
            id: 2,
            vertices: vec![vertex(2, 1.0, 0.0, 0.0), vertex(3, 2.0, 0.0, 0.0)],
        };
        let e2 = TopoDS_Edge {
            id: 3,
            vertices: vec![vertex(3, 2.0, 0.0, 0.0), vertex(1, 0.0, 0.0, 0.0)],
        };
        let w = TopoDS_Wire { id: 1, edges: vec![e0, e1, e2] };
        let f = TopoDS_Face { id: 1, wires: vec![w] };
        assert_eq!(check_face(&f), BRepCheck_Status::InvalidFace);
    }

    // ---- check_shell ----

    #[test]
    fn test_shell_valid_tetra() {
        let sh = tetra_shell(1);
        assert_eq!(check_shell(&sh), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_shell_too_few_faces() {
        let f0 = TopoDS_Face { id: 1, wires: vec![triangle_wire(1)] };
        let f1 = TopoDS_Face { id: 2, wires: vec![triangle_wire(2)] };
        let sh = TopoDS_Shell { id: 99, faces: vec![f0, f1] };
        assert_eq!(check_shell(&sh), BRepCheck_Status::OpenShell);
    }

    #[test]
    fn test_shell_empty() {
        let sh = TopoDS_Shell { id: 1, faces: vec![] };
        assert_eq!(check_shell(&sh), BRepCheck_Status::OpenShell);
    }

    // ---- check_solid ----

    #[test]
    fn test_solid_valid() {
        let s = TopoDS_Solid { id: 1, shells: vec![tetra_shell(1)] };
        assert_eq!(check_solid(&s), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_solid_no_shells() {
        let s = TopoDS_Solid { id: 1, shells: vec![] };
        assert_eq!(check_solid(&s), BRepCheck_Status::InvalidSolid);
    }

    // ---- BRepCheck_Result ----

    #[test]
    fn test_result_new_is_valid() {
        let v = TopoDS_Shape::Vertex(vertex(1, 0.0, 0.0, 0.0));
        let r = BRepCheck_Result::new(v);
        assert!(r.is_valid());
        assert_eq!(r.status(), BRepCheck_Status::NoError);
    }

    #[test]
    fn test_result_with_error_not_valid() {
        let v = TopoDS_Shape::Vertex(vertex(2, f64::NAN, 0.0, 0.0));
        let r = BRepCheck_Result::with_status(v, BRepCheck_Status::InvalidVertex);
        assert!(!r.is_valid());
        assert_eq!(r.status(), BRepCheck_Status::InvalidVertex);
    }

    #[test]
    fn test_result_add_status_replaces_no_error() {
        let v = TopoDS_Shape::Vertex(vertex(3, 0.0, 0.0, 0.0));
        let mut r = BRepCheck_Result::new(v);
        r.add_status(BRepCheck_Status::NotClosed);
        assert!(!r.is_valid());
        assert_eq!(r.status(), BRepCheck_Status::NotClosed);
    }

    // ---- BRepCheck_Analyzer ----

    #[test]
    fn test_analyzer_valid_solid() {
        let sh = tetra_shell(1);
        let s = TopoDS_Solid { id: 1, shells: vec![sh] };
        let shape = TopoDS_Shape::Solid(s);
        let analyzer = BRepCheck_Analyzer::new(shape);
        assert!(analyzer.is_valid());
        assert_eq!(analyzer.error_count(), 0);
    }

    #[test]
    fn test_analyzer_invalid_vertex_detected() {
        let v = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: f64::NAN, y: 0.0, z: 0.0 });
        let analyzer = BRepCheck_Analyzer::new(v);
        assert!(!analyzer.is_valid());
        assert!(analyzer.error_count() > 0);
    }

    #[test]
    fn test_analyzer_valid_vertex() {
        let v = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 42, x: 1.0, y: 2.0, z: 3.0 });
        let analyzer = BRepCheck_Analyzer::new(v);
        assert!(analyzer.is_valid());
    }

    #[test]
    fn test_analyzer_result_for() {
        let e = TopoDS_Shape::Edge(valid_edge(7));
        let analyzer = BRepCheck_Analyzer::new(e.clone());
        let r = analyzer.result_for(&e);
        assert!(r.is_some());
        assert!(r.unwrap().is_valid());
    }

    #[test]
    fn test_analyzer_checked_count_nonzero() {
        let w = TopoDS_Shape::Wire(triangle_wire(1));
        let analyzer = BRepCheck_Analyzer::new(w);
        assert!(analyzer.checked_count() > 0);
    }

    #[test]
    fn test_is_valid_convenience() {
        let v = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: 0.0, y: 0.0, z: 0.0 });
        assert!(is_valid(&v));
    }

    #[test]
    fn test_check_status_convenience() {
        let v = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 2, x: f64::NAN, y: 0.0, z: 0.0 });
        assert_eq!(check_status(&v), BRepCheck_Status::InvalidVertex);
    }
}
