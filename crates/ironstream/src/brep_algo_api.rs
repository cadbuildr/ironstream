//! `BRepAlgoAPI` — boolean operations on solids, mirroring OCCT's
//! `BRepAlgoAPI_Fuse` / `_Cut` / `_Common`. Thin wrappers over the from-scratch
//! BSP CSG engine (`bsp`), operating on solid boundary meshes.

use crate::bsp;
use crate::topods::Solid;

const WELD_TOL: f64 = 1e-6;

/// A ∪ B (`BRepAlgoAPI_Fuse`). Empty operands are treated as identities.
// occt: BRepAlgoAPI_Fuse
pub fn fuse(a: &Solid, b: &Solid) -> Solid {
    if a.is_empty() {
        return b.clone();
    }
    if b.is_empty() {
        return a.clone();
    }
    let m = bsp::union(a.mesh(), b.mesh());
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
