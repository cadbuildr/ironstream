//! Exact threads: a helical sweep folded into the revolved body (external
//! thread) or into the bore it sits in (internal thread).
//!
//! A thread is a planar section moved along a helix. In helical coordinates
//! (radius `r`, invariant `zeta = z - pitch * angle / 2pi`) the section is a
//! fixed polygon, so the threaded surface is a radius function on every
//! meridian: `max(body radius, ridge outer radius)` for an external thread,
//! `min(bore radius, ridge inner radius)` for an internal one. Each meridian
//! is a polyline with breakpoints at the section's vertices (one copy per
//! turn), the body's profile vertices and where the two radii cross, and
//! neighbouring meridians are stitched into a strip.
//!
//! That replaces a mesh boolean of a long helical sweep with a faceted
//! cylinder, which shattered both meshes into thousands of slivers, with a
//! clean surface of about OCCT's triangle count whose crest and root lines are
//! exact.

use ironstream::prelude::*;
use std::f64::consts::PI;

/// A helical sweep of a planar section (no twist).
#[derive(Clone, Debug)]
pub struct HelicalSweep {
    /// The section as placed at the start of the path (world).
    pub section: Vec<Pnt>,
    pub center: Pnt,
    /// Unit axis.
    pub axis: Pnt,
    /// +1 right-handed, -1 left-handed.
    pub sign: f64,
    pub pitch: f64,
    pub height: f64,
}

/// Cylindrical frame about an axis.
#[derive(Clone, Copy)]
pub struct AxisFrame {
    pub o: Pnt,
    pub u: Pnt,
    pub e1: Pnt,
    pub e2: Pnt,
}

impl AxisFrame {
    pub fn new(o: Pnt, u: Pnt) -> Self {
        let u = u.normalized();
        let e1 = u.any_perpendicular().normalized();
        let e2 = u.cross(e1);
        Self { o, u, e1, e2 }
    }
    /// (r, angle, z)
    fn cyl(&self, p: Pnt) -> (f64, f64, f64) {
        let d = p - self.o;
        let (x, y) = (d.dot(self.e1), d.dot(self.e2));
        (x.hypot(y), y.atan2(x), d.dot(self.u))
    }
    fn point(&self, r: f64, ang: f64, z: f64) -> Pnt {
        self.o + self.u * z + (self.e1 * ang.cos() + self.e2 * ang.sin()) * r
    }
    /// Is `axis` through `p` the same line?
    pub fn coaxial(&self, p: Pnt, axis: Pnt, tol: f64) -> bool {
        let a = axis.normalized();
        if a.cross(self.u).norm() > 1e-9 {
            return false;
        }
        let d = p - self.o;
        (d - self.u * d.dot(self.u)).norm() <= tol
    }
}

/// The ridge in helical coordinates.
struct Ridge {
    /// Section polygon as (zeta, r), unwound to `ang_ref`.
    poly: Vec<(f64, f64)>,
    zmin: f64,
    zmax: f64,
    /// Motion angle (in the thread's own direction) of the section.
    ang_ref: f64,
    /// Total motion angle.
    total: f64,
    /// Axial advance along the frame axis per turn (negative when the helix
    /// runs against it).
    adv: f64,
    /// Geometric angle about the frame axis per unit motion angle (+1/-1).
    rot: f64,
}

impl Ridge {
    fn new(sw: &HelicalSweep, f: &AxisFrame) -> Option<Self> {
        if sw.section.len() < 3 || sw.pitch <= 0.0 || sw.height <= 0.0 {
            return None;
        }
        let c: Vec<(f64, f64, f64)> = sw.section.iter().map(|p| f.cyl(*p)).collect();
        // The helix may run along or against the frame axis.
        let dir = if sw.axis.dot(f.u) >= 0.0 { 1.0 } else { -1.0 };
        let rot = sw.sign * dir;
        let adv = sw.pitch * dir;
        // motion angle = rot * geometric angle; unwrap around the first vertex
        let a0 = rot * c[0].1;
        let mut poly = Vec::with_capacity(c.len());
        for &(r, a, z) in &c {
            let mut d = rot * a - a0;
            while d > PI {
                d -= 2.0 * PI;
            }
            while d <= -PI {
                d += 2.0 * PI;
            }
            poly.push((z - adv * d / (2.0 * PI), r));
        }
        let zmin = poly.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let zmax = poly.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        if std::env::var("IRONSTREAM_DEBUG_FOLD").is_ok() {
            let mut zs: Vec<f64> = poly.iter().map(|p| p.0).collect();
            zs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let gaps: Vec<String> = zs
                .windows(2)
                .map(|w| format!("{:.4}", w[1] - w[0]))
                .collect();
            eprintln!("ridge: {} vertices, zeta gaps {:?}", poly.len(), gaps);
        }
        // one turn's copies must not overlap
        if zmax - zmin >= sw.pitch * 0.999 {
            return None;
        }
        let r = Self {
            poly,
            zmin,
            zmax,
            ang_ref: a0,
            total: 2.0 * PI * sw.height / sw.pitch,
            adv,
            rot,
        };
        // each zeta slice must be a single radial interval
        let mut zs: Vec<f64> = r.poly.iter().map(|p| p.0).collect();
        zs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for w in zs.windows(2) {
            if w[1] - w[0] > 1e-12 && r.slice((w[0] + w[1]) * 0.5).len() != 2 {
                return None;
            }
        }
        Some(r)
    }

    /// Radii where the section crosses `zeta`.
    fn slice(&self, s: f64) -> Vec<f64> {
        let n = self.poly.len();
        let mut out = Vec::new();
        for i in 0..n {
            let (a, b) = (self.poly[i], self.poly[(i + 1) % n]);
            if (a.0 < s) != (b.0 < s) {
                out.push(a.1 + (b.1 - a.1) * (s - a.0) / (b.0 - a.0));
            }
        }
        out
    }

    /// Axial offsets (z of the section copy's zeta origin) of every copy on
    /// the meridian at motion angle `ang`.
    fn copies(&self, ang: f64) -> Vec<f64> {
        let mut d = (ang - self.ang_ref) % (2.0 * PI);
        if d < 0.0 {
            d += 2.0 * PI;
        }
        let mut out = Vec::new();
        let mut a = d;
        while a <= self.total + 1e-12 {
            out.push(self.adv * a / (2.0 * PI));
            a += 2.0 * PI;
        }
        out
    }

    /// (inner, outer) radius of the ridge at `z` on a meridian, if present.
    fn at(&self, z: f64, offs: &[f64]) -> Option<(f64, f64)> {
        for &o in offs {
            let s = z - o;
            if s > self.zmin && s < self.zmax {
                let v = self.slice(s);
                if v.len() >= 2 {
                    let lo = v.iter().cloned().fold(f64::INFINITY, f64::min);
                    let hi = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    return Some((lo, hi));
                }
            }
        }
        None
    }
}

/// A radius function of z, piecewise linear over sorted breakpoints.
#[derive(Clone)]
pub struct Radius {
    pub pts: Vec<(f64, f64)>,
}

impl Radius {
    pub fn constant(z0: f64, z1: f64, r: f64) -> Self {
        Self {
            pts: vec![(z0, r), (z1, r)],
        }
    }
    fn lo(&self) -> f64 {
        self.pts[0].0
    }
    fn hi(&self) -> f64 {
        self.pts[self.pts.len() - 1].0
    }
    fn eval(&self, z: f64) -> f64 {
        let p = &self.pts;
        if z <= p[0].0 {
            return p[0].1;
        }
        for w in p.windows(2) {
            if z <= w[1].0 {
                let d = w[1].0 - w[0].0;
                return if d <= 0.0 {
                    w[1].1
                } else {
                    w[0].1 + (w[1].1 - w[0].1) * (z - w[0].0) / d
                };
            }
        }
        p[p.len() - 1].1
    }

    /// Outer radius of a revolved profile (world points in a plane through
    /// the axis), if the profile fills the axis out to one radius per z.
    pub fn of_revolve_profile(profile: &[Pnt], f: &AxisFrame) -> Option<Self> {
        let rz: Vec<(f64, f64)> = profile
            .iter()
            .map(|p| {
                let (r, _, z) = f.cyl(*p);
                (z, r)
            })
            .collect();
        let n = rz.len();
        if n < 3 {
            return None;
        }
        let mut zs: Vec<f64> = rz.iter().map(|p| p.0).collect();
        zs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        zs.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
        let outer = |z: f64| -> f64 {
            let mut best: f64 = 0.0;
            for i in 0..n {
                let (a, b) = (rz[i], rz[(i + 1) % n]);
                let (lo, hi) = (a.0.min(b.0), a.0.max(b.0));
                if z >= lo - 1e-12 && z <= hi + 1e-12 {
                    let r = if (b.0 - a.0).abs() < 1e-15 {
                        a.1.max(b.1)
                    } else {
                        a.1 + (b.1 - a.1) * (z - a.0) / (b.0 - a.0)
                    };
                    best = best.max(r);
                }
            }
            best
        };
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for (i, &z) in zs.iter().enumerate() {
            // one-sided limits catch vertical profile edges
            let eps = 1e-9 * (zs[zs.len() - 1] - zs[0]).max(1.0);
            let left = if i > 0 { outer(z - eps) } else { outer(z) };
            let right = if i + 1 < zs.len() {
                outer(z + eps)
            } else {
                outer(z)
            };
            pts.push((z, left));
            if (right - left).abs() > 1e-12 {
                pts.push((z, right));
            }
        }
        // the profile must be the region 0 <= r <= R(z): compare areas
        let mut area = 0.0;
        for i in 0..n {
            let (a, b) = (rz[i], rz[(i + 1) % n]);
            area += a.0 * b.1 - b.0 * a.1;
        }
        let area = area.abs() * 0.5;
        let mut integral = 0.0;
        for w in pts.windows(2) {
            integral += (w[1].0 - w[0].0) * (w[0].1 + w[1].1) * 0.5;
        }
        if (area - integral).abs() > 1e-6 * integral.max(1e-12) {
            return None;
        }
        Some(Self { pts })
    }
}

/// Which helical line a meridian point lies on: (thread turn, section
/// vertex, side of a step). Points with the same label on neighbouring
/// meridians are joined by quads, so crest and root lines stay unbroken.
type Label = Option<(i64, usize, u8)>;

/// External (`outer = true`: radius = max(body, ridge outer)) or internal
/// (`outer = false`: radius = min(bore, ridge inner)) threaded solid of
/// revolution, `n` meridians. `None` when the ridge does not touch the body
/// or bore everywhere it runs (the boolean is then the honest answer).
pub fn threaded_solid(
    body: &Radius,
    sw: &HelicalSweep,
    f: &AxisFrame,
    outer: bool,
    n: usize,
) -> Option<Solid> {
    let ridge = Ridge::new(sw, f)?;
    let (z0, z1) = (body.lo(), body.hi());
    let span = (z1 - z0).max(1e-9);
    let eps = 1e-9 * span.max(1.0);
    // A ridge modelled a hair off its body (stdlib threads sit 0.01 mm above
    // the core) still means a threaded body: bridge gaps up to 2 % of the
    // thread depth. OCCT's union keeps such a gap as a separate shell.
    let depth = {
        let lo = ridge.poly.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let hi = ridge
            .poly
            .iter()
            .map(|p| p.1)
            .fold(f64::NEG_INFINITY, f64::max);
        hi - lo
    };
    let tol = (1e-6 * span.max(1.0)).max(0.02 * depth);
    let ridge_lo = ridge.poly.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let ridge_hi = ridge
        .poly
        .iter()
        .map(|p| p.1)
        .fold(f64::NEG_INFINITY, f64::max);
    let mut columns: Vec<Vec<(f64, f64, Label)>> = Vec::with_capacity(n);
    for j in 0..n {
        let ang = 2.0 * PI * j as f64 / n as f64; // motion angle
        let offs = ridge.copies(ang);
        // breakpoints, labelled where they lie on a helical line
        let mut zs: Vec<(f64, Label)> = body.pts.iter().map(|p| (p.0, None)).collect();
        for &o in &offs {
            let line =
                ((o - ridge.adv * (ang - ridge.ang_ref) / (2.0 * PI)) / ridge.adv).round() as i64;
            for (vi, &(s, _)) in ridge.poly.iter().enumerate() {
                let z = s + o;
                if z > z0 && z < z1 {
                    zs.push((z, Some((line, vi, 0))));
                }
            }
        }
        zs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        zs.dedup_by(|a, b| (a.0 - b.0).abs() < eps);
        let value = |z: f64| -> Option<f64> {
            let b = body.eval(z);
            let root_in = ridge_lo - b; // outer thread: root above the body
            let root_out = b - ridge_hi; // inner thread: crest short of the bore
            match ridge.at(z, &offs) {
                None => Some(b),
                Some((lo, hi)) => {
                    // Close a hairline gap by moving the ridge onto the body
                    // (no 0.01 mm step ring of slivers at the root).
                    let (lo, hi) = if outer && root_in.abs() <= tol {
                        (lo - root_in, hi - root_in)
                    } else if !outer && root_out.abs() <= tol {
                        (lo + root_out, hi + root_out)
                    } else {
                        (lo, hi)
                    };
                    if outer {
                        if lo > b + tol {
                            if std::env::var("IRONSTREAM_DEBUG_FOLD").is_ok() {
                                eprintln!("float at z={z:.4} body r={b:.4} ridge {lo:.4}..{hi:.4}");
                            }
                            None // ridge floats off the body
                        } else {
                            Some(b.max(hi))
                        }
                    } else if hi < b - tol {
                        None // ridge floats inside the bore
                    } else {
                        Some(b.min(lo))
                    }
                }
            }
        };
        // add crossings of body and ridge radius inside each interval
        let mut full: Vec<(f64, Label)> = Vec::with_capacity(zs.len() * 2);
        for w in zs.windows(2) {
            full.push(w[0]);
            let (a, b) = (w[0].0 + eps, w[1].0 - eps);
            if b <= a {
                continue;
            }
            let g = |z: f64| -> Option<f64> {
                let r = ridge.at(z, &offs)?;
                let b = body.eval(z);
                let (root_in, root_out) = (ridge_lo - b, b - ridge_hi);
                let shift = if outer && root_in.abs() <= tol {
                    -root_in
                } else if !outer && root_out.abs() <= tol {
                    root_out
                } else {
                    0.0
                };
                Some(if outer { r.1 } else { r.0 } + shift - b)
            };
            if let (Some(ga), Some(gb)) = (g(a), g(b)) {
                if (ga > 0.0) != (gb > 0.0) {
                    full.push((a + (b - a) * ga / (ga - gb), None));
                }
            }
        }
        full.push(*zs.last()?);
        // Breakpoints closer than a thousandth of the pitch (a root landing on
        // the core radius puts a crossing right next to a section vertex)
        // only make slivers: keep the first of each cluster, and the ends.
        let gap = 1e-3 * ridge.adv.abs();
        let last = full.last()?.0;
        let mut merged: Vec<(f64, Label)> = Vec::with_capacity(full.len());
        for &(z, l) in &full {
            match merged.last_mut() {
                // a ridge vertex wins over an unlabelled neighbour
                Some(p) if z - p.0 < gap && z != last => {
                    if p.1.is_none() && l.is_some() {
                        *p = (z, l);
                    }
                }
                Some(p) if z - p.0 < gap => *p = (z, None),
                _ => merged.push((z, l)),
            }
        }
        if merged.first() != full.first() {
            merged.insert(0, full[0]);
        }
        let full = merged;
        // a real step (vertical profile edge) is far above the slope * eps
        // noise of one-sided evaluation at an ordinary vertex
        let jump = (1e-6 * span.max(1.0)).max(1e-3 * depth);
        let mut col: Vec<(f64, f64, Label)> = Vec::with_capacity(full.len() * 2);
        for (i, &(z, lab)) in full.iter().enumerate() {
            let l = if i > 0 { value(z - eps)? } else { value(z)? };
            let r = if i + 1 < full.len() {
                value(z + eps)?
            } else {
                value(z)?
            };
            if (r - l).abs() > jump {
                col.push((z, l, lab));
                col.push((z, r, lab.map(|(t, v, _)| (t, v, 1))));
            } else {
                col.push((z, 0.5 * (l + r), lab));
            }
        }
        columns.push(col);
    }

    let mut m = TriMesh::new();
    let vid = |m: &mut TriMesh, p: Pnt| {
        m.verts.push(p);
        m.verts.len() - 1
    };
    let geo = |j: usize| ridge.rot * 2.0 * PI * j as f64 / n as f64; // geometric angle
    let ids: Vec<Vec<usize>> = columns
        .iter()
        .enumerate()
        .map(|(j, col)| {
            col.iter()
                .map(|&(z, r, _)| vid(&mut m, f.point(r, geo(j), z)))
                .collect()
        })
        .collect();
    for j in 0..n {
        let (a, b) = (&columns[j], &columns[(j + 1) % n]);
        let (ia, ib) = (&ids[j], &ids[(j + 1) % n]);
        // Seen from the last meridian, column 0 is one turn further on.
        let shift = if j + 1 == n { 1 } else { 0 };
        let la: Vec<Label> = a.iter().map(|p| p.2).collect();
        let lb: Vec<Label> = b
            .iter()
            .map(|p| p.2.map(|(t, v, s)| (t + shift, v, s)))
            .collect();
        let pos_a: std::collections::HashMap<(i64, usize, u8), usize> = la
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.map(|l| (l, i)))
            .collect();
        let pos_b: std::collections::HashMap<(i64, usize, u8), usize> = lb
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.map(|l| (l, i)))
            .collect();
        let (mut i, mut k) = (0usize, 0usize);
        while i + 1 < a.len() || k + 1 < b.len() {
            let both = i + 1 < a.len() && k + 1 < b.len();
            if both && la[i + 1].is_some() && la[i + 1] == lb[k + 1] {
                // same helical line on both meridians: a quad
                m.tris.push([ia[i], ib[k], ia[i + 1]]);
                m.tris.push([ia[i + 1], ib[k], ib[k + 1]]);
                i += 1;
                k += 1;
                continue;
            }
            // walk the side that has not reached the other's next line yet
            // (only for a line within a pitch: where a thread copy starts or
            // ends between meridians, plain height order is the better fan)
            let near = ridge.adv.abs();
            let a_waits = both
                && la[i + 1]
                    .and_then(|l| pos_b.get(&l))
                    .is_some_and(|&q| q > k + 1 && b[q].0 - b[k + 1].0 < near);
            let b_waits = both
                && lb[k + 1]
                    .and_then(|l| pos_a.get(&l))
                    .is_some_and(|&q| q > i + 1 && a[q].0 - a[i + 1].0 < near);
            let take_a = if a_waits && !b_waits {
                false
            } else if b_waits && !a_waits {
                true
            } else {
                k + 1 >= b.len() || (i + 1 < a.len() && a[i + 1].0 <= b[k + 1].0)
            };
            // outward for a right-handed sweep; flipped below otherwise
            if take_a {
                m.tris.push([ia[i], ib[k], ia[i + 1]]);
                i += 1;
            } else {
                m.tris.push([ia[i], ib[k], ib[k + 1]]);
                k += 1;
            }
        }
    }
    // end caps
    let bot = vid(&mut m, f.point(0.0, 0.0, z0));
    let top = vid(&mut m, f.point(0.0, 0.0, z1));
    for j in 0..n {
        let jn = (j + 1) % n;
        m.tris.push([bot, ids[jn][0], ids[j][0]]);
        m.tris
            .push([top, ids[j][ids[j].len() - 1], ids[jn][ids[jn].len() - 1]]);
    }
    let mut m = m.welded(1e-9 * span.max(1.0));
    if m.signed_volume() < 0.0 {
        m.flip();
    }
    Some(Solid::from_mesh(m))
}
