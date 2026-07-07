//! `BRepAlgoAPI` — boolean operations on solids, mirroring OCCT's
//! `BRepAlgoAPI_Fuse` / `_Cut` / `_Common`. Thin wrappers over the from-scratch
//! BSP CSG engine (`bsp`), operating on solid boundary meshes.

use crate::bsp;
use crate::mesh::TriMesh;
use crate::topods::Solid;

const WELD_TOL: f64 = 1e-6;

/// Do two solids' bounding boxes overlap (with a small margin)? If not, their
/// boundaries cannot intersect, so a union is just a concatenation of meshes.
fn bboxes_overlap(a: &Solid, b: &Solid) -> bool {
    let (ba, bb) = (a.bbox(), b.bbox());
    let eps = WELD_TOL;
    (0..3).all(|i| ba.min[i] <= bb.max[i] + eps && bb.min[i] <= ba.max[i] + eps)
}

/// Concatenate two boundary meshes (no CSG) — valid only for disjoint solids.
fn concat_meshes(a: &TriMesh, b: &TriMesh) -> TriMesh {
    let mut out = a.clone();
    let base = out.verts.len();
    out.verts.extend_from_slice(&b.verts);
    out.tris
        .extend(b.tris.iter().map(|t| [t[0] + base, t[1] + base, t[2] + base]));
    out
}

/// A ∪ B (`BRepAlgoAPI_Fuse`). Empty operands are treated as identities.
// occt: BRepAlgoAPI_Fuse
pub fn fuse(a: &Solid, b: &Solid) -> Solid {
    if a.is_empty() {
        return b.clone();
    }
    if b.is_empty() {
        return a.clone();
    }
    // Fast path: disjoint solids can't share boundary, so union == concat. This
    // sidesteps the BSP entirely — the difference between an O(n·m) split and a
    // cheap append, which is what makes drilling N holes (fuse the tools, then
    // one cut) scale instead of fragmenting the mesh on every step.
    let m = if bboxes_overlap(a, b) {
        bsp::union(a.mesh(), b.mesh())
    } else {
        concat_meshes(a.mesh(), b.mesh())
    };
    let mut out = Solid::from_mesh(m.welded(WELD_TOL));
    out.merge_hints_from(a);
    out.merge_hints_from(b);
    out
}

/// A \ B (`BRepAlgoAPI_Cut`).
// occt: BRepAlgoAPI_Cut
pub fn cut(a: &Solid, b: &Solid) -> Solid {
    if a.is_empty() || b.is_empty() {
        return a.clone();
    }
    let m = bsp::subtract(a.mesh(), b.mesh());
    let mut out = Solid::from_mesh(m.welded(WELD_TOL));
    out.merge_hints_from(a);
    out.merge_hints_from(b);
    out
}

/// A ∩ B (`BRepAlgoAPI_Common`).
// occt: BRepAlgoAPI_Common
pub fn common(a: &Solid, b: &Solid) -> Solid {
    if a.is_empty() || b.is_empty() {
        return Solid::empty();
    }
    let m = bsp::intersect(a.mesh(), b.mesh());
    let mut out = Solid::from_mesh(m.welded(WELD_TOL));
    out.merge_hints_from(a);
    out.merge_hints_from(b);
    out
}

/// Fuse a list of solids into one (left fold). Returns empty for an empty list.
pub fn fuse_all(solids: &[Solid]) -> Solid {
    let mut iter = solids.iter().filter(|s| !s.is_empty());
    let mut acc = match iter.next() {
        Some(s) => s.clone(),
        None => return Solid::empty(),
    };
    for s in iter {
        acc = fuse(&acc, s);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_prim_api::make_box;
    use crate::gp::{Pnt, Trsf};

    #[test]
    fn disjoint_fuse_is_concat_and_exact() {
        let a = make_box(Pnt::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
        let b = make_box(Pnt::new(20.0, 0.0, 0.0), 10.0, 10.0, 10.0);
        assert!(!bboxes_overlap(&a, &b));
        assert!((fuse(&a, &b).volume() - 2000.0).abs() < 1e-9);
    }

    #[test]
    fn overlapping_fuse_still_uses_csg() {
        let a = make_box(Pnt::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
        let b = a.transformed(&Trsf::translation(Pnt::new(5.0, 0.0, 0.0)));
        assert!(bboxes_overlap(&a, &b));
        assert!((fuse(&a, &b).volume() - 1500.0).abs() < 1e-6);
    }
}
