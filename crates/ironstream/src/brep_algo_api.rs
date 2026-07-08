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
    let disjoint = !bboxes_overlap(a, b);
    let m = if disjoint {
        concat_meshes(a.mesh(), b.mesh())
    } else {
        bsp::union(a.mesh(), b.mesh())
    };
    let mut out = Solid::from_mesh(m.welded(WELD_TOL));
    out.merge_hints_from(a);
    out.merge_hints_from(b);
    // disjoint solids' B-reps concatenate exactly (a BSolid may be multi-shell)
    if disjoint {
        if let (Some(ba), Some(bb)) = (a.brep(), b.brep()) {
            let mut faces = ba.faces.clone();
            faces.extend(bb.faces.iter().cloned());
            out.set_brep(Some(crate::brep::BSolid::new(faces)));
        }
    }
    out
}

/// A \ B (`BRepAlgoAPI_Cut`).
// occt: BRepAlgoAPI_Cut
pub fn cut(a: &Solid, b: &Solid) -> Solid {
    if a.is_empty() || b.is_empty() {
        return a.clone();
    }
    // Exact drill fast path: when the target carries a B-rep and the tool is
    // (a fused set of) plain cylinders that pass fully through, punch exact
    // bores instead of running the mesh BSP — analytic barrels, exact volume.
    if std::env::var("IRONSTREAM_NO_EXACT_DRILL").is_err() {
        if let Some(out) = try_exact_drill(a, b) {
            return out;
        }
    }
    let m = bsp::subtract(a.mesh(), b.mesh());
    let mut out = Solid::from_mesh(m.welded(WELD_TOL));
    out.merge_hints_from(a);
    out.merge_hints_from(b);
    out
}

/// The exact bore path for `cut`: every tool component must be a full-period
/// cylinder that spans past the target along its axis, and every punch must be
/// clean (see `brep::drill_through`). Any deviation returns `None` and the
/// mesh boolean runs as before.
fn try_exact_drill(a: &Solid, b: &Solid) -> Option<Solid> {
    let target = a.brep()?;
    let tools = crate::brep::as_cylinder_tools(b.brep()?)?;
    let bbox = a.bbox();
    let corners: Vec<crate::gp::Pnt> = (0..8)
        .map(|i| {
            crate::gp::Pnt::new(
                if i & 1 == 0 { bbox.min[0] } else { bbox.max[0] },
                if i & 2 == 0 { bbox.min[1] } else { bbox.max[1] },
                if i & 4 == 0 { bbox.min[2] } else { bbox.max[2] },
            )
        })
        .collect();
    let mut cur = target.clone();
    for (axis, radius, v0, v1) in tools {
        let dir = axis.z_dir;
        // the tool must span past the whole target along its axis (a through
        // cut): blind holes stay on the mesh path.
        let ts: Vec<f64> = corners.iter().map(|c| (*c - axis.location).dot(dir)).collect();
        let (tmin, tmax) = ts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &t| {
            (lo.min(t), hi.max(t))
        });
        if v0 > tmin + 1e-9 || v1 < tmax - 1e-9 {
            return None;
        }
        cur = crate::brep::drill_through(&cur, axis.location, dir, radius)?;
    }
    let mesh = cur.tessellate(&crate::brep::TessParams::default());
    let mut out = Solid::from_mesh(mesh.welded(WELD_TOL));
    out.merge_hints_from(a);
    out.merge_hints_from(b);
    out.set_brep(Some(cur));
    Some(out)
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

#[cfg(test)]
mod exact_drill_tests {
    use super::*;
    use crate::brep_builder_api::{make_face, make_face_with_holes, make_polygon};
    use crate::brep_prim_api::{make_box, make_prism};
    use crate::gp::Pnt;
    use std::f64::consts::PI;

    fn circle_wire(cx: f64, cy: f64, z: f64, r: f64) -> crate::topods::Wire {
        let pts: Vec<Pnt> = (0..64)
            .map(|i| {
                let a = 2.0 * PI * i as f64 / 64.0;
                Pnt::new(cx + r * a.cos(), cy + r * a.sin(), z)
            })
            .collect();
        make_polygon(&pts)
    }

    #[test]
    fn prism_of_circle_is_exact_cylinder() {
        let face = make_face(circle_wire(5.0, 5.0, 0.0, 3.0));
        let s = make_prism(&face, Pnt::new(0.0, 0.0, 8.0));
        assert!(s.brep().is_some(), "circle prism carries a brep");
        let exact = PI * 9.0 * 8.0;
        assert!((s.volume() - exact).abs() < 1e-9, "vol={} exact={exact}", s.volume());
    }

    #[test]
    fn prism_with_circle_holes_is_exact_at_birth() {
        // a plate whose drilled holes come from the profile itself
        let outer = make_polygon(&[
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(20.0, 0.0, 0.0),
            Pnt::new(20.0, 20.0, 0.0),
            Pnt::new(0.0, 20.0, 0.0),
        ]);
        let face = make_face_with_holes(outer, vec![
            circle_wire(6.0, 10.0, 0.0, 2.0),
            circle_wire(14.0, 10.0, 0.0, 2.0),
        ]);
        let s = make_prism(&face, Pnt::new(0.0, 0.0, 5.0));
        let exact = 2000.0 - 2.0 * PI * 4.0 * 5.0;
        assert!((s.volume() - exact).abs() < 1e-9, "vol={} exact={exact}", s.volume());
    }

    #[test]
    fn cut_takes_exact_drill_path() {
        // plate minus a fused pair of circular cut prisms = two exact bores
        let plate = make_box(Pnt::new(0.0, 0.0, 0.0), 30.0, 30.0, 4.0);
        let t1 = make_prism(&make_face(circle_wire(8.0, 15.0, -1.0, 2.5)), Pnt::new(0.0, 0.0, 6.0));
        let t2 = make_prism(&make_face(circle_wire(22.0, 15.0, -1.0, 2.5)), Pnt::new(0.0, 0.0, 6.0));
        let tools = fuse_all(&[t1, t2]);
        assert!(tools.brep().is_some(), "disjoint fuse concatenates breps");
        let out = cut(&plate, &tools);
        assert!(out.brep().is_some(), "cut kept the exact path");
        let exact = 30.0 * 30.0 * 4.0 - 2.0 * PI * 2.5 * 2.5 * 4.0;
        assert!((out.volume() - exact).abs() < 1e-9, "vol={} exact={exact}", out.volume());
        // and the mesh is a sane watertight tessellation of the same solid
        assert!(out.mesh().volume() > 0.0 && (out.mesh().volume() - exact).abs() / exact < 2e-3);
    }

    #[test]
    fn blind_cut_falls_back_to_mesh() {
        // tool stops inside the plate: not a through cut -> mesh path
        let plate = make_box(Pnt::new(0.0, 0.0, 0.0), 20.0, 20.0, 10.0);
        let tool = make_prism(&make_face(circle_wire(10.0, 10.0, 4.0, 3.0)), Pnt::new(0.0, 0.0, 10.0));
        let out = cut(&plate, &tool);
        assert!(out.brep().is_none(), "blind hole stays on the mesh path");
        let approx = 4000.0 - PI * 9.0 * 6.0;
        assert!((out.volume() - approx).abs() / approx < 5e-3, "vol={}", out.volume());
    }

    #[test]
    fn transformed_prism_keeps_exact_volume() {
        use crate::gp::{Ax1, Trsf};
        let face = make_face(circle_wire(0.0, 0.0, 0.0, 3.0));
        let s = make_prism(&face, Pnt::new(0.0, 0.0, 8.0));
        let t = Trsf::rotation(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)), 0.7)
            .then(&Trsf::translation(Pnt::new(4.0, 5.0, 6.0)));
        let moved = s.transformed(&t);
        assert!(moved.brep().is_some(), "rigid transform keeps the brep");
        assert!((moved.volume() - PI * 9.0 * 8.0).abs() < 1e-9, "vol={}", moved.volume());
    }
}
