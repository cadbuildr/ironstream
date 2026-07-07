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

/// Axis-aligned box `[corner, corner + (dx,dy,dz)]` as six planar faces.
pub fn box_brep(corner: Pnt, dx: f64, dy: f64, dz: f64) -> BSolid {
    use crate::gp::Ax3;
    let (dx, dy, dz) = (dx.abs(), dy.abs(), dz.abs());
    let x0 = corner.x;
    let y0 = corner.y;
    let z0 = corner.z;

    // Each face: an origin, a plane placement (so uv maps to the face), and the
    // rectangle extents in uv. `sense` is true when the plane normal points out.
    let mut faces = Vec::with_capacity(6);
    let mut planar = |origin: Pnt, normal: Pnt, xdir: Pnt, w: f64, h: f64| {
        let placement = Ax3::from_origin_normal(origin, normal, xdir);
        let outer = vec![(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
        faces.push(BFace::new(Surface::Plane { placement }, outer, true));
    };
    // -Z (bottom): normal -z, at z0
    planar(Pnt::new(x0, y0, z0), Pnt::new(0.0, 0.0, -1.0), Pnt::new(0.0, 1.0, 0.0), dy, dx);
    // +Z (top): normal +z, at z0+dz
    planar(Pnt::new(x0, y0, z0 + dz), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0), dx, dy);
    // -Y: normal -y
    planar(Pnt::new(x0, y0, z0), Pnt::new(0.0, -1.0, 0.0), Pnt::new(0.0, 0.0, 1.0), dz, dx);
    // +Y: normal +y
    planar(Pnt::new(x0, y0 + dy, z0), Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 0.0, 0.0), dx, dz);
    // -X: normal -x
    planar(Pnt::new(x0, y0, z0), Pnt::new(-1.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0), dy, dz);
    // +X: normal +x
    planar(Pnt::new(x0 + dx, y0, z0), Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0), dz, dy);

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
}
