//! Analytic B-rep STEP writer.
//!
//! `mesh_io::write_step` emits every triangle as a tiny planar face
//! (`FACETED_BREP`) — geometrically faithful but semantically dead: a
//! half-cylinder arrives in a CAD tool as hundreds of planar slivers.
//!
//! This module instead reconstructs *analytic* faces from the tessellation
//! using the [`Solid`]'s surface provenance (`Solid::hints`, stamped by the
//! primitive builders and carried through booleans):
//!
//! 1. every triangle whose vertices lie on a hinted curved surface (cylinder,
//!    sphere, torus) is claimed by that surface;
//! 2. the remaining triangles are clustered into maximal coplanar regions;
//! 3. each region's boundary polylines are extracted and segmented into exact
//!    `LINE` / `CIRCLE` edges by line- and circle-fitting (falling back to a
//!    chain of short lines where nothing fits);
//! 4. faces are emitted as `ADVANCED_FACE` over `CYLINDRICAL_SURFACE` /
//!    `SPHERICAL_SURFACE` / `TOROIDAL_SURFACE` / `PLANE`, gathered into a
//!    `MANIFOLD_SOLID_BREP`.
//!
//! A half-cylinder therefore exports as *one* cylindrical face bounded by two
//! circular arcs and two straight lines — a real boundary representation —
//! while anything without provenance degrades gracefully to planar/faceted
//! faces in the same file.

use crate::geom::Surface;
use crate::gp::Pnt;
use crate::mesh_io::StepEmitter;
use crate::topods::Solid;
use std::collections::HashMap;

/// On-surface tolerance, relative to the surface radius. BSP splits introduce
/// vertices on triangle *chords*, which sag below the true surface by
/// `r * (1 - cos(pi/segs))` — about 1.2e-3·r at 64 segments — so this must sit
/// comfortably above that while staying far below feature scale.
const REL_ON_SURF: f64 = 4e-3;
/// Absolute tolerance floor (model units).
const ABS_TOL: f64 = 1e-7;
/// Plane clustering: quantum for the unit normal and the plane offset.
const PLANE_QUANT: f64 = 1e-5;
/// Line fit: max deviation of interior points from the end-to-end segment,
/// relative to segment length.
const REL_LINE_TOL: f64 = 1e-3;
/// Circle fit: max radial residual relative to the fitted radius.
const REL_CIRCLE_TOL: f64 = 8e-3;

// ---------------------------------------------------------------------------
// classification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum FaceSurf {
    /// Index into the hint list.
    Hint(usize),
    /// Detected plane: unit normal + signed offset (n·p = d).
    Plane { normal: Pnt, d: f64 },
}

struct FaceGroup {
    surf: FaceSurf,
    tris: Vec<usize>,
}

fn surf_distance(s: &Surface, p: Pnt) -> Option<f64> {
    match s {
        Surface::Cylinder { placement, radius } => {
            let rel = p - placement.location;
            let along = rel.dot(placement.z_dir);
            let radial = rel - placement.z_dir * along;
            Some((radial.norm() - radius).abs())
        }
        Surface::Sphere { placement, radius } => {
            Some(((p - placement.location).norm() - radius).abs())
        }
        Surface::Torus {
            placement,
            major,
            minor,
        } => {
            let rel = p - placement.location;
            let along = rel.dot(placement.z_dir);
            let radial = (rel - placement.z_dir * along).norm();
            Some((((radial - major).powi(2) + along * along).sqrt() - minor).abs())
        }
        _ => None,
    }
}

fn surf_scale(s: &Surface) -> f64 {
    match s {
        Surface::Cylinder { radius, .. } | Surface::Sphere { radius, .. } => *radius,
        Surface::Torus { minor, .. } => *minor,
        _ => 1.0,
    }
}

/// Analytic outward normal of `s` at a point assumed on the surface.
fn surf_normal(s: &Surface, p: Pnt) -> Option<Pnt> {
    match s {
        Surface::Cylinder { placement, .. } => {
            let rel = p - placement.location;
            let along = rel.dot(placement.z_dir);
            Some((rel - placement.z_dir * along).normalized())
        }
        Surface::Sphere { placement, .. } => Some((p - placement.location).normalized()),
        Surface::Torus {
            placement, major, ..
        } => {
            let rel = p - placement.location;
            let along = rel.dot(placement.z_dir);
            let radial = rel - placement.z_dir * along;
            let ring = placement.location + radial.normalized() * *major;
            Some((p - ring).normalized())
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// boundary loops
// ---------------------------------------------------------------------------

/// Directed boundary edges of a triangle group -> closed vertex loops.
fn boundary_loops(tris: &[[usize; 3]], group: &[usize]) -> Vec<Vec<usize>> {
    use std::collections::HashSet;
    let mut dir: HashSet<(usize, usize)> = HashSet::new();
    for &t in group {
        let [a, b, c] = tris[t];
        for (u, v) in [(a, b), (b, c), (c, a)] {
            if !dir.remove(&(v, u)) {
                dir.insert((u, v));
            }
        }
    }
    let mut next: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(u, v) in &dir {
        next.entry(u).or_default().push(v);
    }
    let mut loops = Vec::new();
    let mut remaining: HashSet<(usize, usize)> = dir;
    while let Some(&(start, first)) = remaining.iter().next() {
        let mut lp = vec![start];
        let mut cur = first;
        remaining.remove(&(start, first));
        while cur != start {
            lp.push(cur);
            let outs = match next.get(&cur) {
                Some(o) => o,
                None => break, // open chain: give up on this loop
            };
            let Some(&nxt) = outs.iter().find(|&&v| remaining.contains(&(cur, v))) else {
                break;
            };
            remaining.remove(&(cur, nxt));
            cur = nxt;
        }
        if lp.len() >= 3 && cur == start {
            loops.push(lp);
        }
    }
    loops
}

// ---------------------------------------------------------------------------
// edge fitting
// ---------------------------------------------------------------------------

enum FittedEdge {
    /// Straight segment between two loop points.
    Line { a: usize, b: usize },
    /// Circular arc from `a` to `b` through the chain's interior points.
    Arc {
        a: usize,
        b: usize,
        center: Pnt,
        axis: Pnt,
        radius: f64,
    },
    /// Whole loop is one circle (no corners at all).
    FullCircle { at: usize, center: Pnt, axis: Pnt, radius: f64 },
}

fn max_line_dev(pts: &[Pnt]) -> f64 {
    let (a, b) = (pts[0], pts[pts.len() - 1]);
    let ab = b - a;
    let len = ab.norm();
    if len < ABS_TOL {
        return f64::INFINITY;
    }
    let d = ab * (1.0 / len);
    pts[1..pts.len() - 1]
        .iter()
        .map(|&p| {
            let rel = p - a;
            (rel - d * rel.dot(d)).norm()
        })
        .fold(0.0, f64::max)
}

/// Least-squares circle through `pts` (Kåsa fit in the best plane).
/// Returns (center, unit axis, radius, max residual).
fn fit_circle(pts: &[Pnt]) -> Option<(Pnt, Pnt, f64, f64)> {
    if pts.len() < 3 {
        return None;
    }
    let centroid = pts.iter().fold(Pnt::origin(), |s, &p| s + p) * (1.0 / pts.len() as f64);
    // Newell normal of the point fan (works for arcs too).
    let mut n = Pnt::origin();
    for i in 0..pts.len() - 1 {
        let (p, q) = (pts[i] - centroid, pts[i + 1] - centroid);
        n = n + p.cross(q);
    }
    if n.norm() < ABS_TOL {
        return None;
    }
    let n = n.normalized();
    // In-plane basis.
    let u = (pts[0] - centroid - n * (pts[0] - centroid).dot(n)).normalized();
    let v = n.cross(u);
    // Kåsa: minimize |(x,y) - c|^2 - r^2 linearly.
    let (mut sx, mut sy, mut sxx, mut syy, mut sxy, mut sxz, mut syz, mut sz) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for &p in pts {
        let rel = p - centroid;
        let (x, y) = (rel.dot(u), rel.dot(v));
        let z = x * x + y * y;
        sx += x;
        sy += y;
        sxx += x * x;
        syy += y * y;
        sxy += x * y;
        sxz += x * z;
        syz += y * z;
        sz += z;
    }
    let m = pts.len() as f64;
    let (a11, a12, a21, a22) = (2.0 * (sxx - sx * sx / m), 2.0 * (sxy - sx * sy / m),
                                 2.0 * (sxy - sx * sy / m), 2.0 * (syy - sy * sy / m));
    let (b1, b2) = (sxz - sx * sz / m, syz - sy * sz / m);
    let det = a11 * a22 - a12 * a21;
    if det.abs() < 1e-12 {
        return None;
    }
    let cx = (b1 * a22 - b2 * a12) / det;
    let cy = (a11 * b2 - a21 * b1) / det;
    let center = centroid + u * cx + v * cy;
    let radius = (pts.iter().map(|&p| (p - center).norm()).sum::<f64>()) / m;
    let resid = pts
        .iter()
        .map(|&p| ((p - center).norm() - radius).abs())
        .fold(0.0, f64::max);
    // Planarity residual too.
    let planar = pts
        .iter()
        .map(|&p| (p - centroid).dot(n).abs())
        .fold(0.0, f64::max);
    Some((center, n, radius, resid.max(planar)))
}

/// Fit a single open chain of loop positions `[i0..=i1]` (indices into the
/// rotated loop) as one LINE, one ARC, or a fallback polyline of lines.
fn fit_chain(chain: &[usize], verts: &[Pnt]) -> Vec<FittedEdge> {
    let pts: Vec<Pnt> = chain.iter().map(|&vi| verts[vi]).collect();
    let (a, b) = (chain[0], chain[chain.len() - 1]);
    if chain.len() == 2 {
        return vec![FittedEdge::Line { a, b }];
    }
    let span = (pts[pts.len() - 1] - pts[0]).norm();
    if max_line_dev(&pts) <= REL_LINE_TOL * span.max(ABS_TOL) {
        return vec![FittedEdge::Line { a, b }];
    }
    if let Some((c, ax, r, resid)) = fit_circle(&pts) {
        if resid <= REL_CIRCLE_TOL * r {
            let (p0, p1) = (pts[0], pts[1]);
            let swept = (p0 - c).cross(p1 - c);
            let axis = if swept.dot(ax) >= 0.0 { ax } else { ax * -1.0 };
            return vec![FittedEdge::Arc {
                a,
                b,
                center: c,
                axis,
                radius: r,
            }];
        }
    }
    // fallback: one line per segment
    chain
        .windows(2)
        .map(|w| FittedEdge::Line { a: w[0], b: w[1] })
        .collect()
}

/// Segment one closed vertex loop into exact edges: detect the C0 corners,
/// then fit each corner-to-corner chain as a line or circular arc.
fn fit_loop(loop_verts: &[usize], verts: &[Pnt]) -> Vec<FittedEdge> {
    let n = loop_verts.len();
    let pt = |i: usize| verts[loop_verts[i % n]];

    // Corners: direction change above ~25 degrees (tessellation chords of a
    // 64-segment circle bend ~5.6 degrees, real corners are far sharper).
    const CORNER_COS: f64 = 0.90;
    let mut corners: Vec<usize> = (0..n)
        .filter(|&i| {
            let prev = (pt(i) - pt(i + n - 1)).normalized();
            let next = (pt(i + 1) - pt(i)).normalized();
            prev.dot(next) < CORNER_COS
        })
        .collect();

    if corners.is_empty() {
        // Smooth loop: one full circle if it fits, else polyline fallback.
        let all: Vec<Pnt> = (0..n).map(pt).collect();
        if let Some((c, ax, r, resid)) = fit_circle(&all) {
            if resid <= REL_CIRCLE_TOL * r {
                let (p0, p1) = (all[0], all[1]);
                let swept = (p0 - c).cross(p1 - c);
                let axis = if swept.dot(ax) >= 0.0 { ax } else { ax * -1.0 };
                return vec![FittedEdge::FullCircle {
                    at: loop_verts[0],
                    center: c,
                    axis,
                    radius: r,
                }];
            }
        }
        corners = vec![0]; // treat position 0 as an artificial corner
    }

    // Chains between consecutive corners (wrapping once around the loop).
    let mut edges = Vec::new();
    let k = corners.len();
    for ci in 0..k {
        let start = corners[ci];
        let end = corners[(ci + 1) % k];
        let len = if ci + 1 < k { end - start } else { n - start + end };
        if len == 0 {
            continue;
        }
        let chain: Vec<usize> = (0..=len).map(|o| loop_verts[(start + o) % n]).collect();
        edges.extend(fit_chain(&chain, verts));
    }
    edges
}

// ---------------------------------------------------------------------------
// emission
// ---------------------------------------------------------------------------

fn fmt_pnt(p: Pnt) -> String {
    format!("({:.9},{:.9},{:.9})", p.x, p.y, p.z)
}

struct Emit<'a> {
    e: &'a mut StepEmitter,
    vertex_ids: HashMap<usize, usize>,
}

impl<'a> Emit<'a> {
    fn vertex(&mut self, vi: usize, p: Pnt) -> usize {
        if let Some(&id) = self.vertex_ids.get(&vi) {
            return id;
        }
        let cp = self.e.add(&format!("CARTESIAN_POINT('',{})", fmt_pnt(p)));
        let vp = self.e.add(&format!("VERTEX_POINT('',#{cp})"));
        self.vertex_ids.insert(vi, vp);
        vp
    }

    fn axis2(&mut self, origin: Pnt, z: Pnt, x: Pnt) -> usize {
        let o = self.e.add(&format!("CARTESIAN_POINT('',{})", fmt_pnt(origin)));
        let zd = self
            .e
            .add(&format!("DIRECTION('',({:.9},{:.9},{:.9}))", z.x, z.y, z.z));
        let xd = self
            .e
            .add(&format!("DIRECTION('',({:.9},{:.9},{:.9}))", x.x, x.y, x.z));
        self.e
            .add(&format!("AXIS2_PLACEMENT_3D('',#{o},#{zd},#{xd})"))
    }

    fn edge(&mut self, fe: &FittedEdge, verts: &[Pnt]) -> usize {
        match *fe {
            FittedEdge::Line { a, b } => {
                let (pa, pb) = (verts[a], verts[b]);
                let va = self.vertex(a, pa);
                let vb = self.vertex(b, pb);
                let dir = (pb - pa).normalized();
                let cp = self.e.add(&format!("CARTESIAN_POINT('',{})", fmt_pnt(pa)));
                let d = self.e.add(&format!(
                    "DIRECTION('',({:.9},{:.9},{:.9}))",
                    dir.x, dir.y, dir.z
                ));
                let vec = self.e.add(&format!("VECTOR('',#{d},1.)"));
                let line = self.e.add(&format!("LINE('',#{cp},#{vec})"));
                self.e
                    .add(&format!("EDGE_CURVE('',#{va},#{vb},#{line},.T.)"))
            }
            FittedEdge::Arc {
                a,
                b,
                center,
                axis,
                radius,
            } => {
                let (pa, pb) = (verts[a], verts[b]);
                let va = self.vertex(a, pa);
                let vb = self.vertex(b, pb);
                let refd = (pa - center).normalized();
                let a2 = self.axis2(center, axis, refd);
                let circ = self.e.add(&format!("CIRCLE('',#{a2},{radius:.9})"));
                self.e
                    .add(&format!("EDGE_CURVE('',#{va},#{vb},#{circ},.T.)"))
            }
            FittedEdge::FullCircle {
                at,
                center,
                axis,
                radius,
            } => {
                let p = verts[at];
                let v = self.vertex(at, p);
                let refd = (p - center).normalized();
                let a2 = self.axis2(center, axis, refd);
                let circ = self.e.add(&format!("CIRCLE('',#{a2},{radius:.9})"));
                self.e.add(&format!("EDGE_CURVE('',#{v},#{v},#{circ},.T.)"))
            }
        }
    }
}

/// Signed area magnitude of a 3D polygon (Newell) — used to pick outer loops.
fn loop_area(loop_verts: &[usize], verts: &[Pnt]) -> f64 {
    let mut n = Pnt::origin();
    let m = loop_verts.len();
    for i in 0..m {
        let p = verts[loop_verts[i]];
        let q = verts[loop_verts[(i + 1) % m]];
        n = n + p.cross(q);
    }
    0.5 * n.norm()
}

/// Serialize a solid to STEP with analytic faces where provenance allows.
// occt: STEPControl_Writer
pub fn write_step_analytic(solid: &Solid, name: &str) -> String {
    let m = solid.mesh().welded(1e-7);
    let hints = solid.hints();

    // -- classify ----------------------------------------------------------
    let mut owner: Vec<Option<usize>> = vec![None; m.tris.len()]; // group index
    let mut groups: Vec<FaceGroup> = Vec::new();

    for (hi, h) in hints.iter().enumerate() {
        if surf_distance(h, Pnt::origin()).is_none() && !matches!(h, Surface::Cylinder { .. }) {
            continue;
        }
        let tol = REL_ON_SURF * surf_scale(h) + ABS_TOL;
        let mut claimed = Vec::new();
        for (ti, t) in m.tris.iter().enumerate() {
            if owner[ti].is_some() {
                continue;
            }
            let on = t.iter().all(|&vi| {
                surf_distance(h, m.verts[vi]).map(|d| d <= tol).unwrap_or(false)
            });
            if !on {
                continue;
            }
            // reject chords spanning the surface (e.g. caps of a sphere):
            // the triangle centroid must be on-surface too, and the geometric
            // normal must roughly agree with the analytic one.
            let (a, b, c) = (m.verts[t[0]], m.verts[t[1]], m.verts[t[2]]);
            let centroid = (a + b + c) * (1.0 / 3.0);
            if surf_distance(h, centroid).map(|d| d > 3.0 * tol).unwrap_or(true) {
                continue;
            }
            let gn = (b - a).cross(c - a).normalized();
            if let Some(an) = surf_normal(h, centroid) {
                if gn.dot(an).abs() < 0.7 {
                    continue;
                }
            }
            claimed.push(ti);
        }
        if !claimed.is_empty() {
            for &ti in &claimed {
                owner[ti] = Some(groups.len());
            }
            groups.push(FaceGroup {
                surf: FaceSurf::Hint(hi),
                tris: claimed,
            });
        }
    }

    // planar clustering of the remainder
    let mut plane_key: HashMap<(i64, i64, i64, i64), usize> = HashMap::new();
    for (ti, t) in m.tris.iter().enumerate() {
        if owner[ti].is_some() {
            continue;
        }
        let (a, b, c) = (m.verts[t[0]], m.verts[t[1]], m.verts[t[2]]);
        let n = (b - a).cross(c - a);
        if n.norm() < ABS_TOL {
            continue; // degenerate sliver
        }
        let n = n.normalized();
        let d = n.dot(a);
        let q = |x: f64| (x / PLANE_QUANT).round() as i64;
        let key = (q(n.x), q(n.y), q(n.z), q(d));
        let gi = *plane_key.entry(key).or_insert_with(|| {
            groups.push(FaceGroup {
                surf: FaceSurf::Plane { normal: n, d },
                tris: Vec::new(),
            });
            groups.len() - 1
        });
        owner[ti] = Some(gi);
        groups[gi].tris.push(ti);
    }

    // -- emit ---------------------------------------------------------------
    let mut e = StepEmitter::new();
    let app_ctx = e.add("APPLICATION_CONTEXT('automotive design')");
    let _proto = e.add(&format!(
        "APPLICATION_PROTOCOL_DEFINITION('international standard','config_control_design',2010,#{app_ctx})"
    ));
    let dim = e.add("(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.))");
    let angle = e.add("(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.))");
    let solid_angle = e.add("(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT())");
    let uncert = e.add(&format!(
        "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-06),#{dim},'distance','')"
    ));
    let ctx = e.add(&format!(
        "(GEOMETRIC_REPRESENTATION_CONTEXT(3)GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncert}))GLOBAL_UNIT_ASSIGNED_CONTEXT((#{dim},#{angle},#{solid_angle}))REPRESENTATION_CONTEXT('Context','3D'))"
    ));

    let mut emit = Emit {
        e: &mut e,
        vertex_ids: HashMap::new(),
    };
    let mut face_ids = Vec::new();

    for g in &groups {
        if g.tris.is_empty() {
            continue;
        }
        let loops = boundary_loops(&m.tris, &g.tris);
        if loops.is_empty() {
            // closed surface with no boundary (full sphere/torus/cylinder
            // barrel fused shut): STEP still wants a bound — fall back to
            // faceted for this group.
            for &ti in &g.tris {
                face_ids.push(emit_faceted_triangle(&mut emit, &m.verts, m.tris[ti]));
            }
            continue;
        }

        // fit boundary edges
        let mut bound_ids = Vec::new();
        // largest loop is the outer bound
        let mut order: Vec<usize> = (0..loops.len()).collect();
        order.sort_by(|&i, &j| {
            loop_area(&loops[j], &m.verts)
                .partial_cmp(&loop_area(&loops[i], &m.verts))
                .unwrap()
        });
        for (rank, &li) in order.iter().enumerate() {
            let fitted = fit_loop(&loops[li], &m.verts);
            let mut oriented = Vec::new();
            for fe in &fitted {
                let ec = emit.edge(fe, &m.verts);
                let oe = emit.e.add(&format!("ORIENTED_EDGE('',*,*,#{ec},.T.)"));
                oriented.push(format!("#{oe}"));
            }
            let el = emit
                .e
                .add(&format!("EDGE_LOOP('',({}))", oriented.join(",")));
            let kind = if rank == 0 {
                "FACE_OUTER_BOUND"
            } else {
                "FACE_BOUND"
            };
            bound_ids.push(format!("#{}", emit.e.add(&format!("{kind}('',#{el},.T.)"))));
        }

        // surface geometry + sense
        let t0 = m.tris[g.tris[0]];
        let (a, b, c) = (m.verts[t0[0]], m.verts[t0[1]], m.verts[t0[2]]);
        let centroid = (a + b + c) * (1.0 / 3.0);
        let gn = (b - a).cross(c - a).normalized();
        let (surf_id, same_sense) = match &g.surf {
            FaceSurf::Hint(hi) => {
                let h = &hints[*hi];
                let (placement, body) = match h {
                    Surface::Cylinder { placement, radius } => {
                        (placement, format!("CYLINDRICAL_SURFACE('',@,{radius:.9})"))
                    }
                    Surface::Sphere { placement, radius } => {
                        (placement, format!("SPHERICAL_SURFACE('',@,{radius:.9})"))
                    }
                    Surface::Torus {
                        placement,
                        major,
                        minor,
                    } => (
                        placement,
                        format!("TOROIDAL_SURFACE('',@,{major:.9},{minor:.9})"),
                    ),
                    _ => unreachable!("only curved hints are classified"),
                };
                let a2 = emit.axis2(placement.location, placement.z_dir, placement.x_dir);
                let body = body.replace('@', &format!("#{a2}"));
                let sid = emit.e.add(&body);
                let sense = surf_normal(h, centroid)
                    .map(|an| gn.dot(an) >= 0.0)
                    .unwrap_or(true);
                (sid, sense)
            }
            FaceSurf::Plane { normal, d } => {
                let origin = *normal * *d;
                // any in-plane x direction
                let xd = pick_perp(*normal);
                let a2 = emit.axis2(origin, *normal, xd);
                let sid = emit.e.add(&format!("PLANE('',#{a2})"));
                (sid, gn.dot(*normal) >= 0.0)
            }
        };
        let sense = if same_sense { ".T." } else { ".F." };
        let fid = emit.e.add(&format!(
            "ADVANCED_FACE('',({}),#{surf_id},{sense})",
            bound_ids.join(",")
        ));
        face_ids.push(fid);
    }

    let shell_refs = face_ids
        .iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join(",");
    let shell = e.add(&format!("CLOSED_SHELL('',({shell_refs}))"));
    let brep = e.add(&format!("MANIFOLD_SOLID_BREP('{name}',#{shell})"));
    let rep = e.add(&format!(
        "ADVANCED_BREP_SHAPE_REPRESENTATION('{name}',(#{brep}),#{ctx})"
    ));

    // Minimal product structure — STEP readers locate shapes by walking
    // PRODUCT_DEFINITION -> SHAPE_DEFINITION_REPRESENTATION; without this
    // skeleton the file parses but yields no roots.
    let prod_ctx = e.add(&format!("PRODUCT_CONTEXT('',#{app_ctx},'mechanical')"));
    let product = e.add(&format!("PRODUCT('{name}','{name}','',(#{prod_ctx}))"));
    let category = e.add(&format!(
        "PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#{product}))"
    ));
    let _ = category;
    let formation = e.add(&format!("PRODUCT_DEFINITION_FORMATION('','',#{product})"));
    let pd_ctx = e.add(&format!(
        "PRODUCT_DEFINITION_CONTEXT('part definition',#{app_ctx},'design')"
    ));
    let pd = e.add(&format!(
        "PRODUCT_DEFINITION('design','',#{formation},#{pd_ctx})"
    ));
    let pds = e.add(&format!("PRODUCT_DEFINITION_SHAPE('','',#{pd})"));
    let _sdr = e.add(&format!("SHAPE_DEFINITION_REPRESENTATION(#{pds},#{rep})"));
    e.finish(name)
}

fn pick_perp(n: Pnt) -> Pnt {
    let cand = if n.x.abs() <= n.y.abs() && n.x.abs() <= n.z.abs() {
        Pnt::new(1.0, 0.0, 0.0)
    } else if n.y.abs() <= n.z.abs() {
        Pnt::new(0.0, 1.0, 0.0)
    } else {
        Pnt::new(0.0, 0.0, 1.0)
    };
    n.cross(cand).normalized()
}

fn emit_faceted_triangle(emit: &mut Emit, verts: &[Pnt], t: [usize; 3]) -> usize {
    let (a, b, c) = (verts[t[0]], verts[t[1]], verts[t[2]]);
    let n = (b - a).cross(c - a).normalized();
    let mut oriented = Vec::new();
    for (u, v) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
        let fe = FittedEdge::Line { a: u, b: v };
        let ec = emit.edge(&fe, verts);
        let oe = emit.e.add(&format!("ORIENTED_EDGE('',*,*,#{ec},.T.)"));
        oriented.push(format!("#{oe}"));
    }
    let el = emit
        .e
        .add(&format!("EDGE_LOOP('',({}))", oriented.join(",")));
    let bound = emit.e.add(&format!("FACE_OUTER_BOUND('',#{el},.T.)"));
    let a2 = emit.axis2(a, n, pick_perp(n));
    let plane = emit.e.add(&format!("PLANE('',#{a2})"));
    emit.e
        .add(&format!("ADVANCED_FACE('',(#{bound}),#{plane},.T.)"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_algo_api::cut;
    use crate::brep_prim_api::{make_box, make_cylinder, MeshParams};

    #[test]
    fn half_cylinder_exports_analytic_cylinder_face() {
        let cyl = make_cylinder(10.0, 20.0, MeshParams::default());
        // cut away the x < 0 half
        let block = make_box(Pnt::new(-11.0, -11.0, -1.0), 11.0, 22.0, 22.0);
        let half = cut(&cyl, &block);
        assert!(half.volume() > 0.0);
        assert_eq!(half.hints().len(), 1, "hint survives the boolean");

        let step = write_step_analytic(&half, "half_cylinder");
        assert!(step.contains("CYLINDRICAL_SURFACE"), "analytic face emitted");
        assert!(step.contains("MANIFOLD_SOLID_BREP"));
        assert!(step.contains("CIRCLE"), "cap boundary fitted as arcs");
        // The cylindrical wall must be ONE face, not hundreds of slivers:
        let n_cyl = step.matches("CYLINDRICAL_SURFACE").count();
        assert_eq!(n_cyl, 1, "single cylindrical face");
    }

    #[test]
    fn full_cylinder_boundaries_are_circles() {
        let cyl = make_cylinder(5.0, 8.0, MeshParams::default());
        let step = write_step_analytic(&cyl, "cyl");
        assert!(step.contains("CYLINDRICAL_SURFACE"));
        // both rims fit as full circles
        assert!(step.matches("CIRCLE").count() >= 2);
    }

    #[test]
    fn plain_box_is_six_planes() {
        let b = make_box(Pnt::new(0.0, 0.0, 0.0), 4.0, 5.0, 6.0);
        let step = write_step_analytic(&b, "box");
        assert_eq!(step.matches("PLANE(").count(), 6);
        assert!(!step.contains("CYLINDRICAL_SURFACE"));
    }
}
