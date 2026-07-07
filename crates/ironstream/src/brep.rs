//! Exact boundary representation (B-rep) — the foundation for exact boolean
//! operations.
//!
//! The mesh-BSP boolean engine ([`crate::bsp`]) is fast and general but
//! *fragile*: its result depends on the partition-plane order for
//! coplanar/degenerate geometry, so thin features (a helical thread fused into
//! a bore) and stacked coplanar cuts can come out wrong, and the output is
//! always tessellated (a hole is hundreds of facets, never a cylinder).
//!
//! This module models a solid the way a real CAD kernel does: as a set of
//! **faces on analytic surfaces** ([`crate::geom::Surface`]), each trimmed by
//! loops of **edges on analytic curves** ([`crate::geom::Curve`]). On this
//! representation booleans can be computed *exactly* — surfaces are intersected
//! analytically, faces split along the true intersection curves, and the result
//! carries real cylinders/planes/cones straight through to STEP.
//!
//! # Roadmap
//! * **Stage 1 (here):** the data model, primitive builders (box, cylinder),
//!   tessellation, and exact volume via the divergence theorem.
//! * **Stage 2:** exact boolean where the intersections are planar (box cuts,
//!   sectioning) — fixes stacked-coplanar-cut fragility.
//! * **Stage 3+:** surface-surface intersection for the curved quadric pairs,
//!   general boolean, then wiring it in as the [`crate::topods::Solid`] boolean
//!   path with a mesh-BSP fallback for freeform geometry.
//!
//! Trimming loops are stored in surface **parameter space** `(u, v)` (the
//! "pcurve" of each edge), which is what makes both tessellation and the
//! boolean face-splitting tractable.

use crate::geom::{Curve, Surface};
use crate::gp::Pnt;
use crate::mesh::TriMesh;
use std::f64::consts::PI;

/// An edge: a segment of an analytic [`Curve`] between two parameters, carrying
/// the parameter-space (pcurve) endpoints on each face it bounds. In Stage 1
/// edges are implied by the face trim loops; this type is the Stage-2 sharing
/// unit and is defined here so the model is complete.
#[derive(Clone, Debug)]
pub struct BEdge {
    pub curve: Curve,
    pub t0: f64,
    pub t1: f64,
}

impl BEdge {
    pub fn start(&self) -> Pnt {
        self.curve.value(self.t0)
    }
    pub fn end(&self) -> Pnt {
        self.curve.value(self.t1)
    }
}

/// A trimming loop in a face's `(u, v)` parameter space. The first loop of a
/// face is its outer boundary (CCW in parameter space); any others are holes.
pub type UvLoop = Vec<(f64, f64)>;

/// A face: a patch of an analytic [`Surface`], trimmed by parameter-space loops.
///
/// `sense = true` means the surface's natural normal is the solid's *outward*
/// normal; `false` means it is flipped. Every geometric query that needs the
/// outward normal must consult `sense`.
#[derive(Clone, Debug)]
pub struct BFace {
    pub surface: Surface,
    /// Outer loop first (CCW in uv), then hole loops.
    pub loops: Vec<UvLoop>,
    pub sense: bool,
}

impl BFace {
    pub fn new(surface: Surface, outer: UvLoop, sense: bool) -> Self {
        BFace { surface, loops: vec![outer], sense }
    }

    /// Outward normal at `(u, v)` (natural normal, flipped by `sense`).
    pub fn outward_normal(&self, u: f64, v: f64) -> Pnt {
        let n = self.surface.normal(u, v);
        if self.sense {
            n
        } else {
            -n
        }
    }
}

/// A solid: a closed, oriented set of faces.
#[derive(Clone, Debug, Default)]
pub struct BSolid {
    pub faces: Vec<BFace>,
}

impl BSolid {
    pub fn new(faces: Vec<BFace>) -> Self {
        BSolid { faces }
    }

    /// Tessellate to a triangle mesh (for rendering / mesh interop).
    pub fn tessellate(&self, mp: &TessParams) -> TriMesh {
        let mut m = TriMesh::new();
        for f in &self.faces {
            tessellate_face(f, mp, &mut m);
        }
        m
    }

    /// Exact volume via the divergence theorem: `V = (1/3) ∮ (r · n) dA`.
    ///
    /// Planar faces contribute a closed form (`offset · area / 3`); curved
    /// faces are integrated over their parameter rectangle with high-order
    /// Gauss–Legendre quadrature (effectively exact for the quadric surfaces).
    pub fn volume(&self) -> f64 {
        self.faces.iter().map(face_volume_contribution).sum()
    }
}

/// Tessellation resolution.
#[derive(Clone, Copy, Debug)]
pub struct TessParams {
    pub circle_segments: usize,
    pub axial_segments: usize,
}

impl Default for TessParams {
    fn default() -> Self {
        TessParams { circle_segments: 64, axial_segments: 1 }
    }
}

// ---------------------------------------------------------------------------
// primitive builders
// ---------------------------------------------------------------------------

/// A planar face from four 3D corners wound CCW about the intended outward
/// normal. The normal, in-plane frame and uv loop are all derived from the
/// corners, so the face covers exactly the quad given — no implicit frame
/// surprises.
fn planar_face_from_corners(c: [Pnt; 4]) -> BFace {
    use crate::gp::Ax3;
    let normal = (c[1] - c[0]).cross(c[2] - c[0]).normalized();
    let placement = Ax3::from_origin_normal(c[0], normal, c[1] - c[0]);
    let uv: UvLoop = c
        .iter()
        .map(|p| {
            let d = *p - c[0];
            (d.dot(placement.x_dir), d.dot(placement.y_dir))
        })
        .collect();
    BFace::new(Surface::Plane { placement }, uv, true)
}

/// Axis-aligned box `[corner, corner + (dx,dy,dz)]` as six planar faces. Each
/// face's corners are listed CCW about its outward normal, so the surface is a
/// correctly-oriented, watertight box.
pub fn box_brep(corner: Pnt, dx: f64, dy: f64, dz: f64) -> BSolid {
    let (dx, dy, dz) = (dx.abs(), dy.abs(), dz.abs());
    let x0 = corner.x;
    let y0 = corner.y;
    let z0 = corner.z;
    let (x1, y1, z1) = (x0 + dx, y0 + dy, z0 + dz);
    let p = |x, y, z| Pnt::new(x, y, z);

    let faces = vec![
        // -Z bottom
        planar_face_from_corners([p(x0, y0, z0), p(x0, y1, z0), p(x1, y1, z0), p(x1, y0, z0)]),
        // +Z top
        planar_face_from_corners([p(x0, y0, z1), p(x1, y0, z1), p(x1, y1, z1), p(x0, y1, z1)]),
        // -Y front
        planar_face_from_corners([p(x0, y0, z0), p(x1, y0, z0), p(x1, y0, z1), p(x0, y0, z1)]),
        // +Y back
        planar_face_from_corners([p(x0, y1, z0), p(x0, y1, z1), p(x1, y1, z1), p(x1, y1, z0)]),
        // -X left
        planar_face_from_corners([p(x0, y0, z0), p(x0, y0, z1), p(x0, y1, z1), p(x0, y1, z0)]),
        // +X right
        planar_face_from_corners([p(x1, y0, z0), p(x1, y1, z0), p(x1, y1, z1), p(x1, y0, z1)]),
    ];
    BSolid::new(faces)
}

/// Cylinder of `radius` and `height` along +Z from the origin: one cylindrical
/// barrel face plus two planar cap faces.
pub fn cylinder_brep(radius: f64, height: f64) -> BSolid {
    use crate::gp::Ax3;
    let axis = Ax3::from_origin_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));

    // Barrel: surface parameters u = angle [0, 2π], v = height [0, h].
    let barrel = BFace::new(
        Surface::Cylinder { placement: axis, radius },
        vec![(0.0, 0.0), (2.0 * PI, 0.0), (2.0 * PI, height), (0.0, height)],
        true,
    );

    // Caps are planar disks; their uv loop is a discretized circle in the
    // plane's (x, y). Resolution is refined at tessellation time for curved
    // edges, but the disk boundary is a circle so we store enough points.
    // A cap's uv loop is a discretized circle in its plane's local (x, y);
    // `ccw` orients it for the plane's own frame.
    let disk_loop = |ccw: bool| -> UvLoop {
        let n = 64;
        (0..n)
            .map(|i| {
                let a = 2.0 * PI * i as f64 / n as f64;
                let a = if ccw { a } else { -a };
                (radius * a.cos(), radius * a.sin())
            })
            .collect::<Vec<_>>()
    };
    let bottom = BFace::new(
        Surface::Plane {
            placement: Ax3::from_origin_normal(Pnt::origin(), Pnt::new(0.0, 0.0, -1.0), Pnt::new(1.0, 0.0, 0.0)),
        },
        disk_loop(false),
        true,
    );
    let top = BFace::new(
        Surface::Plane {
            placement: Ax3::from_origin_normal(Pnt::new(0.0, 0.0, height), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0)),
        },
        disk_loop(true),
        true,
    );

    BSolid::new(vec![barrel, bottom, top])
}

// ---------------------------------------------------------------------------
// tessellation
// ---------------------------------------------------------------------------

fn tessellate_face(f: &BFace, mp: &TessParams, out: &mut TriMesh) {
    match &f.surface {
        Surface::Plane { .. } => tessellate_planar(f, out),
        Surface::Cylinder { .. } | Surface::Cone { .. } => tessellate_ruled(f, mp, out),
        _ => tessellate_uv_grid(f, mp, out),
    }
}

/// Planar face: fan-triangulate its (convex, hole-free — Stage 1) uv loop and
/// map back to 3D. Winding follows `sense`.
fn tessellate_planar(f: &BFace, out: &mut TriMesh) {
    let loop_uv = &f.loops[0];
    if loop_uv.len() < 3 {
        return;
    }
    let p3d: Vec<Pnt> = loop_uv.iter().map(|&(u, v)| f.surface.value(u, v)).collect();
    let n = p3d.len();
    for i in 1..n - 1 {
        push_tri(out, p3d[0], p3d[i], p3d[i + 1], f.sense);
    }
}

/// Ruled quadric (cylinder/cone): the uv outer loop is `[0,2π] × [v0, v1]`, so
/// sweep a ring of `circle_segments` around and connect the two rails.
fn tessellate_ruled(f: &BFace, mp: &TessParams, out: &mut TriMesh) {
    let (u0, u1, v0, v1) = uv_bounds(&f.loops[0]);
    let segs = mp.circle_segments.max(8);
    let vs = mp.axial_segments.max(1);
    for j in 0..vs {
        let va = v0 + (v1 - v0) * j as f64 / vs as f64;
        let vb = v0 + (v1 - v0) * (j + 1) as f64 / vs as f64;
        for i in 0..segs {
            let ua = u0 + (u1 - u0) * i as f64 / segs as f64;
            let ub = u0 + (u1 - u0) * (i + 1) as f64 / segs as f64;
            let p00 = f.surface.value(ua, va);
            let p10 = f.surface.value(ub, va);
            let p11 = f.surface.value(ub, vb);
            let p01 = f.surface.value(ua, vb);
            push_tri(out, p00, p10, p11, f.sense);
            push_tri(out, p00, p11, p01, f.sense);
        }
    }
}

/// Generic curved face: grid the uv bounding box (Stage-1 fallback for
/// sphere/torus; boundary trimming is Stage-2 work).
fn tessellate_uv_grid(f: &BFace, mp: &TessParams, out: &mut TriMesh) {
    let (u0, u1, v0, v1) = uv_bounds(&f.loops[0]);
    let nu = mp.circle_segments.max(8);
    let nv = mp.circle_segments.max(8);
    for j in 0..nv {
        for i in 0..nu {
            let ua = u0 + (u1 - u0) * i as f64 / nu as f64;
            let ub = u0 + (u1 - u0) * (i + 1) as f64 / nu as f64;
            let va = v0 + (v1 - v0) * j as f64 / nv as f64;
            let vb = v0 + (v1 - v0) * (j + 1) as f64 / nv as f64;
            let p00 = f.surface.value(ua, va);
            let p10 = f.surface.value(ub, va);
            let p11 = f.surface.value(ub, vb);
            let p01 = f.surface.value(ua, vb);
            push_tri(out, p00, p10, p11, f.sense);
            push_tri(out, p00, p11, p01, f.sense);
        }
    }
}

fn uv_bounds(loop_uv: &UvLoop) -> (f64, f64, f64, f64) {
    let mut u0 = f64::INFINITY;
    let mut u1 = f64::NEG_INFINITY;
    let mut v0 = f64::INFINITY;
    let mut v1 = f64::NEG_INFINITY;
    for &(u, v) in loop_uv {
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    (u0, u1, v0, v1)
}

fn push_tri(out: &mut TriMesh, a: Pnt, b: Pnt, c: Pnt, sense: bool) {
    if sense {
        out.push_triangle(a, b, c);
    } else {
        out.push_triangle(a, c, b);
    }
}

// ---------------------------------------------------------------------------
// exact volume
// ---------------------------------------------------------------------------

/// One face's contribution to `(1/3) ∮ (r · n) dA`.
fn face_volume_contribution(f: &BFace) -> f64 {
    match &f.surface {
        Surface::Plane { placement } => {
            // r · n is the constant plane offset over the whole face.
            let n = if f.sense { placement.z_dir } else { -placement.z_dir };
            let p3d: Vec<Pnt> = f.loops[0].iter().map(|&(u, v)| f.surface.value(u, v)).collect();
            let (area, centroid) = polygon_area_centroid(&p3d, n);
            n.dot(centroid) * area / 3.0
        }
        _ => quadrature_volume(f),
    }
}

/// Area (>= 0) and centroid of a 3D planar polygon with outward normal `n`.
fn polygon_area_centroid(pts: &[Pnt], n: Pnt) -> (f64, Pnt) {
    let m = pts.len();
    if m < 3 {
        return (0.0, Pnt::origin());
    }
    let mut area2 = 0.0;
    let mut c = Pnt::origin();
    let p0 = pts[0];
    for i in 1..m - 1 {
        let cross = (pts[i] - p0).cross(pts[i + 1] - p0);
        let tri2 = cross.dot(n); // signed doubled area along n
        area2 += tri2;
        c = c + (p0 + pts[i] + pts[i + 1]) * (tri2 / 3.0);
    }
    let area = 0.5 * area2;
    if area.abs() < 1e-18 {
        (0.0, p0)
    } else {
        (area, c * (1.0 / area2))
    }
}

/// High-order Gauss–Legendre integration of `(value · outward_normal)` over the
/// face's uv rectangle for curved surfaces.
fn quadrature_volume(f: &BFace) -> f64 {
    // 5-point Gauss-Legendre nodes/weights on [-1, 1].
    const X: [f64; 5] = [
        -0.906179845938664,
        -0.538469310105683,
        0.0,
        0.538469310105683,
        0.906179845938664,
    ];
    const W: [f64; 5] = [
        0.236926885056189,
        0.478628670499366,
        0.568888888888889,
        0.478628670499366,
        0.236926885056189,
    ];
    let (u0, u1, v0, v1) = uv_bounds(&f.loops[0]);
    let (hu, hv) = ((u1 - u0) * 0.5, (v1 - v0) * 0.5);
    let (mu, mv) = ((u1 + u0) * 0.5, (v1 + v0) * 0.5);
    let eps = 1e-6;
    let mut acc = 0.0;
    for (i, &xi) in X.iter().enumerate() {
        let u = mu + hu * xi;
        for (j, &xj) in X.iter().enumerate() {
            let v = mv + hv * xj;
            let p = f.surface.value(u, v);
            let n = f.outward_normal(u, v);
            // area element |∂P/∂u × ∂P/∂v|
            let du = (f.surface.value(u + eps, v) - f.surface.value(u - eps, v)) * (1.0 / (2.0 * eps));
            let dv = (f.surface.value(u, v + eps) - f.surface.value(u, v - eps)) * (1.0 / (2.0 * eps));
            let da = du.cross(dv).norm();
            acc += W[i] * W[j] * p.dot(n) * da;
        }
    }
    acc * hu * hv / 3.0
}

// ---------------------------------------------------------------------------
// Stage 2: exact planar boolean via half-space clipping
// ---------------------------------------------------------------------------
//
// The building block of a planar boolean is clipping a solid by a plane —
// keeping the half behind a normal and capping the opening. Because every
// operation here is on planar faces, the intersection of two faces is a line
// and the clip is exact: no tessellation, and the result's volume is exact.
//
// `intersect_convex` composes clips: a convex solid B is the intersection of
// the half-spaces behind its faces, so `A ∩ B` is `A` clipped by each face of
// `B`. General (non-convex) booleans — where a cap can split into several loops
// and faces turn non-convex — are Stage 3, together with the curved surfaces.

/// A planar facet: outward unit normal `n` and a polygon wound CCW about `n`.
struct Facet {
    n: Pnt,
    poly: Vec<Pnt>,
}

const WELD: f64 = 1e-9;

impl BSolid {
    /// The planar faces as outward-oriented facets (non-planar faces are
    /// skipped — Stage 2 is planar-only).
    fn to_facets(&self) -> Vec<Facet> {
        self.faces
            .iter()
            .filter_map(|f| {
                let Surface::Plane { placement } = &f.surface else {
                    return None;
                };
                let n = if f.sense { placement.z_dir } else { -placement.z_dir };
                let mut poly: Vec<Pnt> =
                    f.loops[0].iter().map(|&(u, v)| f.surface.value(u, v)).collect();
                // The stored loop is CCW about the natural normal (z_dir); when
                // the outward normal is flipped, reverse so it is CCW about `n`.
                if !f.sense {
                    poly.reverse();
                }
                Some(Facet { n, poly })
            })
            .collect()
    }

    /// Clip to the half-space *behind* `n` (keep points with `(p - pt)·n ≤ 0`)
    /// and cap the opening. Assumes a convex cut (one cap loop) — the Stage-2
    /// scope. Faces fully outside drop; faces straddling the plane are trimmed.
    pub fn clip_by_plane(&self, pt: Pnt, n: Pnt) -> BSolid {
        let n = n.normalized();
        let s = |p: Pnt| (p - pt).dot(n);
        let mut facets: Vec<Facet> = Vec::new();
        let mut cut_pts: Vec<Pnt> = Vec::new();

        for f in self.to_facets() {
            let (clipped, cuts) = clip_polygon(&f.poly, &s);
            if clipped.len() >= 3 {
                facets.push(Facet { n: f.n, poly: clipped });
            }
            cut_pts.extend(cuts);
        }
        if let Some(cap) = build_convex_cap(&cut_pts, n) {
            facets.push(cap);
        }
        BSolid::new(facets.into_iter().map(facet_to_bface).collect())
    }
}

/// `A ∩ B` for convex `A`, `B`: clip `A` by every face-plane of `B`.
pub fn intersect_convex(a: &BSolid, b: &BSolid) -> BSolid {
    let mut result = a.clone();
    for f in b.to_facets() {
        // A point on the face and its outward normal define the half-space.
        let pt = f.poly[0];
        result = result.clip_by_plane(pt, f.n);
        if result.faces.is_empty() {
            break; // fully clipped away — no overlap
        }
    }
    result
}

/// Sutherland–Hodgman clip of `poly` to `{ p : s(p) ≤ 0 }`. Returns the clipped
/// polygon and the intersection points introduced on the cutting plane.
fn clip_polygon(poly: &[Pnt], s: &dyn Fn(Pnt) -> f64) -> (Vec<Pnt>, Vec<Pnt>) {
    let m = poly.len();
    let mut out: Vec<Pnt> = Vec::with_capacity(m + 2);
    let mut cuts: Vec<Pnt> = Vec::new();
    for i in 0..m {
        let a = poly[i];
        let b = poly[(i + 1) % m];
        let sa = s(a);
        let sb = s(b);
        let a_in = sa <= WELD;
        let b_in = sb <= WELD;
        if a_in {
            out.push(a);
        }
        if a_in != b_in {
            let t = sa / (sa - sb);
            let ip = a + (b - a) * t;
            out.push(ip);
            cuts.push(ip);
        }
    }
    (out, cuts)
}

/// Assemble the cap face closing a convex cut: weld the shared cut points,
/// order them CCW about `n`, and orient the facet outward (`+n`).
fn build_convex_cap(cut_pts: &[Pnt], n: Pnt) -> Option<Facet> {
    // Weld duplicates (each cap vertex arrives once per adjacent clipped face).
    let mut uniq: Vec<Pnt> = Vec::new();
    for &p in cut_pts {
        if !uniq.iter().any(|q| (*q - p).norm() < 1e-7) {
            uniq.push(p);
        }
    }
    if uniq.len() < 3 {
        return None;
    }
    let c = uniq.iter().fold(Pnt::origin(), |acc, &p| acc + p) * (1.0 / uniq.len() as f64);
    // In-plane frame for angular sort.
    let x = (uniq[0] - c).normalized();
    let y = n.cross(x).normalized();
    uniq.sort_by(|a, b| {
        let aa = (*a - c).dot(y).atan2((*a - c).dot(x));
        let ab = (*b - c).dot(y).atan2((*b - c).dot(x));
        aa.partial_cmp(&ab).unwrap()
    });
    Some(Facet { n, poly: uniq })
}

/// Rebuild a [`BFace`] from a facet: a plane with `z_dir = n`, its loop mapped
/// into that plane's `(u, v)`. The polygon is CCW about `n = z_dir`, so `sense`
/// is `true` (natural normal is outward).
fn facet_to_bface(f: Facet) -> BFace {
    use crate::gp::Ax3;
    let origin = f.poly[0];
    let x_hint = f.poly[1] - f.poly[0];
    let placement = Ax3::from_origin_normal(origin, f.n, x_hint);
    let uv: UvLoop = f
        .poly
        .iter()
        .map(|p| {
            let d = *p - origin;
            (d.dot(placement.x_dir), d.dot(placement.y_dir))
        })
        .collect();
    BFace::new(Surface::Plane { placement }, uv, true)
}

// ---------------------------------------------------------------------------
// Stage 3a: general planar boolean (polygon-level BSP)
// ---------------------------------------------------------------------------
//
// Union / subtract / intersect for arbitrary (non-convex) solids bounded by
// planar faces. The engine is the classic BSP clipping dance, but on *whole
// B-rep faces with their exact surface planes* — not triangle soup. That is
// what removes the mesh-BSP fragility: a box contributes 6 splitting planes
// (each taken verbatim from its `Surface::Plane`), not hundreds of near-
// coplanar triangle planes with re-derived normals, so coplanar faces of the
// two solids land on *identical* planes and are handled by the explicit
// coplanar branch instead of epsilon luck. Splitting a convex face by planes
// yields convex fragments, so fan tessellation stays valid throughout.

const BSP_EPS: f64 = 1e-9;

/// A polygon riding through the BSP: its exact plane and its CCW boundary.
#[derive(Clone)]
struct Poly {
    n: Pnt,
    d: f64, // n · p = d
    pts: Vec<Pnt>,
}

impl Poly {
    fn from_facet(f: &Facet) -> Self {
        let d = f.n.dot(f.poly[0]);
        Poly { n: f.n, d, pts: f.poly.clone() }
    }

    fn flip(&mut self) {
        self.n = -self.n;
        self.d = -self.d;
        self.pts.reverse();
    }
}

/// Split `p` by the plane `(n, d)` into the four csg classes.
fn split_poly(
    p: &Poly,
    n: Pnt,
    d: f64,
    co_front: &mut Vec<Poly>,
    co_back: &mut Vec<Poly>,
    front: &mut Vec<Poly>,
    back: &mut Vec<Poly>,
) {
    const COPLANAR: u8 = 0;
    const FRONT: u8 = 1;
    const BACK: u8 = 2;

    let mut poly_type = 0u8;
    let types: Vec<u8> = p
        .pts
        .iter()
        .map(|&v| {
            let t = n.dot(v) - d;
            let ty = if t < -BSP_EPS {
                BACK
            } else if t > BSP_EPS {
                FRONT
            } else {
                COPLANAR
            };
            poly_type |= ty;
            ty
        })
        .collect();

    match poly_type {
        0 => {
            // coplanar: facing decides which side of the dance owns it
            if n.dot(p.n) > 0.0 {
                co_front.push(p.clone());
            } else {
                co_back.push(p.clone());
            }
        }
        1 => front.push(p.clone()),
        2 => back.push(p.clone()),
        _ => {
            // spanning: walk the boundary, emitting to both sides
            let m = p.pts.len();
            let mut f_pts: Vec<Pnt> = Vec::with_capacity(m + 2);
            let mut b_pts: Vec<Pnt> = Vec::with_capacity(m + 2);
            for i in 0..m {
                let j = (i + 1) % m;
                let (ti, tj) = (types[i], types[j]);
                let (vi, vj) = (p.pts[i], p.pts[j]);
                if ti != BACK {
                    f_pts.push(vi);
                }
                if ti != FRONT {
                    b_pts.push(vi);
                }
                if (ti | tj) == (FRONT | BACK) {
                    let t = (d - n.dot(vi)) / n.dot(vj - vi);
                    let v = vi + (vj - vi) * t;
                    f_pts.push(v);
                    b_pts.push(v);
                }
            }
            if f_pts.len() >= 3 {
                front.push(Poly { n: p.n, d: p.d, pts: f_pts });
            }
            if b_pts.len() >= 3 {
                back.push(Poly { n: p.n, d: p.d, pts: b_pts });
            }
        }
    }
}

/// A BSP node over exact face planes.
#[derive(Default)]
struct BspNode {
    plane: Option<(Pnt, f64)>,
    front: Option<Box<BspNode>>,
    back: Option<Box<BspNode>>,
    polys: Vec<Poly>,
}

impl BspNode {
    fn from_polys(polys: Vec<Poly>) -> Self {
        let mut n = BspNode::default();
        n.build(polys);
        n
    }

    fn build(&mut self, polys: Vec<Poly>) {
        if polys.is_empty() {
            return;
        }
        let (pn, pd) = *self.plane.get_or_insert((polys[0].n, polys[0].d));
        let mut front = Vec::new();
        let mut back = Vec::new();
        let mut co_f = Vec::new();
        let mut co_b = Vec::new();
        for p in &polys {
            split_poly(p, pn, pd, &mut co_f, &mut co_b, &mut front, &mut back);
        }
        // coplanar polygons live at this node regardless of facing
        self.polys.extend(co_f);
        self.polys.extend(co_b);
        if !front.is_empty() {
            self.front.get_or_insert_with(Default::default).build(front);
        }
        if !back.is_empty() {
            self.back.get_or_insert_with(Default::default).build(back);
        }
    }

    fn invert(&mut self) {
        for p in &mut self.polys {
            p.flip();
        }
        if let Some((n, d)) = self.plane {
            self.plane = Some((-n, -d));
        }
        if let Some(f) = &mut self.front {
            f.invert();
        }
        if let Some(b) = &mut self.back {
            b.invert();
        }
        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Remove the parts of `polys` inside this BSP's solid.
    fn clip_polys(&self, polys: Vec<Poly>) -> Vec<Poly> {
        let Some((pn, pd)) = self.plane else {
            return polys;
        };
        let mut front = Vec::new();
        let mut back = Vec::new();
        let mut co_f = Vec::new();
        let mut co_b = Vec::new();
        for p in &polys {
            split_poly(p, pn, pd, &mut co_f, &mut co_b, &mut front, &mut back);
        }
        // coplanar-front goes with front, coplanar-back with back
        front.extend(co_f);
        back.extend(co_b);
        let mut front = match &self.front {
            Some(f) => f.clip_polys(front),
            None => front,
        };
        let back = match &self.back {
            Some(b) => b.clip_polys(back),
            None => Vec::new(), // no back subtree: inside the solid — dropped
        };
        front.extend(back);
        front
    }

    fn clip_to(&mut self, bsp: &BspNode) {
        self.polys = bsp.clip_polys(std::mem::take(&mut self.polys));
        if let Some(f) = &mut self.front {
            f.clip_to(bsp);
        }
        if let Some(b) = &mut self.back {
            b.clip_to(bsp);
        }
    }

    fn all_polys(&self, out: &mut Vec<Poly>) {
        out.extend(self.polys.iter().cloned());
        if let Some(f) = &self.front {
            f.all_polys(out);
        }
        if let Some(b) = &self.back {
            b.all_polys(out);
        }
    }
}

/// Boolean operation selector for [`boolean`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoolOp {
    Union,
    Subtract,
    Intersect,
}

/// General boolean between two planar-faced solids (either may be non-convex).
/// Curved faces are not yet carried through this path (Stage 3b).
///
/// The clipping dance is csg.js's, verbatim: the mutual `clip_to` passes trim
/// each solid's faces to the other's exterior (with an inversion pass that
/// removes coplanar duplicates), `b`'s survivors are folded into `a`'s tree,
/// and for subtract/intersect the final `invert` flips the whole result back
/// outward.
pub fn boolean(a: &BSolid, b: &BSolid, op: BoolOp) -> BSolid {
    let a_polys: Vec<Poly> = a.to_facets().iter().map(Poly::from_facet).collect();
    let b_polys: Vec<Poly> = b.to_facets().iter().map(Poly::from_facet).collect();
    let mut ta = BspNode::from_polys(a_polys);
    let mut tb = BspNode::from_polys(b_polys);

    match op {
        BoolOp::Union => {
            ta.clip_to(&tb);
            tb.clip_to(&ta);
            tb.invert();
            tb.clip_to(&ta);
            tb.invert();
            let mut bp = Vec::new();
            tb.all_polys(&mut bp);
            ta.build(bp);
        }
        BoolOp::Subtract => {
            ta.invert();
            ta.clip_to(&tb);
            tb.clip_to(&ta);
            tb.invert();
            tb.clip_to(&ta);
            tb.invert();
            let mut bp = Vec::new();
            tb.all_polys(&mut bp);
            ta.build(bp);
            ta.invert();
        }
        BoolOp::Intersect => {
            ta.invert();
            tb.clip_to(&ta);
            tb.invert();
            ta.clip_to(&tb);
            tb.clip_to(&ta);
            let mut bp = Vec::new();
            tb.all_polys(&mut bp);
            ta.build(bp);
            ta.invert();
        }
    }

    let mut merged = Vec::new();
    ta.all_polys(&mut merged);
    let faces = merged
        .into_iter()
        .filter(|p| p.pts.len() >= 3)
        .map(|p| facet_to_bface(Facet { n: p.n, poly: p.pts }))
        .collect();
    BSolid::new(faces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_brep_exact_volume() {
        let b = box_brep(Pnt::new(-2.0, -3.0, 1.0), 4.0, 5.0, 6.0);
        assert!((b.volume() - 120.0).abs() < 1e-9, "vol={}", b.volume());
        assert_eq!(b.faces.len(), 6);
    }

    #[test]
    fn box_brep_tessellates_watertight_volume() {
        let b = box_brep(Pnt::origin(), 2.0, 2.0, 2.0);
        let m = b.tessellate(&TessParams::default());
        assert!((m.volume() - 8.0).abs() < 1e-9, "mesh vol={}", m.volume());
    }

    #[test]
    fn cylinder_brep_volume() {
        let c = cylinder_brep(10.0, 20.0);
        let expect = PI * 100.0 * 20.0;
        assert_eq!(c.faces.len(), 3);
        // The barrel contribution is exact (analytic quadrature); the caps are
        // bounded by 64-point loops in Stage 1, so their area is a 64-gon —
        // ~5e-4 low. Exact circular areas arrive with analytic edges (Stage 2,
        // where the same edges become the BOP intersection curves).
        assert!(
            (c.volume() - expect).abs() / expect < 2e-3,
            "vol={} expect={}",
            c.volume(),
            expect
        );
    }

    #[test]
    fn cylinder_tessellation_volume_converges() {
        let c = cylinder_brep(5.0, 8.0);
        let m = c.tessellate(&TessParams { circle_segments: 256, axial_segments: 1 });
        let expect = PI * 25.0 * 8.0;
        // tessellated volume undershoots slightly; within 0.1% at 256 segments
        assert!((m.volume() - expect).abs() / expect < 1e-3, "mesh vol={}", m.volume());
    }

    // ---- Stage 2: exact planar boolean ----

    #[test]
    fn clip_box_by_axis_plane_halves_volume() {
        // [0,10]^3 clipped to z <= 5 is a 10x10x5 box.
        let b = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let half = b.clip_by_plane(Pnt::new(0.0, 0.0, 5.0), Pnt::new(0.0, 0.0, 1.0));
        assert!((half.volume() - 500.0).abs() < 1e-9, "vol={}", half.volume());
        // and it is watertight: the cap closes the opening exactly.
        let m = half.tessellate(&TessParams::default());
        assert!((m.volume() - 500.0).abs() < 1e-9, "mesh vol={}", m.volume());
    }

    #[test]
    fn clip_box_by_angled_plane_exact_wedge() {
        // Cut the unit cube by the plane x + z = 1 (normal (1,0,1)), keeping the
        // side x + z <= 1. The removed corner is a tetra-ish wedge; the kept
        // volume is 1 - 1/2*(1*1)/1... compute directly: region of unit cube with
        // x+z<=1 has volume 1 - 1/2 = 0.5 (prism split of the cube diagonally).
        let b = box_brep(Pnt::origin(), 1.0, 1.0, 1.0);
        let kept = b.clip_by_plane(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 1.0));
        assert!((kept.volume() - 0.5).abs() < 1e-9, "vol={}", kept.volume());
        let m = kept.tessellate(&TessParams::default());
        assert!((m.volume() - 0.5).abs() < 1e-9, "mesh vol={}", m.volume());
    }

    #[test]
    fn intersect_two_boxes_exact() {
        // [0,10]^3 ∩ [5,15]^3 = [5,10]^3, volume 125.
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0);
        let x = intersect_convex(&a, &b);
        assert!((x.volume() - 125.0).abs() < 1e-9, "vol={}", x.volume());
        let m = x.tessellate(&TessParams::default());
        assert!((m.volume() - 125.0).abs() < 1e-9, "mesh vol={}", m.volume());
    }

    #[test]
    fn intersect_disjoint_boxes_is_empty() {
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(20.0, 0.0, 0.0), 5.0, 5.0, 5.0);
        let x = intersect_convex(&a, &b);
        assert!(x.volume().abs() < 1e-9, "expected empty, vol={}", x.volume());
    }

    // ---- Stage 3a: general planar boolean ----

    /// analytic volume AND tessellated mesh volume must both match `expect`
    /// (the mesh check proves the result is watertight & correctly oriented).
    fn assert_solid(s: &BSolid, expect: f64, label: &str) {
        let v = s.volume();
        assert!(
            (v - expect).abs() < 1e-6,
            "{label}: analytic vol={v} expect={expect}"
        );
        let m = s.tessellate(&TessParams::default());
        assert!(
            (m.volume() - expect).abs() < 1e-6,
            "{label}: mesh vol={} expect={expect} (not watertight?)",
            m.volume()
        );
    }

    #[test]
    fn boolean_subtract_corner() {
        // [0,10]^3 minus [5,15]^3 removes the shared 5^3 corner: 1000 - 125.
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0);
        assert_solid(&boolean(&a, &b, BoolOp::Subtract), 875.0, "subtract corner");
    }

    #[test]
    fn boolean_union_overlap() {
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0);
        assert_solid(&boolean(&a, &b, BoolOp::Union), 1875.0, "union overlap");
    }

    #[test]
    fn boolean_intersect_matches_convex_path() {
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0);
        assert_solid(&boolean(&a, &b, BoolOp::Intersect), 125.0, "intersect");
    }

    #[test]
    fn boolean_through_hole_is_nonconvex() {
        // A square tunnel through the middle: the result is genuinely
        // non-convex — the case the convex clipper cannot do.
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(4.0, 4.0, -1.0), 2.0, 2.0, 12.0);
        assert_solid(&boolean(&a, &b, BoolOp::Subtract), 960.0, "through hole");
    }

    #[test]
    fn boolean_union_coplanar_faces() {
        // Two boxes sharing the z=5 plane: the classic coplanar case that
        // breaks epsilon-based mesh BSPs. Exact face planes make it exact.
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 5.0);
        let b = box_brep(Pnt::new(0.0, 0.0, 5.0), 5.0, 10.0, 5.0);
        assert_solid(&boolean(&a, &b, BoolOp::Union), 750.0, "L union");
    }

    #[test]
    fn boolean_subtract_disjoint_is_identity() {
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let b = box_brep(Pnt::new(30.0, 0.0, 0.0), 5.0, 5.0, 5.0);
        assert_solid(&boolean(&a, &b, BoolOp::Subtract), 1000.0, "disjoint cut");
    }

    #[test]
    fn boolean_stacked_coplanar_cuts() {
        // Two cuts whose walls are coplanar with each other AND with the box
        // face — the stacked-coplanar fragility case from the corpus. Each cut
        // removes a 2x10x2 notch from the top; the second notch shares the
        // x=2 wall plane with the first's x=2 wall.
        let a = box_brep(Pnt::origin(), 10.0, 10.0, 10.0);
        let c1 = box_brep(Pnt::new(0.0, 0.0, 8.0), 2.0, 10.0, 2.0);
        let c2 = box_brep(Pnt::new(2.0, 0.0, 8.0), 2.0, 10.0, 2.0);
        let cut1 = boolean(&a, &c1, BoolOp::Subtract);
        let cut2 = boolean(&cut1, &c2, BoolOp::Subtract);
        assert_solid(&cut2, 1000.0 - 40.0 - 40.0, "stacked coplanar cuts");
    }
}
