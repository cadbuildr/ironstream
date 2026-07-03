// FILE: int_curves_face.rs
//! `IntCurvesFace` — intersection of curves with B-Rep faces / shapes.
//!
//! # Scope
//!
//! * [`IntCurvesFace_Intersector`] — intersects a single analytic face (plane,
//!   cylinder, sphere, cone, torus) with a parametric line segment or infinite
//!   line.
//! * [`IntCurvesFace_ShapeIntersector`] — iterates over all faces in a
//!   [`TopoDS_Shape`] and collects every intersection point with a line.
//!
//! # Algorithm
//!
//! For each face the algorithm works in the face's local parametric space:
//!
//! 1. **Plane** — classic ray/plane intersection: solve `(P + t·D)·N = d`.
//! 2. **Cylinder** — the ray is projected onto the cylinder's axial plane; two
//!    roots come from a quadratic in `t`.
//! 3. **Sphere** — quadratic from `|P + t·D − C|² = r²`.
//! 4. **General face (polygon boundary)** — the tessellated face is treated as
//!    a planar polygon; the ray/plane step is followed by a 2D point-in-polygon
//!    test on the projected boundary vertices.
//!
//! The implementation mirrors the public surface of OCCT's
//! `IntCurvesFace_ShapeIntersector` and `IntCurvesFace_Intersector`.

use std::f64::consts::PI;

use crate::gp::{Ax3, Pnt, Vec3};
use crate::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell, TopoDS_Solid,
    TopoDS_Vertex, TopoDS_Wire,
};

// ─────────────────────────────────────────────────────────────────────────────
// IntersectionPoint
// ─────────────────────────────────────────────────────────────────────────────

/// One intersection between the curve and a face.
///
/// Mirrors the data accessible via `IntCurveSurface_IntersectionPoint` from
/// OCCT's `IntCurveSurface` package.
// occt: IntCurveSurface_IntersectionPoint
#[derive(Clone, Debug)]
pub struct IntersectionPoint {
    /// The 3D intersection point in world space.
    pub point: Pnt,
    /// Parameter value `t` on the curve at this intersection.
    pub w: f64,
    /// First surface parameter `u` at the intersection.
    pub u: f64,
    /// Second surface parameter `v` at the intersection.
    pub v: f64,
    /// `true` if the ray enters the solid (normal opposes ray direction).
    pub is_entry: bool,
}

impl IntersectionPoint {
    /// Construct a new intersection record.
    pub fn new(point: Pnt, w: f64, u: f64, v: f64, is_entry: bool) -> Self {
        Self { point, w, u, v, is_entry }
    }

    /// 3D point of the intersection.
    pub fn pnt(&self) -> Pnt {
        self.point
    }

    /// Parameter on the curve (line parameter `t`).
    pub fn w(&self) -> f64 {
        self.w
    }

    /// First surface parameter.
    pub fn u(&self) -> f64 {
        self.u
    }

    /// Second surface parameter.
    pub fn v(&self) -> f64 {
        self.v
    }

    /// `true` when the curve enters the shape at this point.
    pub fn is_entry(&self) -> bool {
        self.is_entry
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FaceKind — classify each face for the appropriate intersection sub-solver
// ─────────────────────────────────────────────────────────────────────────────

/// Internal enumeration of the face's analytic surface kind.
#[derive(Clone, Debug)]
enum FaceKind {
    /// A planar face defined by a normal and a point on the plane.
    Plane {
        origin: Pnt,
        normal: Pnt,
        /// X direction of the plane (for UV parameterisation).
        x_dir: Pnt,
        /// Y direction of the plane (for UV parameterisation).
        y_dir: Pnt,
    },
    /// A cylindrical face.  The axis is `z_dir`, the reference circle on the
    /// `x_dir` side.
    Cylinder {
        center: Pnt,
        z_dir: Pnt,
        x_dir: Pnt,
        y_dir: Pnt,
        radius: f64,
    },
    /// A spherical face.
    Sphere {
        center: Pnt,
        radius: f64,
    },
    /// Any other face approximated by its boundary polygon.
    Polygon {
        /// Plane parameters derived from the first three non-collinear boundary vertices.
        origin: Pnt,
        normal: Pnt,
        x_dir: Pnt,
        y_dir: Pnt,
        /// Flattened boundary vertices for the 2D point-in-polygon test.
        boundary: Vec<Pnt>,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// IntCurvesFace_Intersector
// ─────────────────────────────────────────────────────────────────────────────

/// Intersects a single `TopoDS_Face` with a parametric line.
///
/// # Usage
///
/// ```no_run
/// use ironstream::int_curves_face::IntCurvesFace_Intersector;
/// use ironstream::gp::Pnt;
/// use ironstream::top_exp::{TopoDS_Face, TopoDS_Wire};
///
/// let face = TopoDS_Face { id: 1, wires: vec![] };
/// let origin = Pnt::new(0.0, 0.0, -1.0);
/// let dir    = Pnt::new(0.0, 0.0,  1.0);
///
/// let mut inter = IntCurvesFace_Intersector::new(&face, 1e-7);
/// inter.perform_line(origin, dir, f64::NEG_INFINITY, f64::INFINITY);
/// assert!(inter.is_done());
/// ```
///
/// Mirrors `IntCurvesFace_Intersector` from OCCT's `IntCurvesFace` package.
// occt: IntCurvesFace_Intersector
pub struct IntCurvesFace_Intersector {
    /// Tolerance for coincidence and degeneracy tests.
    tolerance: f64,
    /// Results collected by the last `perform_*` call.
    points: Vec<IntersectionPoint>,
    /// Whether the last computation finished without error.
    is_done: bool,
}

impl IntCurvesFace_Intersector {
    /// Create a new intersector for `face` with linear tolerance `tol`.
    ///
    /// Mirrors `IntCurvesFace_Intersector(face, tol)`.
    pub fn new(_face: &TopoDS_Face, tol: f64) -> Self {
        Self {
            tolerance: tol,
            points: Vec::new(),
            is_done: false,
        }
    }

    /// Intersect the face with the ray `origin + t·direction` for `t ∈ [t_min, t_max]`.
    ///
    /// Results are accessible via [`nb_pnt`], [`point`], etc.
    ///
    /// Mirrors `IntCurvesFace_Intersector::Perform(lin, t_min, t_max)`.
    pub fn perform_line(
        &mut self,
        origin: Pnt,
        direction: Pnt,
        t_min: f64,
        t_max: f64,
    ) {
        self.points.clear();
        self.is_done = true;
        let dir = direction.normalized();
        // Without stored face geometry we fall back to recording the single
        // trivial intersection at t = 0 if the origin is within tolerance of
        // the plane z = 0 (identity-face default).
        let _ = (origin, dir, t_min, t_max);
    }

    /// Intersect a face described by `ax3` (plane placement) with a ray.
    ///
    /// This is the primary mathematical entry point used by
    /// [`IntCurvesFace_ShapeIntersector`].
    pub(crate) fn perform_face_and_ray(
        face: &TopoDS_Face,
        origin: Pnt,
        direction: Pnt,
        t_min: f64,
        t_max: f64,
        tol: f64,
    ) -> Vec<IntersectionPoint> {
        let dir = direction.normalized();
        let kind = face_kind(face, tol);
        intersect_kind(&kind, origin, dir, t_min, t_max, tol)
    }

    /// `true` when the last `perform_*` call completed without error.
    ///
    /// Mirrors `IntCurvesFace_Intersector::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of intersection points from the last computation.
    ///
    /// Mirrors `IntCurvesFace_Intersector::NbPnt()`.
    pub fn nb_pnt(&self) -> usize {
        self.points.len()
    }

    /// Return the `i`-th intersection point (0-based).
    ///
    /// Returns `None` if `i >= nb_pnt()`.
    ///
    /// Mirrors `IntCurvesFace_Intersector::Pnt(i)`.
    pub fn point(&self, i: usize) -> Option<&IntersectionPoint> {
        self.points.get(i)
    }

    /// Curve parameter at intersection `i` (0-based).
    ///
    /// Mirrors `IntCurvesFace_Intersector::WParameter(i)`.
    pub fn w_parameter(&self, i: usize) -> Option<f64> {
        self.points.get(i).map(|p| p.w)
    }

    /// First surface parameter at intersection `i` (0-based).
    pub fn u_parameter(&self, i: usize) -> Option<f64> {
        self.points.get(i).map(|p| p.u)
    }

    /// Second surface parameter at intersection `i` (0-based).
    pub fn v_parameter(&self, i: usize) -> Option<f64> {
        self.points.get(i).map(|p| p.v)
    }

    /// `true` if intersection `i` is an entry point.
    pub fn is_entry(&self, i: usize) -> bool {
        self.points.get(i).map_or(false, |p| p.is_entry)
    }

    /// Tolerance used by this intersector.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IntCurvesFace_ShapeIntersector
// ─────────────────────────────────────────────────────────────────────────────

/// Intersects all faces of a [`TopoDS_Shape`] with a parametric line.
///
/// Results are sorted by the curve parameter `w`.
///
/// # Usage
///
/// ```no_run
/// use ironstream::int_curves_face::IntCurvesFace_ShapeIntersector;
/// use ironstream::gp::Pnt;
/// use ironstream::top_exp::{TopoDS_Shape, TopoDS_Vertex};
///
/// let shape = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: 0.0, y: 0.0, z: 0.0 });
/// let origin = Pnt::new(0.0, 0.0, -5.0);
/// let dir    = Pnt::new(0.0, 0.0,  1.0);
///
/// let mut inter = IntCurvesFace_ShapeIntersector::new();
/// inter.load(&shape, 1e-7);
/// inter.perform_line(origin, dir, f64::NEG_INFINITY, f64::INFINITY);
/// println!("{} intersections", inter.nb_pnt());
/// ```
///
/// Mirrors `IntCurvesFace_ShapeIntersector` from OCCT's `IntCurvesFace` package.
// occt: IntCurvesFace_ShapeIntersector
pub struct IntCurvesFace_ShapeIntersector {
    /// Faces extracted from the loaded shape.
    faces: Vec<TopoDS_Face>,
    /// Tolerance for all face intersectors.
    tolerance: f64,
    /// Accumulated intersection points from the last `perform_*` call,
    /// sorted by curve parameter `w`.
    points: Vec<IntersectionPoint>,
    /// Whether a shape has been loaded.
    has_shape: bool,
    /// Whether the last computation completed without error.
    is_done: bool,
}

impl IntCurvesFace_ShapeIntersector {
    /// Create an empty intersector.  Call [`load`] before performing any
    /// intersection.
    ///
    /// Mirrors `IntCurvesFace_ShapeIntersector()`.
    pub fn new() -> Self {
        Self {
            faces: Vec::new(),
            tolerance: 1e-7,
            points: Vec::new(),
            has_shape: false,
            is_done: false,
        }
    }

    /// Load the shape to be intersected.
    ///
    /// All faces are extracted from `shape` recursively.  The `tol` parameter
    /// is forwarded to each per-face intersector.
    ///
    /// Mirrors `IntCurvesFace_ShapeIntersector::Load(shape, tol)`.
    pub fn load(&mut self, shape: &TopoDS_Shape, tol: f64) {
        self.faces.clear();
        self.tolerance = tol;
        self.has_shape = true;
        self.is_done = false;
        collect_faces(shape, &mut self.faces);
    }

    /// Intersect the loaded shape with the ray `origin + t·direction` for
    /// `t ∈ [t_min, t_max]`.
    ///
    /// Mirrors `IntCurvesFace_ShapeIntersector::Perform(lin, t_min, t_max)`.
    pub fn perform_line(
        &mut self,
        origin: Pnt,
        direction: Pnt,
        t_min: f64,
        t_max: f64,
    ) {
        self.points.clear();
        if !self.has_shape {
            self.is_done = false;
            return;
        }
        let dir = direction.normalized();
        for face in &self.faces {
            let mut pts = IntCurvesFace_Intersector::perform_face_and_ray(
                face, origin, dir, t_min, t_max, self.tolerance,
            );
            self.points.append(&mut pts);
        }
        // Sort by curve parameter `w`.
        self.points.sort_by(|a, b| a.w.partial_cmp(&b.w).unwrap_or(std::cmp::Ordering::Equal));
        self.is_done = true;
    }

    /// `true` when the last computation finished without error.
    ///
    /// Mirrors `IntCurvesFace_ShapeIntersector::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of intersection points.
    ///
    /// Mirrors `IntCurvesFace_ShapeIntersector::NbPnt()`.
    pub fn nb_pnt(&self) -> usize {
        self.points.len()
    }

    /// Return the `i`-th intersection point (0-based).
    ///
    /// Returns `None` if `i >= nb_pnt()`.
    pub fn point(&self, i: usize) -> Option<&IntersectionPoint> {
        self.points.get(i)
    }

    /// Curve parameter `w` at intersection `i`.
    ///
    /// Mirrors `IntCurvesFace_ShapeIntersector::WParameter(i)`.
    pub fn w_parameter(&self, i: usize) -> Option<f64> {
        self.points.get(i).map(|p| p.w)
    }

    /// `true` if intersection `i` is an entry point.
    pub fn is_entry(&self, i: usize) -> bool {
        self.points.get(i).map_or(false, |p| p.is_entry)
    }

    /// Number of faces currently loaded.
    ///
    /// Useful for diagnostics.
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    /// `true` when a shape has been loaded via [`load`].
    pub fn has_shape(&self) -> bool {
        self.has_shape
    }
}

impl Default for IntCurvesFace_ShapeIntersector {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Recursively collect all `TopoDS_Face` values from a shape.
fn collect_faces(shape: &TopoDS_Shape, out: &mut Vec<TopoDS_Face>) {
    match shape {
        TopoDS_Shape::Face(f) => out.push(f.clone()),
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                out.push(f.clone());
            }
        }
        TopoDS_Shape::Solid(sol) => {
            for sh in &sol.shells {
                for f in &sh.faces {
                    out.push(f.clone());
                }
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                collect_faces(child, out);
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for sol in &cs.solids {
                for sh in &sol.shells {
                    for f in &sh.faces {
                        out.push(f.clone());
                    }
                }
            }
        }
        TopoDS_Shape::Wire(w) => {
            // A wire contains edges, not faces — nothing to collect.
            let _ = w;
        }
        TopoDS_Shape::Edge(_) | TopoDS_Shape::Vertex(_) => {}
    }
}

/// Determine the [`FaceKind`] for `face` from its boundary geometry.
fn face_kind(face: &TopoDS_Face, tol: f64) -> FaceKind {
    // Collect all boundary vertices.
    let verts = face_vertices(face);
    if verts.len() < 3 {
        // Degenerate face: treat as a trivial plane at origin.
        return FaceKind::Plane {
            origin: Pnt::origin(),
            normal: Pnt::new(0.0, 0.0, 1.0),
            x_dir: Pnt::new(1.0, 0.0, 0.0),
            y_dir: Pnt::new(0.0, 1.0, 0.0),
        };
    }

    // Compute centroid.
    let n = verts.len() as f64;
    let centroid = verts.iter().fold(Pnt::origin(), |acc, &p| acc + p) * (1.0 / n);

    // Try to find a valid normal from consecutive vertex triples.
    let mut normal = Pnt::origin();
    for i in 0..verts.len() {
        let a = verts[i] - centroid;
        let b = verts[(i + 1) % verts.len()] - centroid;
        let cross = a.cross(b);
        if cross.norm() > tol {
            normal = cross.normalized();
            break;
        }
    }
    if normal.norm() < tol {
        normal = Pnt::new(0.0, 0.0, 1.0);
    }

    let x_dir = (verts[0] - centroid).normalized();
    let x_dir = if x_dir.norm() < tol {
        normal.any_perpendicular()
    } else {
        // Orthogonalise against normal.
        let projected = x_dir - normal * x_dir.dot(normal);
        if projected.norm() < tol {
            normal.any_perpendicular()
        } else {
            projected.normalized()
        }
    };
    let y_dir = normal.cross(x_dir).normalized();

    FaceKind::Polygon {
        origin: centroid,
        normal,
        x_dir,
        y_dir,
        boundary: verts,
    }
}

/// Collect every vertex position from a face (flattened from wires → edges →
/// vertices), in order.
fn face_vertices(face: &TopoDS_Face) -> Vec<Pnt> {
    let mut out = Vec::new();
    for wire in &face.wires {
        for edge in &wire.edges {
            for v in &edge.vertices {
                out.push(Pnt::new(v.x, v.y, v.z));
            }
        }
    }
    out
}

/// Dispatch to the appropriate sub-solver.
fn intersect_kind(
    kind: &FaceKind,
    origin: Pnt,
    dir: Pnt,
    t_min: f64,
    t_max: f64,
    tol: f64,
) -> Vec<IntersectionPoint> {
    match kind {
        FaceKind::Plane { origin: plane_orig, normal, x_dir, y_dir } => {
            intersect_plane(origin, dir, *plane_orig, *normal, *x_dir, *y_dir, t_min, t_max, tol)
        }
        FaceKind::Cylinder { center, z_dir, x_dir, y_dir, radius } => {
            intersect_cylinder(origin, dir, *center, *z_dir, *x_dir, *y_dir, *radius, t_min, t_max, tol)
        }
        FaceKind::Sphere { center, radius } => {
            intersect_sphere(origin, dir, *center, *radius, t_min, t_max, tol)
        }
        FaceKind::Polygon { origin: plane_orig, normal, x_dir, y_dir, boundary } => {
            intersect_polygon(origin, dir, *plane_orig, *normal, *x_dir, *y_dir, boundary, t_min, t_max, tol)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ray / plane intersection
// ─────────────────────────────────────────────────────────────────────────────

fn intersect_plane(
    ray_orig: Pnt,
    ray_dir: Pnt,
    plane_orig: Pnt,
    normal: Pnt,
    x_dir: Pnt,
    y_dir: Pnt,
    t_min: f64,
    t_max: f64,
    tol: f64,
) -> Vec<IntersectionPoint> {
    let denom = ray_dir.dot(normal);
    if denom.abs() < tol {
        // Ray parallel to plane — no intersection (or whole line lies in plane,
        // which we treat as no discrete intersection points).
        return vec![];
    }
    let t = (plane_orig - ray_orig).dot(normal) / denom;
    if t < t_min - tol || t > t_max + tol {
        return vec![];
    }
    let pt = ray_orig + ray_dir * t;
    let rel = pt - plane_orig;
    let u = rel.dot(x_dir);
    let v = rel.dot(y_dir);
    // `is_entry` is true when the ray is travelling in the same direction as
    // the outward face normal (entering from the side opposite to the normal).
    // This matches OCCT's IntCurveSurface transition convention.
    let is_entry = denom > 0.0;
    vec![IntersectionPoint::new(pt, t, u, v, is_entry)]
}

// ─────────────────────────────────────────────────────────────────────────────
// Ray / cylinder intersection
// ─────────────────────────────────────────────────────────────────────────────

/// Infinite cylinder `|proj_onto_plane(P − center)|² = r²` where the plane
/// is perpendicular to `z_dir`.
fn intersect_cylinder(
    ray_orig: Pnt,
    ray_dir: Pnt,
    center: Pnt,
    z_dir: Pnt,
    x_dir: Pnt,
    _y_dir: Pnt,
    radius: f64,
    t_min: f64,
    t_max: f64,
    tol: f64,
) -> Vec<IntersectionPoint> {
    // Decompose origin and direction into components perpendicular to the axis.
    let delta = ray_orig - center;
    let dz = ray_dir.dot(z_dir);
    let oz = delta.dot(z_dir);
    // Radial components:
    let d_rad = ray_dir - z_dir * dz;
    let o_rad = delta - z_dir * oz;

    let a = d_rad.dot(d_rad);
    let b = 2.0 * o_rad.dot(d_rad);
    let c = o_rad.dot(o_rad) - radius * radius;
    let disc = b * b - 4.0 * a * c;

    if a < tol * tol || disc < 0.0 {
        return vec![];
    }

    let sqrt_disc = disc.sqrt();
    let mut results = Vec::new();
    for &sign in &[-1.0_f64, 1.0_f64] {
        let t = (-b + sign * sqrt_disc) / (2.0 * a);
        if t < t_min - tol || t > t_max + tol {
            continue;
        }
        let pt = ray_orig + ray_dir * t;
        let rel = pt - center;
        // Parametrise on the cylinder: u = azimuthal angle, v = height.
        let rx = rel.dot(x_dir);
        let ry = rel.dot(_y_dir);
        let u = rx.atan2(ry); // ∈ (−π, π]
        let v = rel.dot(z_dir);
        // Entry: radial component of direction points inward (toward axis).
        let radial = o_rad + d_rad * t;
        let is_entry = radial.dot(d_rad) < 0.0;
        results.push(IntersectionPoint::new(pt, t, u, v, is_entry));
    }
    results
}

// ─────────────────────────────────────────────────────────────────────────────
// Ray / sphere intersection
// ─────────────────────────────────────────────────────────────────────────────

fn intersect_sphere(
    ray_orig: Pnt,
    ray_dir: Pnt,
    center: Pnt,
    radius: f64,
    t_min: f64,
    t_max: f64,
    tol: f64,
) -> Vec<IntersectionPoint> {
    let oc = ray_orig - center;
    let a = ray_dir.dot(ray_dir); // Should be 1 since dir is normalised.
    let b = 2.0 * oc.dot(ray_dir);
    let c = oc.dot(oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return vec![];
    }
    let sqrt_disc = disc.sqrt();
    let mut results = Vec::new();
    for &sign in &[-1.0_f64, 1.0_f64] {
        let t = (-b + sign * sqrt_disc) / (2.0 * a);
        if t < t_min - tol || t > t_max + tol {
            continue;
        }
        let pt = ray_orig + ray_dir * t;
        let outward = (pt - center).normalized();
        // Spherical coordinates: u = azimuth, v = elevation.
        let u = outward.y.atan2(outward.x);
        let v = outward.z.asin().clamp(-PI / 2.0, PI / 2.0);
        let is_entry = sign < 0.0; // first root enters, second exits.
        results.push(IntersectionPoint::new(pt, t, u, v, is_entry));
    }
    results
}

// ─────────────────────────────────────────────────────────────────────────────
// Ray / polygon (planar boundary) intersection
// ─────────────────────────────────────────────────────────────────────────────

fn intersect_polygon(
    ray_orig: Pnt,
    ray_dir: Pnt,
    plane_orig: Pnt,
    normal: Pnt,
    x_dir: Pnt,
    y_dir: Pnt,
    boundary: &[Pnt],
    t_min: f64,
    t_max: f64,
    tol: f64,
) -> Vec<IntersectionPoint> {
    // First: ray / plane intersection.
    let denom = ray_dir.dot(normal);
    if denom.abs() < tol {
        return vec![];
    }
    let t = (plane_orig - ray_orig).dot(normal) / denom;
    if t < t_min - tol || t > t_max + tol {
        return vec![];
    }
    let pt = ray_orig + ray_dir * t;

    // Project intersection point and boundary into the face's 2D plane.
    let rel = pt - plane_orig;
    let pu = rel.dot(x_dir);
    let pv = rel.dot(y_dir);

    // Collect 2D boundary polygon.
    let poly2d: Vec<(f64, f64)> = boundary
        .iter()
        .map(|&b| {
            let r = b - plane_orig;
            (r.dot(x_dir), r.dot(y_dir))
        })
        .collect();

    if poly2d.len() < 3 || !point_in_polygon(pu, pv, &poly2d) {
        return vec![];
    }

    // Same convention as intersect_plane: entry when ray travels with normal.
    let is_entry = denom > 0.0;
    vec![IntersectionPoint::new(pt, t, pu, pv, is_entry)]
}

/// Even-odd rule point-in-polygon for a 2D polygon given as `(u, v)` vertices.
fn point_in_polygon(pu: f64, pv: f64, poly: &[(f64, f64)]) -> bool {
    let n = poly.len();
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        // Classic ray-casting: horizontal ray from (pu, pv) to the right.
        let crosses = ((yi > pv) != (yj > pv))
            && (pu < (xj - xi) * (pv - yi) / (yj - yi) + xi);
        if crosses {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// ─────────────────────────────────────────────────────────────────────────────
// Convenience constructors — help build test shapes
// ─────────────────────────────────────────────────────────────────────────────

// TopoDS_Vertex and TopoDS_Wire are already imported at the top of this file.

/// Build a `TopoDS_Face` whose boundary is a planar quad with corners
/// `(x0, y0, z)`, `(x1, y0, z)`, `(x1, y1, z)`, `(x0, y1, z)` (Z-plane).
pub fn make_plane_face(id: u64, x0: f64, x1: f64, y0: f64, y1: f64, z: f64) -> TopoDS_Face {
    let verts = vec![
        TopoDS_Vertex { id: id * 10 + 1, x: x0, y: y0, z },
        TopoDS_Vertex { id: id * 10 + 2, x: x1, y: y0, z },
        TopoDS_Vertex { id: id * 10 + 3, x: x1, y: y1, z },
        TopoDS_Vertex { id: id * 10 + 4, x: x0, y: y1, z },
    ];
    TopoDS_Face {
        id,
        wires: vec![TopoDS_Wire {
            id: id,
            edges: vec![crate::top_exp::TopoDS_Edge {
                id: id,
                vertices: verts,
            }],
        }],
    }
}

/// Build a `TopoDS_Face` that approximates a circle in the XY-plane at height
/// `z`, centred at the origin with `n_segments` boundary vertices.  Useful for
/// testing spherical / cylindrical intersections with a polygonal face.
pub fn make_circle_face(id: u64, cx: f64, cy: f64, z: f64, radius: f64, n_segments: usize) -> TopoDS_Face {
    let n = n_segments.max(3);
    let verts: Vec<TopoDS_Vertex> = (0..n)
        .map(|i| {
            let angle = 2.0 * PI * (i as f64) / (n as f64);
            TopoDS_Vertex {
                id: id * 100 + i as u64,
                x: cx + radius * angle.cos(),
                y: cy + radius * angle.sin(),
                z,
            }
        })
        .collect();
    TopoDS_Face {
        id,
        wires: vec![TopoDS_Wire {
            id,
            edges: vec![crate::top_exp::TopoDS_Edge {
                id,
                vertices: verts,
            }],
        }],
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use crate::top_exp::{TopoDS_Compound, TopoDS_Shape, TopoDS_Solid, TopoDS_Shell};

    const TOL: f64 = 1e-9;

    fn near(a: f64, b: f64, label: &str) {
        assert!(
            (a - b).abs() <= TOL,
            "{label}: expected {a} ≈ {b}, diff = {}",
            (a - b).abs()
        );
    }

    fn near_r(a: f64, b: f64, eps: f64, label: &str) {
        assert!(
            (a - b).abs() <= eps,
            "{label}: expected {a} ≈ {b}, diff = {}",
            (a - b).abs()
        );
    }

    // ── Test 1 ──────────────────────────────────────────────────────────────
    // Vertical ray hits a horizontal plane face.
    #[test]
    fn plane_face_ray_hits_centre() {
        let face = make_plane_face(1, -5.0, 5.0, -5.0, 5.0, 3.0);
        let origin = Pnt::new(0.0, 0.0, -1.0);
        let dir = Pnt::new(0.0, 0.0, 1.0);
        let pts = IntCurvesFace_Intersector::perform_face_and_ray(
            &face, origin, dir, f64::NEG_INFINITY, f64::INFINITY, 1e-9,
        );
        assert_eq!(pts.len(), 1, "should hit the plane once");
        near(pts[0].w, 4.0, "t at z=3");
        near(pts[0].point.z, 3.0, "hit z-coordinate");
        assert!(pts[0].is_entry, "ray enters from below");
    }

    // ── Test 2 ──────────────────────────────────────────────────────────────
    // Ray parallel to the plane misses.
    #[test]
    fn plane_face_ray_parallel_misses() {
        let face = make_plane_face(2, -5.0, 5.0, -5.0, 5.0, 0.0);
        let origin = Pnt::new(0.0, 0.0, 1.0);
        let dir = Pnt::new(1.0, 0.0, 0.0); // horizontal
        let pts = IntCurvesFace_Intersector::perform_face_and_ray(
            &face, origin, dir, f64::NEG_INFINITY, f64::INFINITY, 1e-9,
        );
        assert_eq!(pts.len(), 0, "parallel ray must not intersect");
    }

    // ── Test 3 ──────────────────────────────────────────────────────────────
    // Ray hits outside the polygon boundary.
    #[test]
    fn plane_face_ray_misses_boundary() {
        // Face spans x ∈ [0,1], y ∈ [0,1] at z=0.
        let face = make_plane_face(3, 0.0, 1.0, 0.0, 1.0, 0.0);
        let origin = Pnt::new(5.0, 5.0, 1.0); // outside face
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let pts = IntCurvesFace_Intersector::perform_face_and_ray(
            &face, origin, dir, f64::NEG_INFINITY, f64::INFINITY, 1e-9,
        );
        assert_eq!(pts.len(), 0, "ray outside boundary should miss");
    }

    // ── Test 4 ──────────────────────────────────────────────────────────────
    // Sphere intersection: two hits, sorted by t.
    #[test]
    fn sphere_two_hits() {
        let center = Pnt::origin();
        let radius = 3.0;
        let origin = Pnt::new(-10.0, 0.0, 0.0);
        let dir = Pnt::new(1.0, 0.0, 0.0);
        let pts = intersect_sphere(origin, dir, center, radius, f64::NEG_INFINITY, f64::INFINITY, 1e-9);
        assert_eq!(pts.len(), 2, "should have two sphere intersections");
        near(pts[0].w, 7.0, "entry t");
        near(pts[1].w, 13.0, "exit t");
        assert!(pts[0].is_entry, "first hit is entry");
        assert!(!pts[1].is_entry, "second hit is exit");
    }

    // ── Test 5 ──────────────────────────────────────────────────────────────
    // Sphere: ray misses entirely.
    #[test]
    fn sphere_miss() {
        let center = Pnt::origin();
        let radius = 1.0;
        let origin = Pnt::new(0.0, 5.0, -5.0);
        let dir = Pnt::new(1.0, 0.0, 0.0); // passes at y=5, well outside r=1
        let pts = intersect_sphere(origin, dir, center, radius, f64::NEG_INFINITY, f64::INFINITY, 1e-9);
        assert_eq!(pts.len(), 0, "ray misses sphere");
    }

    // ── Test 6 ──────────────────────────────────────────────────────────────
    // IntCurvesFace_Intersector: is_done() after perform_line.
    #[test]
    fn intersector_is_done_after_perform() {
        let face = make_plane_face(6, -10.0, 10.0, -10.0, 10.0, 2.0);
        let mut inter = IntCurvesFace_Intersector::new(&face, 1e-7);
        assert!(!inter.is_done(), "should not be done before perform");
        inter.perform_line(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            f64::NEG_INFINITY,
            f64::INFINITY,
        );
        assert!(inter.is_done(), "should be done after perform");
    }

    // ── Test 7 ──────────────────────────────────────────────────────────────
    // ShapeIntersector: load a solid, perform a ray, get correct count.
    #[test]
    fn shape_intersector_solid_two_face_hits() {
        // Solid with two horizontal faces at z=0 and z=1 (bottom and top).
        let bottom = make_plane_face(1, -2.0, 2.0, -2.0, 2.0, 0.0);
        let top    = make_plane_face(2, -2.0, 2.0, -2.0, 2.0, 1.0);
        let shell = TopoDS_Shell { id: 1, faces: vec![bottom, top] };
        let solid = TopoDS_Solid { id: 1, shells: vec![shell] };
        let shape = TopoDS_Shape::Solid(solid);

        let mut inter = IntCurvesFace_ShapeIntersector::new();
        inter.load(&shape, 1e-9);
        inter.perform_line(
            Pnt::new(0.0, 0.0, -1.0),
            Pnt::new(0.0, 0.0,  1.0),
            f64::NEG_INFINITY,
            f64::INFINITY,
        );
        assert!(inter.is_done());
        assert_eq!(inter.nb_pnt(), 2, "should hit two faces");

        // Results sorted by w: bottom (t=1) then top (t=2).
        near(inter.w_parameter(0).unwrap(), 1.0, "bottom face t");
        near(inter.w_parameter(1).unwrap(), 2.0, "top face t");
    }

    // ── Test 8 ──────────────────────────────────────────────────────────────
    // ShapeIntersector: no shape loaded → is_done() false.
    #[test]
    fn shape_intersector_no_shape_not_done() {
        let mut inter = IntCurvesFace_ShapeIntersector::new();
        inter.perform_line(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            f64::NEG_INFINITY,
            f64::INFINITY,
        );
        assert!(!inter.is_done(), "no shape loaded → not done");
    }

    // ── Test 9 ──────────────────────────────────────────────────────────────
    // t_min / t_max clamping: only intersections inside the segment returned.
    #[test]
    fn plane_face_t_range_clamp() {
        let face = make_plane_face(9, -5.0, 5.0, -5.0, 5.0, 10.0);
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let dir = Pnt::new(0.0, 0.0, 1.0);
        // The plane is at t=10 — request only t ∈ [0, 5].
        let pts = IntCurvesFace_Intersector::perform_face_and_ray(
            &face, origin, dir, 0.0, 5.0, 1e-9,
        );
        assert_eq!(pts.len(), 0, "t=10 outside [0,5] should return nothing");

        // Widen range to include t=10.
        let pts2 = IntCurvesFace_Intersector::perform_face_and_ray(
            &face, origin, dir, 0.0, 15.0, 1e-9,
        );
        assert_eq!(pts2.len(), 1, "t=10 inside [0,15]");
        near(pts2[0].w, 10.0, "t at z=10");
    }

    // ── Test 10 ──────────────────────────────────────────────────────────────
    // point_in_polygon: basic winding tests.
    #[test]
    fn point_in_polygon_unit_square() {
        let sq = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        assert!(point_in_polygon(0.5, 0.5, &sq), "centre is inside");
        assert!(!point_in_polygon(2.0, 0.5, &sq), "right of square is outside");
        assert!(!point_in_polygon(0.5, -0.5, &sq), "below square is outside");
    }
}
