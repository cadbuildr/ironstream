//! Triangle mesh — the tessellated boundary representation IronStream produces
//! and on which volumes, bounding boxes and boolean operations are computed.
//!
//! This is the concrete carrier behind `TopoDS_Solid` (see `topods.rs`). The
//! kernel keeps analytic surface metadata on faces where it can, but every
//! solid can always be reduced to this watertight triangle soup for meshing,
//! STL export and CSG.

use crate::gp::{Pnt, Trsf};

/// An axis-aligned bounding box.
#[derive(Clone, Copy, Debug)]
// occt: Bnd_Box
pub struct BBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BBox {
    pub fn empty() -> Self {
        Self {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        }
    }

    pub fn add(&mut self, p: Pnt) {
        let a = p.as_array();
        for i in 0..3 {
            if a[i] < self.min[i] {
                self.min[i] = a[i];
            }
            if a[i] > self.max[i] {
                self.max[i] = a[i];
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min[0] > self.max[0]
    }

    pub fn volume(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        let dx = (self.max[0] - self.min[0]).max(0.0);
        let dy = (self.max[1] - self.min[1]).max(0.0);
        let dz = (self.max[2] - self.min[2]).max(0.0);
        dx * dy * dz
    }

    /// Intersection-over-union of two AABBs (a bounding-box overlap metric).
    pub fn iou(&self, other: &BBox) -> f64 {
        if self.is_empty() || other.is_empty() {
            return 0.0;
        }
        let mut imin = [0.0; 3];
        let mut imax = [0.0; 3];
        for i in 0..3 {
            imin[i] = self.min[i].max(other.min[i]);
            imax[i] = self.max[i].min(other.max[i]);
        }
        let mut inter = 1.0;
        for i in 0..3 {
            inter *= (imax[i] - imin[i]).max(0.0);
        }
        let union = self.volume() + other.volume() - inter;
        if union > 0.0 {
            inter / union
        } else {
            0.0
        }
    }
}

/// A watertight (by construction) triangle mesh.
#[derive(Clone, Debug, Default)]
// occt: Poly_Triangulation
pub struct TriMesh {
    pub verts: Vec<Pnt>,
    pub tris: Vec<[usize; 3]>,
}

impl TriMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.tris.is_empty()
    }

    pub fn triangle_count(&self) -> usize {
        self.tris.len()
    }

    /// Push three world-space corners as a triangle, appending vertices.
    pub fn push_triangle(&mut self, a: Pnt, b: Pnt, c: Pnt) {
        let i = self.verts.len();
        self.verts.push(a);
        self.verts.push(b);
        self.verts.push(c);
        self.tris.push([i, i + 1, i + 2]);
    }

    /// Push a convex polygon (CCW) as a triangle fan.
    pub fn push_polygon(&mut self, poly: &[Pnt]) {
        if poly.len() < 3 {
            return;
        }
        for k in 1..poly.len() - 1 {
            self.push_triangle(poly[0], poly[k], poly[k + 1]);
        }
    }

    /// Append another mesh's geometry into this one.
    pub fn extend(&mut self, other: &TriMesh) {
        let base = self.verts.len();
        self.verts.extend_from_slice(&other.verts);
        for t in &other.tris {
            self.tris.push([t[0] + base, t[1] + base, t[2] + base]);
        }
    }

    pub fn bbox(&self) -> BBox {
        let mut b = BBox::empty();
        for v in &self.verts {
            b.add(*v);
        }
        b
    }

    /// Signed volume via the divergence theorem (sum of tetra signed volumes
    /// to the origin). For a closed, outward-oriented mesh the magnitude is the
    /// enclosed volume; the sign is positive when normals point outward.
    pub fn signed_volume(&self) -> f64 {
        let mut v = 0.0;
        for t in &self.tris {
            let a = self.verts[t[0]];
            let b = self.verts[t[1]];
            let c = self.verts[t[2]];
            v += a.dot(b.cross(c));
        }
        v / 6.0
    }

    pub fn volume(&self) -> f64 {
        self.signed_volume().abs()
    }

    pub fn surface_area(&self) -> f64 {
        let mut s = 0.0;
        for t in &self.tris {
            let a = self.verts[t[0]];
            let b = self.verts[t[1]];
            let c = self.verts[t[2]];
            s += (b - a).cross(c - a).norm() * 0.5;
        }
        s
    }

    /// Apply an affine transform in place. If the transform is
    /// orientation-reversing (negative determinant, e.g. a mirror) we flip
    /// triangle winding so normals stay outward and volume stays positive.
    pub fn transform(&mut self, t: &Trsf) {
        for v in &mut self.verts {
            *v = t.apply_point(*v);
        }
        if t.linear_det() < 0.0 {
            for tri in &mut self.tris {
                tri.swap(1, 2);
            }
        }
    }

    pub fn transformed(&self, t: &Trsf) -> TriMesh {
        let mut m = self.clone();
        m.transform(t);
        m
    }

    /// Flip every triangle's orientation (used by CSG inversion).
    pub fn flip(&mut self) {
        for tri in &mut self.tris {
            tri.swap(1, 2);
        }
    }

    /// Weld coincident vertices on a grid of `tol` and drop degenerate / exactly
    /// duplicated triangles. Keeps STL output compact and the volume integral
    /// numerically clean without changing geometry.
    pub fn welded(&self, tol: f64) -> TriMesh {
        use std::collections::HashMap;
        let inv = if tol > 0.0 { 1.0 / tol } else { 1.0e9 };
        let key = |p: Pnt| -> (i64, i64, i64) {
            (
                (p.x * inv).round() as i64,
                (p.y * inv).round() as i64,
                (p.z * inv).round() as i64,
            )
        };
        let mut map: HashMap<(i64, i64, i64), usize> = HashMap::new();
        let mut out = TriMesh::new();
        let mut remap = Vec::with_capacity(self.verts.len());
        for v in &self.verts {
            let k = key(*v);
            let idx = *map.entry(k).or_insert_with(|| {
                out.verts.push(*v);
                out.verts.len() - 1
            });
            remap.push(idx);
        }
        for t in &self.tris {
            let (a, b, c) = (remap[t[0]], remap[t[1]], remap[t[2]]);
            if a == b || b == c || a == c {
                continue; // degenerate
            }
            out.tris.push([a, b, c]);
        }
        out
    }

    /// Number of boundary edges (edges used by exactly one triangle). Zero for a
    /// closed manifold; a quick watertightness sanity signal.
    pub fn open_edge_count(&self, tol: f64) -> usize {
        use std::collections::HashMap;
        let welded = self.welded(tol);
        let mut edges: HashMap<(usize, usize), i32> = HashMap::new();
        for t in &welded.tris {
            for &(i, j) in &[(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                let k = if i < j { (i, j) } else { (j, i) };
                *edges.entry(k).or_insert(0) += 1;
            }
        }
        edges.values().filter(|&&c| c != 2).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;

    /// A unit cube built by hand should have volume 1 and area 6.
    fn unit_cube() -> TriMesh {
        let p = |x: f64, y: f64, z: f64| Pnt::new(x, y, z);
        let mut m = TriMesh::new();
        // 8 corners
        let v = [
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.0, 1.0, 0.0),
            p(0.0, 1.0, 0.0),
            p(0.0, 0.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
            p(0.0, 1.0, 1.0),
        ];
        // outward-facing quads (CCW seen from outside) -> two tris each
        let quad = |m: &mut TriMesh, a: usize, b: usize, c: usize, d: usize| {
            m.push_triangle(v[a], v[b], v[c]);
            m.push_triangle(v[a], v[c], v[d]);
        };
        quad(&mut m, 0, 3, 2, 1); // bottom (z=0, normal -Z)
        quad(&mut m, 4, 5, 6, 7); // top (z=1, normal +Z)
        quad(&mut m, 0, 1, 5, 4); // y=0
        quad(&mut m, 1, 2, 6, 5); // x=1
        quad(&mut m, 2, 3, 7, 6); // y=1
        quad(&mut m, 3, 0, 4, 7); // x=0
        m
    }

    #[test]
    fn cube_volume_and_area() {
        let m = unit_cube();
        assert!((m.volume() - 1.0).abs() < 1e-9, "vol={}", m.volume());
        assert!((m.surface_area() - 6.0).abs() < 1e-9);
        assert_eq!(m.open_edge_count(1e-6), 0, "cube must be watertight");
    }

    #[test]
    fn bbox_iou_identity() {
        let m = unit_cube();
        let b = m.bbox();
        assert!((b.iou(&b) - 1.0).abs() < 1e-12);
    }
}
