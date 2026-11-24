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

    // earcut handles hole bridging (visible bridges), duplicate/collinear
    // points and self-touching rings; winding of each ring is normalised inside.
    let mut flat: Vec<[f64; 2]> = Vec::with_capacity(pts3d.len());
    let mut starts = Vec::new();
    for (k, l) in loops.iter().enumerate() {
        if k > 0 {
            starts.push(flat.len());
        }
        flat.extend(l.iter().map(|p| p.1));
    }
    let tris2d: Vec<[usize; 3]> = crate::earcut::earcut(&flat, &starts);
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
