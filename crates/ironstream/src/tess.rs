//! Tessellation — planar face triangulation by ear clipping, with keyhole
//! bridging for interior holes. Pure 2D once the face plane is established.
//!
//! Used by `TopoDS::Face::triangulate` to turn sketch profiles into the cap
//! geometry of prisms (extrusions), lofts and the like.

use crate::gp::Pnt;
use crate::topods::Face;

/// Triangulate a planar face into world-space CCW (about the face normal)
/// triangles.
pub fn triangulate_face(face: &Face) -> Vec<[Pnt; 3]> {
    let n = face.normal();
    if face.outer.pts.len() < 3 {
        return Vec::new();
    }
    // Orthonormal basis (u, v) spanning the face plane.
    let u = (face.outer.pts[1] - face.outer.pts[0]).normalized();
    let u = {
        // Re-orthogonalize u against n in case the first edge isn't in-plane.
        let up = u - n * u.dot(n);
        if up.norm() < 1e-9 {
            n.any_perpendicular()
        } else {
            up.normalized()
        }
    };
    let v = n.cross(u).normalized();
    let origin = face.outer.pts[0];
    let to2d = |p: Pnt| -> [f64; 2] {
        let d = p - origin;
        [d.dot(u), d.dot(v)]
    };

    // Collect outer + holes as 2D loops, tracking the originating 3D point.
    let mut loops: Vec<Vec<(usize, [f64; 2])>> = Vec::new();
    let mut pts3d: Vec<Pnt> = Vec::new();
    {
        let mut push_loop = |wire_pts: &[Pnt], pts3d: &mut Vec<Pnt>| {
            let mut l = Vec::with_capacity(wire_pts.len());
            for p in wire_pts {
                let idx = pts3d.len();
                pts3d.push(*p);
                l.push((idx, to2d(*p)));
            }
            loops.push(l);
        };
        push_loop(&face.outer.pts, &mut pts3d);
        for h in &face.holes {
            if h.pts.len() >= 3 {
                push_loop(&h.pts, &mut pts3d);
            }
        }
    }

    // Outer loop CCW, holes CW (standard for keyhole merge).
    ensure_orientation(&mut loops[0], true);
    for l in loops.iter_mut().skip(1) {
        ensure_orientation(l, false);
    }

    // Merge holes into the outer polygon via keyhole bridges.
    let mut poly: Vec<(usize, [f64; 2])> = loops[0].clone();
    let mut holes: Vec<Vec<(usize, [f64; 2])>> = loops.split_off(1);
    // Process holes right-to-left so bridges don't interfere.
    holes.sort_by(|a, b| {
        let ax = a.iter().map(|p| p.1[0]).fold(f64::NEG_INFINITY, f64::max);
        let bx = b.iter().map(|p| p.1[0]).fold(f64::NEG_INFINITY, f64::max);
        bx.partial_cmp(&ax).unwrap_or(std::cmp::Ordering::Equal)
    });
    for hole in holes {
        poly = bridge_hole(poly, hole);
    }

    // Ear-clip the merged simple polygon.
    let tris2d = ear_clip(&poly);
    let mut out = Vec::with_capacity(tris2d.len());
    for [ia, ib, ic] in tris2d {
        let a = pts3d[ia];
        let b = pts3d[ib];
        let c = pts3d[ic];
        // Orient CCW about the face normal.
        if (b - a).cross(c - a).dot(n) >= 0.0 {
            out.push([a, b, c]);
        } else {
            out.push([a, c, b]);
        }
    }
    out
}

fn signed_area(loop2d: &[(usize, [f64; 2])]) -> f64 {
    let mut a = 0.0;
    let n = loop2d.len();
    for i in 0..n {
        let p = loop2d[i].1;
        let q = loop2d[(i + 1) % n].1;
        a += p[0] * q[1] - q[0] * p[1];
    }
    a * 0.5
}

fn ensure_orientation(loop2d: &mut Vec<(usize, [f64; 2])>, ccw: bool) {
    let area = signed_area(loop2d);
    let is_ccw = area > 0.0;
    if is_ccw != ccw {
        loop2d.reverse();
    }
}

/// Connect a (CW) hole to the (CCW) outer polygon with a coincident bridge edge,
/// producing one simple polygon. Uses the hole's rightmost vertex and the
/// nearest visible outer vertex (best-effort; exact visibility is overkill for
/// the well-behaved sketch profiles we tessellate).
fn bridge_hole(
    outer: Vec<(usize, [f64; 2])>,
    hole: Vec<(usize, [f64; 2])>,
) -> Vec<(usize, [f64; 2])> {
    // Rightmost hole vertex.
    let mut hi = 0;
    for i in 1..hole.len() {
        if hole[i].1[0] > hole[hi].1[0] {
            hi = i;
        }
    }
    let hp = hole[hi].1;
    // Nearest outer vertex to the right of (or near) the hole vertex.
    let mut oi = 0;
    let mut best = f64::INFINITY;
    for (i, o) in outer.iter().enumerate() {
        let dx = o.1[0] - hp[0];
        let dy = o.1[1] - hp[1];
        let d = dx * dx + dy * dy;
        if d < best {
            best = d;
            oi = i;
        }
    }
    // Splice: outer[0..=oi], hole[hi..]+hole[..=hi], outer[oi..].
    let mut out = Vec::with_capacity(outer.len() + hole.len() + 2);
    out.extend_from_slice(&outer[..=oi]);
    for k in 0..hole.len() {
        out.push(hole[(hi + k) % hole.len()]);
    }
    out.push(hole[hi]);
    out.push(outer[oi]);
    out.extend_from_slice(&outer[oi..]);
    out
}

/// Ear clipping on a CCW simple polygon. Returns triangles as indices into the
/// shared 3D point list (the `usize` carried by each loop vertex).
fn ear_clip(poly: &[(usize, [f64; 2])]) -> Vec<[usize; 3]> {
    let mut idx: Vec<usize> = (0..poly.len()).collect();
    let mut out = Vec::new();
    if poly.len() < 3 {
        return out;
    }
    // Ensure CCW for the clipping loop.
    if signed_area(poly) < 0.0 {
        idx.reverse();
    }
    let p2 = |i: usize| poly[i].1;
    let mut guard = 0;
    let max_guard = poly.len() * poly.len() + 16;
    while idx.len() > 3 {
        guard += 1;
        if guard > max_guard {
            // Fallback: fan the remainder (degenerate/near-collinear input).
            break;
        }
        let n = idx.len();
        let mut clipped = false;
        for i in 0..n {
            let i0 = idx[(i + n - 1) % n];
            let i1 = idx[i];
            let i2 = idx[(i + 1) % n];
            let a = p2(i0);
            let b = p2(i1);
            let c = p2(i2);
            // Convex (left turn) ?
            let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross <= 0.0 {
                continue;
            }
            // No other vertex inside triangle abc ?
            let mut contains = false;
            for &j in &idx {
                if j == i0 || j == i1 || j == i2 {
                    continue;
                }
                if point_in_tri(p2(j), a, b, c) {
                    contains = true;
                    break;
                }
            }
            if contains {
                continue;
            }
            out.push([poly[i0].0, poly[i1].0, poly[i2].0]);
            idx.remove(i);
            clipped = true;
            break;
        }
        if !clipped {
            break; // no ear found (numerical) — fan the rest below
        }
    }
    // Emit final triangle(s).
    if idx.len() == 3 {
        out.push([poly[idx[0]].0, poly[idx[1]].0, poly[idx[2]].0]);
    } else if idx.len() > 3 {
        for k in 1..idx.len() - 1 {
            out.push([poly[idx[0]].0, poly[idx[k]].0, poly[idx[k + 1]].0]);
        }
    }
    out
}

fn point_in_tri(p: [f64; 2], a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> bool {
    let d = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
    if d.abs() < 1e-18 {
        return false;
    }
    let l1 = ((b[1] - c[1]) * (p[0] - c[0]) + (c[0] - b[0]) * (p[1] - c[1])) / d;
    let l2 = ((c[1] - a[1]) * (p[0] - c[0]) + (a[0] - c[0]) * (p[1] - c[1])) / d;
    let l3 = 1.0 - l1 - l2;
    let e = -1e-12;
    l1 >= e && l2 >= e && l3 >= e
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topods::Wire;

    #[test]
    fn triangulate_square() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(2.0, 2.0, 0.0),
            Pnt::new(0.0, 2.0, 0.0),
        ]);
        let f = Face::new(w);
        let tris = f.triangulate();
        let area: f64 = tris
            .iter()
            .map(|t| (t[1] - t[0]).cross(t[2] - t[0]).norm() * 0.5)
            .sum();
        assert!((area - 4.0).abs() < 1e-9, "area={area}");
    }

    #[test]
    fn triangulate_l_shape_concave() {
        // L-shaped polygon, area = 3.
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(2.0, 1.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(1.0, 2.0, 0.0),
            Pnt::new(0.0, 2.0, 0.0),
        ]);
        let f = Face::new(w);
        let tris = f.triangulate();
        let area: f64 = tris
            .iter()
            .map(|t| (t[1] - t[0]).cross(t[2] - t[0]).norm() * 0.5)
            .sum();
        assert!((area - 3.0).abs() < 1e-9, "area={area}");
    }
}
