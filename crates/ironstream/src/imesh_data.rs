// FILE: imesh_data.rs
//! `IMeshData` — triangulation data structures mirroring OpenCascade's
//! `IMeshData` package.
//!
//! # Key types
//!
//! * [`IMeshData_Parameters`] — meshing quality / control parameters.
//! * [`IMeshData_Curve`]      — a discrete polyline approximation of a 3-D
//!   edge curve, storing parameter values and 3-D points.
//! * [`IMeshData_Edge`]       — a topological edge decorated with one or more
//!   discrete curves and parametric data.
//! * [`IMeshData_Face`]       — a topological face decorated with a list of
//!   mesh edges and a triangulation result.

use crate::gp::Pnt;

// ---------------------------------------------------------------------------
// IMeshData_Parameters
// ---------------------------------------------------------------------------

/// Meshing quality / control parameters.
///
/// Mirrors `IMeshData_Parameters` / `BRepMesh_FastDiscret_Params` from OCCT.
// occt: IMeshData_Parameters
#[derive(Clone, Debug)]
pub struct IMeshData_Parameters {
    /// Maximum chord height deviation (absolute or relative — see `relative`).
    pub linear_deflection: f64,

    /// Maximum angular deviation in radians between adjacent face normals at
    /// a triangle edge.
    pub angular_deflection: f64,

    /// When `true` `linear_deflection` is interpreted as a fraction of the
    /// bounding-box diagonal; when `false` it is an absolute model-unit value.
    pub relative: bool,

    /// Advisory flag: allow meshing of different shapes to proceed in parallel.
    /// In this single-threaded implementation the flag is honoured in the API
    /// surface but has no runtime effect.
    pub in_parallel: bool,

    /// When `true`, only out-of-date or absent triangulations are recomputed.
    pub mesh_in_parallel: bool,

    /// Allow a controlled reduction in mesh quality to avoid very-long runs.
    pub allow_quality_decrease: bool,

    /// Minimum edge length expressed as a fraction of `linear_deflection`.
    pub min_size: f64,
}

impl IMeshData_Parameters {
    /// Construct with explicit linear / angular deflections and all boolean
    /// options at their conservative defaults.
    pub fn new(linear_deflection: f64, angular_deflection: f64) -> Self {
        Self {
            linear_deflection: linear_deflection.max(1.0e-7),
            angular_deflection: angular_deflection.max(1.0e-5),
            relative: false,
            in_parallel: false,
            mesh_in_parallel: false,
            allow_quality_decrease: false,
            min_size: 1.0e-7,
        }
    }

    /// Convenience: also set the `relative` flag in one call.
    pub fn with_relative(linear_deflection: f64, angular_deflection: f64, relative: bool) -> Self {
        let mut p = Self::new(linear_deflection, angular_deflection);
        p.relative = relative;
        p
    }

    /// Effective linear deflection given bounding-box diagonal `diag`.
    ///
    /// In absolute mode returns `linear_deflection` unchanged; in relative mode
    /// returns `linear_deflection * diag` (clamped to ≥ 1e-7).
    pub fn effective_deflection(&self, diag: f64) -> f64 {
        if self.relative {
            (self.linear_deflection * diag).max(1.0e-7)
        } else {
            self.linear_deflection
        }
    }

    /// `true` when the parameter set represents a coarser-than-usual mesh
    /// (linear deflection > 0.1 or angular deflection > π/6).
    pub fn is_coarse(&self) -> bool {
        use std::f64::consts::PI;
        self.linear_deflection > 0.1 || self.angular_deflection > PI / 6.0
    }

    /// Clamp both deflections to physically sensible bounds and return `self`.
    pub fn clamped(mut self, max_linear: f64, max_angular: f64) -> Self {
        self.linear_deflection = self.linear_deflection.min(max_linear).max(1.0e-7);
        self.angular_deflection = self.angular_deflection.min(max_angular).max(1.0e-5);
        self
    }
}

impl Default for IMeshData_Parameters {
    fn default() -> Self {
        Self::new(0.1, 0.5)
    }
}

// ---------------------------------------------------------------------------
// IMeshData_Curve
// ---------------------------------------------------------------------------

/// A single discrete polyline approximation of a 3-D edge curve.
///
/// Stores paired `(parameter, point)` samples along the curve so that the
/// parametric position can be correlated with the 3-D position.  This mirrors
/// `IMeshData_Curve` from OCCT.
// occt: IMeshData_Curve
#[derive(Clone, Debug)]
pub struct IMeshData_Curve {
    /// Paired `(t, P)` samples.  `t` is the curve parameter; `P` is the
    /// corresponding 3-D position.
    samples: Vec<(f64, Pnt)>,
}

impl IMeshData_Curve {
    /// Create an empty curve buffer.
    pub fn new() -> Self {
        Self { samples: Vec::new() }
    }

    /// Create from pre-built sample vectors (equal length required).
    ///
    /// Panics if `params.len() != points.len()`.
    pub fn from_samples(params: Vec<f64>, points: Vec<Pnt>) -> Self {
        assert_eq!(
            params.len(),
            points.len(),
            "IMeshData_Curve: params and points must have the same length"
        );
        Self {
            samples: params.into_iter().zip(points).collect(),
        }
    }

    /// Append a `(parameter, point)` sample.
    pub fn add_point(&mut self, t: f64, p: Pnt) {
        self.samples.push((t, p));
    }

    /// Number of discrete samples.
    pub fn nb_points(&self) -> usize {
        self.samples.len()
    }

    /// Return the `i`-th 3-D point (0-based).  Panics if out of range.
    pub fn point(&self, i: usize) -> Pnt {
        self.samples[i].1
    }

    /// Return the `i`-th curve parameter (0-based).  Panics if out of range.
    pub fn parameter(&self, i: usize) -> f64 {
        self.samples[i].0
    }

    /// Slice of all `(t, P)` samples.
    pub fn samples(&self) -> &[(f64, Pnt)] {
        &self.samples
    }

    /// Return `true` if there are no samples.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Compute the approximate arc length of the polyline.
    pub fn length(&self) -> f64 {
        if self.samples.len() < 2 {
            return 0.0;
        }
        self.samples
            .windows(2)
            .map(|w| w[0].1.distance(w[1].1))
            .sum()
    }

    /// Return the parameter range `(t_min, t_max)`, or `(0.0, 0.0)` if empty.
    pub fn param_range(&self) -> (f64, f64) {
        if self.samples.is_empty() {
            return (0.0, 0.0);
        }
        let t0 = self.samples.first().unwrap().0;
        let t1 = self.samples.last().unwrap().0;
        (t0, t1)
    }

    /// Subdivide the curve so that no chord exceeds `max_chord`.
    ///
    /// New points are inserted by linear interpolation; existing samples are
    /// kept.  Returns a new `IMeshData_Curve` with the refined samples.
    pub fn subdivided(&self, max_chord: f64) -> IMeshData_Curve {
        if self.samples.len() < 2 {
            return self.clone();
        }
        let mut out = IMeshData_Curve::new();
        out.add_point(self.samples[0].0, self.samples[0].1);
        for w in self.samples.windows(2) {
            let (t0, p0) = w[0];
            let (t1, p1) = w[1];
            let chord = p0.distance(p1);
            if chord > max_chord {
                let n = (chord / max_chord).ceil() as usize;
                for k in 1..n {
                    let alpha = k as f64 / n as f64;
                    let ti = t0 + alpha * (t1 - t0);
                    let pi = p0.lerp(p1, alpha);
                    out.add_point(ti, pi);
                }
            }
            out.add_point(t1, p1);
        }
        out
    }
}

impl Default for IMeshData_Curve {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// IMeshData_Edge
// ---------------------------------------------------------------------------

/// Orientation of an edge relative to its parent wire.
// occt: TopAbs_Orientation (Forward / Reversed)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeOrientation {
    Forward,
    Reversed,
}

/// A discretised topological edge: an edge id, its orientation, one or more
/// discrete 3-D curves (one per face it borders), and a tolerance value.
///
/// Mirrors `IMeshData_Edge` from OCCT's incremental meshing data model.
// occt: IMeshData_Edge
#[derive(Clone, Debug)]
pub struct IMeshData_Edge {
    /// Stable numeric identifier matching the originating `TopoDS_Edge`.
    pub id: u64,

    /// Edge orientation within its parent wire.
    pub orientation: EdgeOrientation,

    /// 3-D discrete curves — one entry per bordering face in the general
    /// case; typically one or two entries.
    curves: Vec<IMeshData_Curve>,

    /// 2-D parametric polylines in the surface UV space of each bordering
    /// face (paired 1-to-1 with `curves`).
    pcurves: Vec<Vec<(f64, f64)>>,

    /// Geometric tolerance of the edge vertices.
    pub tolerance: f64,

    /// `true` if the edge has been processed / is free of geometry issues.
    pub is_free: bool,
}

impl IMeshData_Edge {
    /// Create a new edge with the given id and orientation.
    pub fn new(id: u64, orientation: EdgeOrientation) -> Self {
        Self {
            id,
            orientation,
            curves: Vec::new(),
            pcurves: Vec::new(),
            tolerance: 1.0e-7,
            is_free: false,
        }
    }

    /// Append a 3-D discrete curve for this edge (one per bordering face).
    pub fn add_curve(&mut self, curve: IMeshData_Curve) {
        self.curves.push(curve);
    }

    /// Append a 2-D parametric polyline in the UV space of the bordering face.
    pub fn add_pcurve(&mut self, uv_pts: Vec<(f64, f64)>) {
        self.pcurves.push(uv_pts);
    }

    /// Number of 3-D curves stored (usually 1 per bordering face).
    pub fn nb_curves(&self) -> usize {
        self.curves.len()
    }

    /// Return a reference to the `i`-th 3-D curve (0-based).
    pub fn curve(&self, i: usize) -> &IMeshData_Curve {
        &self.curves[i]
    }

    /// Return a mutable reference to the `i`-th 3-D curve (0-based).
    pub fn curve_mut(&mut self, i: usize) -> &mut IMeshData_Curve {
        &mut self.curves[i]
    }

    /// Number of 2-D parametric curves stored.
    pub fn nb_pcurves(&self) -> usize {
        self.pcurves.len()
    }

    /// Return a reference to the `i`-th UV polyline (0-based).
    pub fn pcurve(&self, i: usize) -> &[(f64, f64)] {
        &self.pcurves[i]
    }

    /// Compute the approximate 3-D arc length of the first discrete curve, or
    /// 0.0 if no curves have been added.
    pub fn length(&self) -> f64 {
        self.curves.first().map_or(0.0, |c| c.length())
    }

    /// `true` when the edge carries at least one non-empty discrete curve.
    pub fn has_geometry(&self) -> bool {
        self.curves.iter().any(|c| !c.is_empty())
    }
}

// ---------------------------------------------------------------------------
// IMeshData_Face
// ---------------------------------------------------------------------------

/// Status of a meshed face.
// occt: IMeshData_Status (subset)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceStatus {
    /// Not yet processed.
    NotMeshed,
    /// Discretisation succeeded.
    Ok,
    /// Degenerate face (e.g. zero-area).
    Degenerate,
    /// Meshing failed for this face.
    Failed,
}

/// A face augmented with its discrete-mesh data: a set of `IMeshData_Edge`
/// children and an optional list of 3-D triangle vertices and triangle indices.
///
/// Mirrors `IMeshData_Face` from OCCT's incremental meshing data model.
// occt: IMeshData_Face
#[derive(Clone, Debug)]
pub struct IMeshData_Face {
    /// Stable numeric identifier matching the originating `TopoDS_Face`.
    pub id: u64,

    /// Processing status.
    pub status: FaceStatus,

    /// Deflection at which this face was (or will be) meshed.
    pub deflection: f64,

    /// Discretised boundary edges.
    edges: Vec<IMeshData_Edge>,

    /// Triangulation nodes (3-D positions).
    nodes: Vec<Pnt>,

    /// Triangle connectivity: each entry is `(i0, i1, i2)` into `nodes`.
    triangles: Vec<(usize, usize, usize)>,
}

impl IMeshData_Face {
    /// Create an empty face data container for face `id`.
    pub fn new(id: u64) -> Self {
        Self {
            id,
            status: FaceStatus::NotMeshed,
            deflection: 0.0,
            edges: Vec::new(),
            nodes: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Append a discretised edge.
    pub fn add_edge(&mut self, edge: IMeshData_Edge) {
        self.edges.push(edge);
    }

    /// Number of discretised edges stored.
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Return a reference to the `i`-th edge (0-based).
    pub fn edge(&self, i: usize) -> &IMeshData_Edge {
        &self.edges[i]
    }

    /// Return a mutable reference to the `i`-th edge (0-based).
    pub fn edge_mut(&mut self, i: usize) -> &mut IMeshData_Edge {
        &mut self.edges[i]
    }

    /// Replace the triangulation data wholesale.
    pub fn set_triangulation(
        &mut self,
        nodes: Vec<Pnt>,
        triangles: Vec<(usize, usize, usize)>,
        deflection: f64,
    ) {
        self.nodes = nodes;
        self.triangles = triangles;
        self.deflection = deflection;
        self.status = if self.triangles.is_empty() {
            FaceStatus::Degenerate
        } else {
            FaceStatus::Ok
        };
    }

    /// Number of triangulation nodes.
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Number of triangles.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Return the `i`-th node (0-based).
    pub fn node(&self, i: usize) -> Pnt {
        self.nodes[i]
    }

    /// Return the `i`-th triangle as `(i0, i1, i2)` (0-based).
    pub fn triangle(&self, i: usize) -> (usize, usize, usize) {
        self.triangles[i]
    }

    /// `true` when a valid triangulation has been stored.
    pub fn is_meshed(&self) -> bool {
        self.status == FaceStatus::Ok
    }

    /// Compute the surface area of the stored triangulation.
    pub fn area(&self) -> f64 {
        self.triangles
            .iter()
            .map(|&(i0, i1, i2)| {
                let a = self.nodes[i0];
                let b = self.nodes[i1];
                let c = self.nodes[i2];
                (b - a).cross(c - a).norm() * 0.5
            })
            .sum()
    }

    /// Compute the axis-aligned bounding box of the triangulation nodes as
    /// `(min_corner, max_corner)`.  Returns `(origin, origin)` when empty.
    pub fn bounding_box(&self) -> (Pnt, Pnt) {
        if self.nodes.is_empty() {
            return (Pnt::origin(), Pnt::origin());
        }
        let mut mn = self.nodes[0];
        let mut mx = self.nodes[0];
        for &p in &self.nodes[1..] {
            if p.x < mn.x { mn.x = p.x; }
            if p.y < mn.y { mn.y = p.y; }
            if p.z < mn.z { mn.z = p.z; }
            if p.x > mx.x { mx.x = p.x; }
            if p.y > mx.y { mx.y = p.y; }
            if p.z > mx.z { mx.z = p.z; }
        }
        (mn, mx)
    }

    /// Validate the triangulation: every triangle index must be in range and
    /// the three vertices must be distinct.  Returns the list of bad triangle
    /// indices (empty means valid).
    pub fn validate_triangulation(&self) -> Vec<usize> {
        let n = self.nodes.len();
        let mut bad = Vec::new();
        for (ti, &(i0, i1, i2)) in self.triangles.iter().enumerate() {
            if i0 >= n || i1 >= n || i2 >= n || i0 == i1 || i1 == i2 || i0 == i2 {
                bad.push(ti);
            }
        }
        bad
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;

    // --- IMeshData_Parameters -----------------------------------------------

    #[test]
    fn params_default_values() {
        let p = IMeshData_Parameters::default();
        assert!((p.linear_deflection - 0.1).abs() < 1e-12);
        assert!((p.angular_deflection - 0.5).abs() < 1e-12);
        assert!(!p.relative);
        assert!(!p.in_parallel);
    }

    #[test]
    fn params_clamp_floor() {
        // Deflections below the minimum should be silently clamped.
        let p = IMeshData_Parameters::new(0.0, 0.0);
        assert!(p.linear_deflection >= 1e-7);
        assert!(p.angular_deflection >= 1e-5);
    }

    #[test]
    fn params_effective_deflection_absolute() {
        let p = IMeshData_Parameters::new(0.05, 0.3);
        // Absolute mode: result == linear_deflection regardless of diag.
        assert!((p.effective_deflection(100.0) - 0.05).abs() < 1e-12);
    }

    #[test]
    fn params_effective_deflection_relative() {
        let p = IMeshData_Parameters::with_relative(0.01, 0.3, true);
        // Relative mode: 0.01 * 50.0 == 0.5
        assert!((p.effective_deflection(50.0) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn params_is_coarse() {
        let coarse = IMeshData_Parameters::new(0.5, 1.0);
        assert!(coarse.is_coarse());
        let fine = IMeshData_Parameters::new(0.01, 0.1);
        assert!(!fine.is_coarse());
    }

    // --- IMeshData_Curve ----------------------------------------------------

    #[test]
    fn curve_add_and_retrieve() {
        let mut c = IMeshData_Curve::new();
        c.add_point(0.0, Pnt::new(0.0, 0.0, 0.0));
        c.add_point(1.0, Pnt::new(1.0, 0.0, 0.0));
        c.add_point(2.0, Pnt::new(2.0, 0.0, 0.0));
        assert_eq!(c.nb_points(), 3);
        assert!((c.point(1).x - 1.0).abs() < 1e-12);
        assert!((c.parameter(2) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn curve_length() {
        let mut c = IMeshData_Curve::new();
        c.add_point(0.0, Pnt::new(0.0, 0.0, 0.0));
        c.add_point(1.0, Pnt::new(3.0, 4.0, 0.0));
        // Distance between (0,0,0) and (3,4,0) == 5.
        assert!((c.length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn curve_param_range() {
        let c = IMeshData_Curve::from_samples(
            vec![0.5, 1.5, 3.0],
            vec![Pnt::origin(), Pnt::new(1.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)],
        );
        let (t0, t1) = c.param_range();
        assert!((t0 - 0.5).abs() < 1e-12);
        assert!((t1 - 3.0).abs() < 1e-12);
    }

    #[test]
    fn curve_subdivided_inserts_midpoints() {
        let mut c = IMeshData_Curve::new();
        c.add_point(0.0, Pnt::new(0.0, 0.0, 0.0));
        c.add_point(1.0, Pnt::new(4.0, 0.0, 0.0));
        // max_chord = 1.0 → segment of length 4 should become 5 points.
        let refined = c.subdivided(1.0);
        assert!(refined.nb_points() >= 5, "got {}", refined.nb_points());
        // Total length must be preserved.
        assert!((refined.length() - 4.0).abs() < 1e-10);
    }

    // --- IMeshData_Edge -----------------------------------------------------

    #[test]
    fn edge_add_curve_and_length() {
        let mut edge = IMeshData_Edge::new(1, EdgeOrientation::Forward);
        let mut curve = IMeshData_Curve::new();
        curve.add_point(0.0, Pnt::new(0.0, 0.0, 0.0));
        curve.add_point(1.0, Pnt::new(0.0, 3.0, 4.0));
        edge.add_curve(curve);
        assert_eq!(edge.nb_curves(), 1);
        assert!((edge.length() - 5.0).abs() < 1e-10);
        assert!(edge.has_geometry());
    }

    #[test]
    fn edge_pcurve_roundtrip() {
        let mut edge = IMeshData_Edge::new(2, EdgeOrientation::Reversed);
        edge.add_pcurve(vec![(0.0, 0.0), (0.5, 0.5), (1.0, 1.0)]);
        assert_eq!(edge.nb_pcurves(), 1);
        assert_eq!(edge.pcurve(0).len(), 3);
        assert_eq!(edge.pcurve(0)[1], (0.5, 0.5));
    }

    // --- IMeshData_Face -----------------------------------------------------

    #[test]
    fn face_triangulation_area() {
        // Right-triangle in the XY plane: vertices (0,0,0),(1,0,0),(0,1,0).
        let nodes = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ];
        let triangles = vec![(0, 1, 2)];
        let mut face = IMeshData_Face::new(42);
        face.set_triangulation(nodes, triangles, 0.1);
        assert_eq!(face.status, FaceStatus::Ok);
        // Area of right triangle with legs 1,1 == 0.5.
        assert!((face.area() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn face_bounding_box() {
        let nodes = vec![
            Pnt::new(-1.0, 0.0, 0.0),
            Pnt::new(2.0, 3.0, 1.0),
            Pnt::new(0.0, -1.0, 4.0),
        ];
        let mut face = IMeshData_Face::new(7);
        face.set_triangulation(nodes, vec![(0, 1, 2)], 0.05);
        let (mn, mx) = face.bounding_box();
        assert!((mn.x - (-1.0)).abs() < 1e-12);
        assert!((mx.x - 2.0).abs() < 1e-12);
        assert!((mn.y - (-1.0)).abs() < 1e-12);
        assert!((mx.z - 4.0).abs() < 1e-12);
    }

    #[test]
    fn face_validate_triangulation_detects_bad_index() {
        let nodes = vec![Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)];
        // Triangle references index 2 which is out of range.
        let triangles = vec![(0, 1, 2)];
        let mut face = IMeshData_Face::new(99);
        face.set_triangulation(nodes, triangles, 0.1);
        let bad = face.validate_triangulation();
        assert_eq!(bad, vec![0usize]);
    }

    #[test]
    fn face_status_degenerate_on_empty_triangles() {
        let mut face = IMeshData_Face::new(10);
        face.set_triangulation(vec![Pnt::origin()], vec![], 0.1);
        assert_eq!(face.status, FaceStatus::Degenerate);
        assert!(!face.is_meshed());
    }
}
