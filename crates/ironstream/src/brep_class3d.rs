// FILE: brep_class3d.rs
//! `BRepClass3d` — solid classifier for B-Rep shapes, mirroring OpenCascade's
//! `BRepClass3d` package.
//!
//! Provides point-in-solid queries using a ray-casting algorithm over the
//! triangulated faces of the solid.  The algorithm fires a semi-infinite ray
//! from the query point and counts how many closed-solid face triangles it
//! crosses; an odd count means the point is inside.
//!
//! # Key types
//!
//! * [`BRepClass3d_SolidClassifier`] — the primary query object.  Load a shape
//!   once, then call [`BRepClass3d_SolidClassifier::perform`] for each query
//!   point.
//! * [`BRepClass3d_SolidExplorer`] — iterates over the faces of a solid
//!   exposed as [`SolidFace`] records (each face is a collection of triangles
//!   with a computed outward normal).
//!
//! # Algorithm
//!
//! The point-in-solid test is the **ray-casting** (Jordan curve) method in 3D:
//!
//! 1. Fire a ray from the query point `P` along a stable direction `+X` (with
//!    a small perturbation to avoid axis-aligned degeneracies).
//! 2. For every triangle in the solid boundary, test ray–triangle intersection
//!    using the Möller–Trumbore algorithm.
//! 3. Count intersections with `t > 0` (forward ray hits).
//! 4. Odd count → [`State::IN`]; even → [`State::OUT`].
//! 5. If any hit has `t` < tolerance, classify as [`State::ON`].
//!
//! This is the same strategy used in OCCT's `BRepClass3d_SolidClassifier`.

use crate::gp::{Pnt, EPS};
use crate::top_abs::State;
use crate::top_exp::{
    TopoDS_Face, TopoDS_Shape, TopoDS_Shell, TopoDS_Solid, TopoDS_Wire,
};

// ─────────────────────────── helpers ─────────────────────────────────────────

/// Compute a unit normal for a triangle given its three vertices.
fn triangle_normal(a: Pnt, b: Pnt, c: Pnt) -> Pnt {
    let ab = b - a;
    let ac = c - a;
    ab.cross(ac).normalized()
}

/// Möller–Trumbore ray–triangle intersection.
///
/// Returns `Some(t)` where `t` is the parameter along the ray `origin +
/// t * dir` at the hit point, or `None` if there is no intersection.
///
/// The hit is considered valid for `t >= -tol` (slightly behind the origin to
/// catch on-surface points).  Back-face hits are included because we need a
/// watertight count.
fn ray_triangle_intersect(
    origin: Pnt,
    dir: Pnt,
    v0: Pnt,
    v1: Pnt,
    v2: Pnt,
    tol: f64,
) -> Option<f64> {
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let h = dir.cross(e2);
    let a = e1.dot(h);
    if a.abs() < 1e-14 {
        // Ray is parallel to the triangle plane.
        return None;
    }
    let f = 1.0 / a;
    let s = origin - v0;
    let u = f * s.dot(h);
    if !((-tol..=1.0 + tol).contains(&u)) {
        return None;
    }
    let q = s.cross(e1);
    let v = f * dir.dot(q);
    if v < -tol || u + v > 1.0 + tol {
        return None;
    }
    let t = f * e2.dot(q);
    Some(t)
}

// ─────────────────────────── SolidFace ───────────────────────────────────────

/// A single face of a solid expressed as a flat list of triangles (each
/// triangle is three [`Pnt`]s) plus the face's average outward normal.
///
/// Triangulation is produced from the wire vertices of a [`TopoDS_Face`] using
/// a simple fan triangulation of the outer wire.
// occt: BRepClass3d_SolidExplorer // (per-face data)
#[derive(Clone, Debug)]
pub struct SolidFace {
    /// Pre-computed outward unit normal.
    pub normal: Pnt,
    /// Flat triangle list: every three consecutive entries form one triangle.
    pub triangles: Vec<[Pnt; 3]>,
}

impl SolidFace {
    /// Build a [`SolidFace`] from a [`TopoDS_Face`] using fan triangulation of
    /// the outer wire.
    fn from_topo_face(face: &TopoDS_Face) -> Option<Self> {
        let wire = face.wires.first()?;
        let pts: Vec<Pnt> = wire
            .edges
            .iter()
            .filter_map(|e| e.vertices.first())
            .map(|v| Pnt::new(v.x, v.y, v.z))
            .collect();
        if pts.len() < 3 {
            return None;
        }
        // Fan triangulation from pts[0].
        let mut triangles: Vec<[Pnt; 3]> = Vec::with_capacity(pts.len() - 2);
        for i in 1..pts.len() - 1 {
            triangles.push([pts[0], pts[i], pts[i + 1]]);
        }
        // Average normal over all triangles.
        let mut n = Pnt::origin();
        for tri in &triangles {
            n = n + triangle_normal(tri[0], tri[1], tri[2]);
        }
        let normal = n.normalized();
        Some(SolidFace { normal, triangles })
    }
}

// ─────────────────────────── BRepClass3d_SolidExplorer ───────────────────────

/// Iterates over all faces of a solid (or a compound of solids) and exposes
/// them as [`SolidFace`] records suitable for point-in-solid classification.
///
/// Mirrors `BRepClass3d_SolidExplorer` from OCCT.  In OCCT the explorer
/// provides `InitShell`, `MoreShell`, `NextShell`, etc.; here we provide an
/// ergonomic Rust iterator interface instead while keeping the same OCCT-shaped
/// constructor and named accessors.
// occt: BRepClass3d_SolidExplorer
#[derive(Clone, Debug)]
pub struct BRepClass3d_SolidExplorer {
    /// All faces extracted from the shape.
    faces: Vec<SolidFace>,
    /// Iterator index.
    index: usize,
}

impl BRepClass3d_SolidExplorer {
    /// Construct the explorer from any [`TopoDS_Shape`].
    ///
    /// All solid and shell sub-shapes are visited and their faces collected.
    pub fn new(shape: &TopoDS_Shape) -> Self {
        let mut faces = Vec::new();
        collect_faces(shape, &mut faces);
        BRepClass3d_SolidExplorer { faces, index: 0 }
    }

    /// `InitShell` analogue — reset the iterator to the beginning.
    pub fn init(&mut self) {
        self.index = 0;
    }

    /// Returns `true` while there are more faces to visit.
    pub fn more(&self) -> bool {
        self.index < self.faces.len()
    }

    /// Advance to the next face.
    pub fn next(&mut self) {
        if self.index < self.faces.len() {
            self.index += 1;
        }
    }

    /// Return a reference to the current face.  Panics if `!more()`.
    pub fn current_face(&self) -> &SolidFace {
        &self.faces[self.index]
    }

    /// Total number of collected faces.
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    /// Immutable slice of all collected faces.
    pub fn faces(&self) -> &[SolidFace] {
        &self.faces
    }
}

/// Recursively collect [`SolidFace`]s from any shape.
fn collect_faces(shape: &TopoDS_Shape, out: &mut Vec<SolidFace>) {
    match shape {
        TopoDS_Shape::Solid(s) => collect_from_solid(s, out),
        TopoDS_Shape::Shell(sh) => {
            for face in &sh.faces {
                if let Some(sf) = SolidFace::from_topo_face(face) {
                    out.push(sf);
                }
            }
        }
        TopoDS_Shape::Face(f) => {
            if let Some(sf) = SolidFace::from_topo_face(f) {
                out.push(sf);
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                collect_faces(child, out);
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for solid in &cs.solids {
                collect_from_solid(solid, out);
            }
        }
        _ => {}
    }
}

fn collect_from_solid(solid: &TopoDS_Solid, out: &mut Vec<SolidFace>) {
    for shell in &solid.shells {
        for face in &shell.faces {
            if let Some(sf) = SolidFace::from_topo_face(face) {
                out.push(sf);
            }
        }
    }
}

// ─────────────────────────── BRepClass3d_SolidClassifier ─────────────────────

/// Classifies 3-D points against the interior of a B-Rep solid.
///
/// # Usage (mirrors OCCT)
///
/// ```rust
/// use ironstream::brep_class3d::BRepClass3d_SolidClassifier;
/// use ironstream::top_exp::{TopoDS_Shape, TopoDS_Solid, TopoDS_Shell,
///                           TopoDS_Face, TopoDS_Wire, TopoDS_Edge, TopoDS_Vertex};
/// use ironstream::gp::Pnt;
///
/// // build a minimal solid (just the structure — real geometry omitted here)
/// let shape = TopoDS_Shape::Solid(TopoDS_Solid { id: 1, shells: vec![] });
/// let mut clf = BRepClass3d_SolidClassifier::new();
/// clf.load(&shape);
/// clf.perform(Pnt::origin(), 1e-7);
/// let state = clf.state();
/// ```
///
/// `perform` may be called repeatedly for different points; each call
/// overwrites the cached result.
// occt: BRepClass3d_SolidClassifier
#[derive(Debug)]
pub struct BRepClass3d_SolidClassifier {
    /// All faces collected from the most recently loaded shape.
    faces: Vec<SolidFace>,
    /// Classification result of the last `perform` call.
    state: State,
    /// Whether a shape has been loaded.
    is_loaded: bool,
    /// Whether `perform` has been called at least once since loading.
    is_done: bool,
    /// Rejection threshold used in the last `perform`.
    tolerance: f64,
}

impl BRepClass3d_SolidClassifier {
    /// Create an empty classifier (no shape loaded yet).
    pub fn new() -> Self {
        BRepClass3d_SolidClassifier {
            faces: Vec::new(),
            state: State::UNKNOWN,
            is_loaded: false,
            is_done: false,
            tolerance: EPS,
        }
    }

    /// Load a shape for classification.  Previous results are cleared.
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::Load(shape)`.
    pub fn load(&mut self, shape: &TopoDS_Shape) {
        self.faces.clear();
        collect_faces(shape, &mut self.faces);
        self.state = State::UNKNOWN;
        self.is_loaded = true;
        self.is_done = false;
    }

    /// Classify the point `p` against the loaded solid.
    ///
    /// `tol` is used as the on-surface threshold: if the closest ray hit has
    /// `|t| < tol` the point is classified as [`State::ON`].
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::Perform(point, tol)`.
    pub fn perform(&mut self, p: Pnt, tol: f64) {
        self.tolerance = tol;
        self.is_done = true;
        if !self.is_loaded || self.faces.is_empty() {
            self.state = State::OUT;
            return;
        }
        self.state = classify_point(&self.faces, p, tol);
    }

    /// Returns the classification result of the last [`Self::perform`] call.
    ///
    /// Returns [`State::UNKNOWN`] if `perform` has not yet been called.
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::State()`.
    pub fn state(&self) -> State {
        self.state
    }

    /// Returns `true` if [`Self::perform`] has been called at least once.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns `true` if the last classification returned [`State::IN`].
    pub fn is_in_solid(&self) -> bool {
        self.state == State::IN
    }

    /// Returns `true` if the last classification returned [`State::ON`].
    pub fn is_on_boundary(&self) -> bool {
        self.state == State::ON
    }

    /// Returns `true` if the last classification returned [`State::OUT`].
    pub fn is_outside(&self) -> bool {
        self.state == State::OUT
    }

    /// Returns the tolerance used in the last `perform` call.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Convenience: load shape and classify a single point.
    ///
    /// Equivalent to calling `load` then `perform`.
    pub fn load_and_perform(&mut self, shape: &TopoDS_Shape, p: Pnt, tol: f64) -> State {
        self.load(shape);
        self.perform(p, tol);
        self.state
    }
}

impl Default for BRepClass3d_SolidClassifier {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── core classification ─────────────────────────────

/// Ray-cast a point against a set of `SolidFace`s and return its
/// [`State`].
///
/// Uses a slightly perturbed `+X` ray to avoid axis-aligned degeneracies with
/// typical OCCT-style geometry.
fn classify_point(faces: &[SolidFace], p: Pnt, tol: f64) -> State {
    // Ray direction: slightly off-axis to avoid edge/vertex degenerate cases.
    let dir = Pnt::new(1.0, 1.732_050_8e-4, 1.414_213_5e-4).normalized();

    let mut crossings = 0usize;
    let mut on_surface = false;

    for face in faces {
        for tri in &face.triangles {
            if let Some(t) = ray_triangle_intersect(p, dir, tri[0], tri[1], tri[2], tol) {
                if t.abs() < tol {
                    on_surface = true;
                }
                if t > -tol {
                    crossings += 1;
                }
            }
        }
    }

    if on_surface {
        State::ON
    } else if crossings % 2 == 1 {
        State::IN
    } else {
        State::OUT
    }
}

// ─────────────────────────── free functions ──────────────────────────────────

/// Convenience free function: classify `point` against `shape` with a
/// given `tolerance`.
///
/// Returns a [`State`] directly.
pub fn classify(shape: &TopoDS_Shape, point: Pnt, tolerance: f64) -> State {
    let mut clf = BRepClass3d_SolidClassifier::new();
    clf.load(shape);
    clf.perform(point, tolerance);
    clf.state()
}

// ─────────────────────────── unit tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use crate::top_abs::State;
    use crate::top_exp::{
        TopoDS_Edge, TopoDS_Face, TopoDS_Shell, TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
        TopoDS_Shape,
    };

    // ── helpers ──────────────────────────────────────────────────────────────

    fn vtx(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
        TopoDS_Vertex { id, x, y, z }
    }

    fn edge_between(id: u64, a: TopoDS_Vertex, b: TopoDS_Vertex) -> TopoDS_Edge {
        TopoDS_Edge { id, vertices: vec![a, b] }
    }

    /// Build a triangular face from three (x,y,z) tuples.
    fn tri_face(id: u64, p0: (f64,f64,f64), p1: (f64,f64,f64), p2: (f64,f64,f64)) -> TopoDS_Face {
        let base = id * 100;
        let v0 = vtx(base, p0.0, p0.1, p0.2);
        let v1 = vtx(base+1, p1.0, p1.1, p1.2);
        let v2 = vtx(base+2, p2.0, p2.1, p2.2);
        let e0 = edge_between(base+10, v0.clone(), v1.clone());
        let e1 = edge_between(base+11, v1.clone(), v2.clone());
        let e2 = edge_between(base+12, v2.clone(), v0.clone());
        let wire = TopoDS_Wire { id: base+20, edges: vec![e0, e1, e2] };
        TopoDS_Face { id, wires: vec![wire] }
    }

    /// Build a rough unit-cube solid (12 triangles, two per rectangular face).
    fn unit_cube_solid() -> TopoDS_Solid {
        // Six faces of a unit cube centred at (0.5, 0.5, 0.5):
        //   bottom z=0, top z=1, front y=0, back y=1, left x=0, right x=1
        // Each quad is split into two triangles.
        let mut faces = Vec::new();
        let mut id = 0u64;

        // bottom z=0: (0,0,0),(1,0,0),(1,1,0),(0,1,0)
        faces.push(tri_face(id, (0.,0.,0.), (1.,0.,0.), (1.,1.,0.))); id+=1;
        faces.push(tri_face(id, (0.,0.,0.), (1.,1.,0.), (0.,1.,0.))); id+=1;
        // top z=1
        faces.push(tri_face(id, (0.,0.,1.), (1.,1.,1.), (1.,0.,1.))); id+=1;
        faces.push(tri_face(id, (0.,0.,1.), (0.,1.,1.), (1.,1.,1.))); id+=1;
        // front y=0
        faces.push(tri_face(id, (0.,0.,0.), (1.,0.,1.), (1.,0.,0.))); id+=1;
        faces.push(tri_face(id, (0.,0.,0.), (0.,0.,1.), (1.,0.,1.))); id+=1;
        // back y=1
        faces.push(tri_face(id, (0.,1.,0.), (1.,1.,0.), (1.,1.,1.))); id+=1;
        faces.push(tri_face(id, (0.,1.,0.), (1.,1.,1.), (0.,1.,1.))); id+=1;
        // left x=0
        faces.push(tri_face(id, (0.,0.,0.), (0.,1.,0.), (0.,1.,1.))); id+=1;
        faces.push(tri_face(id, (0.,0.,0.), (0.,1.,1.), (0.,0.,1.))); id+=1;
        // right x=1
        faces.push(tri_face(id, (1.,0.,0.), (1.,1.,1.), (1.,1.,0.))); id+=1;
        faces.push(tri_face(id, (1.,0.,0.), (1.,0.,1.), (1.,1.,1.)));

        let shell = TopoDS_Shell { id: 999, faces };
        TopoDS_Solid { id: 1000, shells: vec![shell] }
    }

    // ── tests ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_new_classifier_is_not_done() {
        let clf = BRepClass3d_SolidClassifier::new();
        assert!(!clf.is_done());
        assert_eq!(clf.state(), State::UNKNOWN);
    }

    #[test]
    fn test_load_empty_solid_classifies_out() {
        let shape = TopoDS_Shape::Solid(TopoDS_Solid { id: 1, shells: vec![] });
        let mut clf = BRepClass3d_SolidClassifier::new();
        clf.load(&shape);
        clf.perform(Pnt::new(0.5, 0.5, 0.5), 1e-7);
        assert_eq!(clf.state(), State::OUT);
    }

    #[test]
    fn test_point_inside_unit_cube() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let mut clf = BRepClass3d_SolidClassifier::new();
        clf.load(&shape);
        clf.perform(Pnt::new(0.5, 0.5, 0.5), 1e-7);
        assert_eq!(clf.state(), State::IN, "centre should be inside");
    }

    #[test]
    fn test_point_outside_unit_cube() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let mut clf = BRepClass3d_SolidClassifier::new();
        clf.load(&shape);
        clf.perform(Pnt::new(5.0, 5.0, 5.0), 1e-7);
        assert_eq!(clf.state(), State::OUT, "far point should be outside");
    }

    #[test]
    fn test_is_done_after_perform() {
        let shape = TopoDS_Shape::Solid(TopoDS_Solid { id: 2, shells: vec![] });
        let mut clf = BRepClass3d_SolidClassifier::new();
        clf.load(&shape);
        clf.perform(Pnt::origin(), 1e-7);
        assert!(clf.is_done());
    }

    #[test]
    fn test_explorer_nb_faces() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let exp = BRepClass3d_SolidExplorer::new(&shape);
        // 12 triangular faces
        assert_eq!(exp.nb_faces(), 12);
    }

    #[test]
    fn test_explorer_iteration() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let mut exp = BRepClass3d_SolidExplorer::new(&shape);
        let mut count = 0;
        while exp.more() {
            let _face = exp.current_face();
            count += 1;
            exp.next();
        }
        assert_eq!(count, 12);
    }

    #[test]
    fn test_explorer_init_resets_iterator() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let mut exp = BRepClass3d_SolidExplorer::new(&shape);
        exp.next();
        exp.next();
        exp.init();
        assert!(exp.more());
    }

    #[test]
    fn test_ray_triangle_intersect_basic() {
        // Ray along +Z from below the triangle in the XY-plane.
        let origin = Pnt::new(0.1, 0.1, -1.0);
        let dir    = Pnt::new(0.0, 0.0,  1.0);
        let v0 = Pnt::new(0.0, 0.0, 0.0);
        let v1 = Pnt::new(1.0, 0.0, 0.0);
        let v2 = Pnt::new(0.0, 1.0, 0.0);
        let t = ray_triangle_intersect(origin, dir, v0, v1, v2, 1e-9);
        assert!(t.is_some());
        let t = t.unwrap();
        assert!((t - 1.0).abs() < 1e-9, "expected t≈1, got {t}");
    }

    #[test]
    fn test_ray_triangle_no_intersect_parallel() {
        let origin = Pnt::new(0.0, 0.0, 1.0);
        let dir    = Pnt::new(1.0, 0.0, 0.0); // parallel to XY-plane
        let v0 = Pnt::new(0.0, 0.0, 0.0);
        let v1 = Pnt::new(1.0, 0.0, 0.0);
        let v2 = Pnt::new(0.0, 1.0, 0.0);
        let t = ray_triangle_intersect(origin, dir, v0, v1, v2, 1e-9);
        assert!(t.is_none());
    }

    #[test]
    fn test_classify_free_function_inside() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let s = classify(&shape, Pnt::new(0.5, 0.5, 0.5), 1e-7);
        assert_eq!(s, State::IN);
    }

    #[test]
    fn test_load_and_perform_convenience() {
        let solid = unit_cube_solid();
        let shape = TopoDS_Shape::Solid(solid);
        let mut clf = BRepClass3d_SolidClassifier::new();
        let s = clf.load_and_perform(&shape, Pnt::new(0.5, 0.5, 0.5), 1e-7);
        assert_eq!(s, State::IN);
    }
}
