//! BSP-tree constructive solid geometry (CSG) — the boolean engine.
//!
//! This is a from-scratch Rust implementation of the binary space partitioning
//! CSG algorithm (Naylor, Amanatides & Thibault, "Merging BSP trees yields
//! polyhedral set operations", SIGGRAPH 1990). It is original code: no lines
//! are taken from any existing CAD kernel or CSG library — only the published
//! algorithm is shared.
//!
//! Solids enter and leave as [`TriMesh`] triangle soups. Internally we work on
//! convex polygons carrying a supporting plane, split them against partition
//! planes, and recombine. The boolean entry points run the (potentially deep)
//! recursion on a dedicated large-stack thread so big tessellations can't blow
//! the default stack.

use crate::gp::Pnt;
use crate::mesh::TriMesh;

/// Plane classification tolerance. Coarser than `gp::EPS` because it gates the
/// front/back/spanning decision on tessellated, slightly noisy geometry.
const PLANE_EPS: f64 = 1.0e-7;

const COPLANAR: u8 = 0;
const FRONT: u8 = 1;
const BACK: u8 = 2;
const SPANNING: u8 = 3;

#[derive(Clone, Copy)]
struct Plane {
    n: Pnt,
    w: f64,
}

impl Plane {
    fn from_points(a: Pnt, b: Pnt, c: Pnt) -> Option<Plane> {
        let n = (b - a).cross(c - a);
        let len = n.norm();
        if len < 1.0e-12 {
            return None; // degenerate triangle
        }
        let n = n * (1.0 / len);
        Some(Plane { n, w: n.dot(a) })
    }

    fn flip(&mut self) {
        self.n = -self.n;
        self.w = -self.w;
    }
}

#[derive(Clone)]
struct Polygon {
    verts: Vec<Pnt>,
    plane: Plane,
}

impl Polygon {
    fn flip(&mut self) {
        self.verts.reverse();
        self.plane.flip();
    }
}

/// Split `poly` by `plane`, routing the (possibly fragmented) result into the
/// four output buckets.
fn split_polygon(
    plane: &Plane,
    poly: &Polygon,
    coplanar_front: &mut Vec<Polygon>,
    coplanar_back: &mut Vec<Polygon>,
    front: &mut Vec<Polygon>,
    back: &mut Vec<Polygon>,
) {
    let mut poly_type = 0u8;
    let mut types = Vec::with_capacity(poly.verts.len());
    for v in &poly.verts {
        let t = plane.n.dot(*v) - plane.w;
        let ty = if t < -PLANE_EPS {
            BACK
        } else if t > PLANE_EPS {
            FRONT
        } else {
            COPLANAR
        };
        poly_type |= ty;
        types.push(ty);
    }

    match poly_type {
        COPLANAR => {
            if plane.n.dot(poly.plane.n) > 0.0 {
                coplanar_front.push(poly.clone());
            } else {
                coplanar_back.push(poly.clone());
            }
        }
        FRONT => front.push(poly.clone()),
        BACK => back.push(poly.clone()),
        _ => {
            // SPANNING — cut the polygon along `plane`.
            let n = poly.verts.len();
            let mut fv: Vec<Pnt> = Vec::new();
            let mut bv: Vec<Pnt> = Vec::new();
            for i in 0..n {
                let j = (i + 1) % n;
                let ti = types[i];
                let tj = types[j];
                let vi = poly.verts[i];
                let vj = poly.verts[j];
                if ti != BACK {
                    fv.push(vi);
                }
                if ti != FRONT {
                    bv.push(vi);
                }
                if (ti | tj) == SPANNING {
                    let denom = plane.n.dot(vj - vi);
                    if denom.abs() > 1.0e-18 {
                        let t = (plane.w - plane.n.dot(vi)) / denom;
                        let v = vi.lerp(vj, t);
                        fv.push(v);
                        bv.push(v);
                    }
                }
            }
            if fv.len() >= 3 {
                front.push(Polygon {
                    verts: fv,
                    plane: poly.plane,
                });
            }
            if bv.len() >= 3 {
                back.push(Polygon {
                    verts: bv,
                    plane: poly.plane,
                });
            }
        }
    }
}

#[derive(Default)]
struct Node {
    plane: Option<Plane>,
    front: Option<Box<Node>>,
    back: Option<Box<Node>>,
}

/// Pick a partition plane among a few candidate polygons: the one that splits
/// the fewest of a sample (ties broken by balance). Using `polygons[0]` blindly
/// — the textbook choice — fragments curved tessellations (threads, fillets,
/// many-hole plates) quadratically; this keeps trees and fragment counts small.
fn choose_splitter(polygons: &[Polygon]) -> Plane {
    let n = polygons.len();
    if n <= 6 {
        return polygons[0].plane;
    }
    let cands = 7.min(n);
    let samples = 40.min(n);
    let mut best = polygons[0].plane;
    let mut best_score = usize::MAX;
    for c in 0..cands {
        let pl = polygons[c * n / cands].plane;
        let (mut f, mut b, mut sp) = (0usize, 0usize, 0usize);
        for k in 0..samples {
            let poly = &polygons[k * n / samples];
            let mut ty = 0u8;
            for v in &poly.verts {
                let t = pl.n.dot(*v) - pl.w;
                ty |= if t < -PLANE_EPS {
                    BACK
                } else if t > PLANE_EPS {
                    FRONT
                } else {
                    COPLANAR
                };
            }
            match ty {
                FRONT => f += 1,
                BACK => b += 1,
                SPANNING => sp += 1,
                _ => {}
            }
        }
        let score = sp * 6 + f.abs_diff(b);
        if score < best_score {
            best_score = score;
            best = pl;
        }
    }
    best
}

impl Node {
    /// Build a classification tree. Coplanar fragments are absorbed by the
    /// node's plane (classification never needs them back), so the tree holds
    /// planes only — the boundary polygons we output are never fragmented by
    /// their own solid's partition.
    fn build(&mut self, polygons: Vec<Polygon>) {
        if polygons.is_empty() {
            return;
        }
        let plane = *self.plane.get_or_insert_with(|| choose_splitter(&polygons));
        let mut cf = Vec::new();
        let mut cb = Vec::new();
        let mut front = Vec::new();
        let mut back = Vec::new();
        for p in &polygons {
            split_polygon(&plane, p, &mut cf, &mut cb, &mut front, &mut back);
        }
        drop(polygons);
        if !front.is_empty() {
            self.front
                .get_or_insert_with(|| Box::new(Node::default()))
                .build(front);
        }
        if !back.is_empty() {
            self.back
                .get_or_insert_with(|| Box::new(Node::default()))
                .build(back);
        }
    }

    /// Return the subset of `polygons` that lies outside this node's solid.
    fn clip_polygons(&self, polygons: Vec<Polygon>) -> Vec<Polygon> {
        let plane = match self.plane {
            Some(p) => p,
            None => return polygons,
        };
        let mut coplanar_front = Vec::new();
        let mut coplanar_back = Vec::new();
        let mut front = Vec::new();
        let mut back = Vec::new();
        for p in &polygons {
            split_polygon(
                &plane,
                p,
                &mut coplanar_front,
                &mut coplanar_back,
                &mut front,
                &mut back,
            );
        }
        drop(polygons);
        // Coplanar-front fragments lie outside on this plane; coplanar-back are
        // on it but facing inward — route them with front/back respectively.
        front.extend(coplanar_front);
        back.extend(coplanar_back);
        let mut front = match &self.front {
            Some(node) => node.clip_polygons(front),
            None => front,
        };
        let back = match &self.back {
            Some(node) => node.clip_polygons(back),
            None => Vec::new(), // inside the solid -> discard
        };
        front.extend(back);
        front
    }

    fn invert(&mut self) {
        if let Some(plane) = &mut self.plane {
            plane.flip();
        }
        if let Some(f) = &mut self.front {
            f.invert();
        }
        if let Some(b) = &mut self.back {
            b.invert();
        }
        std::mem::swap(&mut self.front, &mut self.back);
    }
}

type Aabb = ([f64; 3], [f64; 3]);

fn poly_aabb(p: &Polygon) -> Aabb {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for v in &p.verts {
        for (k, c) in [v.x, v.y, v.z].into_iter().enumerate() {
            lo[k] = lo[k].min(c);
            hi[k] = hi[k].max(c);
        }
    }
    (lo, hi)
}

/// A solid's classification tree plus its bounds. `inverted` = the tree
/// describes the complement of the solid.
struct Tree {
    root: Node,
    lo: [f64; 3],
    hi: [f64; 3],
    inverted: bool,
}

impl Tree {
    fn new(polygons: &[Polygon]) -> Tree {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for p in polygons {
            let (a, b) = poly_aabb(p);
            for k in 0..3 {
                lo[k] = lo[k].min(a[k]);
                hi[k] = hi[k].max(b[k]);
            }
        }
        let mut root = Node::default();
        root.build(polygons.to_vec());
        Tree { root, lo, hi, inverted: false }
    }

    fn invert(&mut self) {
        self.root.invert();
        self.inverted = !self.inverted;
    }

    /// Clip `polygons` to the outside of this tree's solid. Polygons whose
    /// bounds miss the solid's bounds are outside it by construction, so they
    /// skip the tree entirely (kept, or dropped for a complement tree) instead
    /// of being cut by every partition plane in the tree.
    fn clip(&self, polygons: Vec<Polygon>) -> Vec<Polygon> {
        let ext = (0..3).map(|k| self.hi[k] - self.lo[k]).fold(0.0, f64::max);
        let eps = 1.0e-6 * ext.max(1.0);
        let (mut near, mut far) = (Vec::new(), Vec::new());
        for p in polygons {
            let (a, b) = poly_aabb(&p);
            let hit = (0..3).all(|k| a[k] <= self.hi[k] + eps && b[k] >= self.lo[k] - eps);
            if hit {
                near.push(p);
            } else {
                far.push(p);
            }
        }
        let mut out = self.root.clip_polygons(near);
        if !self.inverted {
            out.extend(far);
        }
        out
    }
}

fn flipped(mut polys: Vec<Polygon>) -> Vec<Polygon> {
    for p in &mut polys {
        p.flip();
    }
    polys
}

fn mesh_to_polygons(m: &TriMesh) -> Vec<Polygon> {
    let mut out = Vec::with_capacity(m.tris.len());
    for t in &m.tris {
        let a = m.verts[t[0]];
        let b = m.verts[t[1]];
        let c = m.verts[t[2]];
        if let Some(plane) = Plane::from_points(a, b, c) {
            out.push(Polygon {
                verts: vec![a, b, c],
                plane,
            });
        }
    }
    out
}

fn polygons_to_mesh(polys: &[Polygon]) -> TriMesh {
    let mut m = TriMesh::new();
    for p in polys {
        m.push_polygon(&p.verts); // fan-triangulate convex fragments
    }
    m
}

/// Run a boolean op on a thread with a generous stack so deep BSP recursion on
/// large tessellations cannot overflow the default (small) stack.
///
/// WebAssembly targets have no threads; there the op runs inline on the
/// caller's stack (wasm engines grow the shadow stack far enough for the
/// tessellation sizes we mesh at).
#[cfg(not(target_family = "wasm"))]
fn run_csg<F>(f: F) -> TriMesh
where
    F: FnOnce() -> TriMesh + Send + 'static,
{
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(f)
        .expect("spawn csg thread")
        .join()
        .expect("csg thread panicked")
}

#[cfg(target_family = "wasm")]
fn run_csg<F>(f: F) -> TriMesh
where
    F: FnOnce() -> TriMesh + Send + 'static,
{
    f()
}

/// A ∪ B (`BRepAlgoAPI_Fuse`).
///
/// Same set algebra as the classic BSP merge (Naylor et al.; csg.js), but on
/// polygon lists against plane-only trees: each solid's boundary is clipped by
/// the other's tree, the second pass with flipped polygons drops the duplicate
/// of coincident same-facing faces, and the results are concatenated instead
/// of being re-split into one merged tree.
pub fn union(a: &TriMesh, b: &TriMesh) -> TriMesh {
    let (pa, pb) = (mesh_to_polygons(a), mesh_to_polygons(b));
    run_csg(move || {
        let ta = Tree::new(&pa);
        let tb = Tree::new(&pb);
        let mut a1 = tb.clip(pa);
        let b1 = ta.clip(pb);
        let b1 = flipped(ta.clip(flipped(b1)));
        a1.extend(b1);
        polygons_to_mesh(&a1)
    })
}

/// A \ B (`BRepAlgoAPI_Cut`).
pub fn subtract(a: &TriMesh, b: &TriMesh) -> TriMesh {
    let (pa, pb) = (mesh_to_polygons(a), mesh_to_polygons(b));
    run_csg(move || {
        let mut ta = Tree::new(&pa);
        let tb = Tree::new(&pb);
        ta.invert();
        let mut a1 = tb.clip(flipped(pa));
        let b1 = ta.clip(pb);
        let b1 = flipped(ta.clip(flipped(b1)));
        a1.extend(b1);
        polygons_to_mesh(&flipped(a1))
    })
}

/// A ∩ B (`BRepAlgoAPI_Common`).
pub fn intersect(a: &TriMesh, b: &TriMesh) -> TriMesh {
    let (pa, pb) = (mesh_to_polygons(a), mesh_to_polygons(b));
    run_csg(move || {
        let mut ta = Tree::new(&pa);
        let mut tb = Tree::new(&pb);
        ta.invert();
        let b1 = ta.clip(pb);
        let b1 = flipped(b1);
        tb.invert();
        let mut a1 = tb.clip(flipped(pa));
        let b1 = ta.clip(b1);
        a1.extend(b1);
        polygons_to_mesh(&flipped(a1))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_prim_api::make_box;

    #[test]
    fn union_of_disjoint_boxes_adds_volume() {
        let a = make_box(Pnt::origin(), 1.0, 1.0, 1.0).mesh().clone();
        let b = make_box(Pnt::new(5.0, 0.0, 0.0), 1.0, 1.0, 1.0)
            .mesh()
            .clone();
        let u = union(&a, &b).welded(1e-6);
        assert!((u.volume() - 2.0).abs() < 1e-6, "vol={}", u.volume());
    }

    #[test]
    fn subtract_overlapping_boxes() {
        // 2x2x2 box minus a 1x2x2 slab -> volume 4. The surface stays
        // volumetrically closed (exact divergence-theorem volume) even though
        // BSP CSG leaves T-junctions where faces coincide; those are immaterial
        // to volume / bbox / voxel IOU.
        let a = make_box(Pnt::origin(), 2.0, 2.0, 2.0).mesh().clone();
        let b = make_box(Pnt::origin(), 1.0, 2.0, 2.0).mesh().clone();
        let d = subtract(&a, &b).welded(1e-6);
        assert!((d.volume() - 4.0).abs() < 1e-6, "vol={}", d.volume());
    }

    #[test]
    fn drill_hole_through_box_volume() {
        // Box 10x10x10 with a cylinder (r=2, tall) drilled through Z. The tool
        // is taller than the box so its caps don't coincide with box faces.
        let a = make_box(Pnt::new(-5.0, -5.0, 0.0), 10.0, 10.0, 10.0)
            .mesh()
            .clone();
        let mut tool = crate::brep_prim_api::make_cylinder(
            2.0,
            20.0,
            crate::brep_prim_api::MeshParams::default(),
        );
        tool.transform(&crate::gp::Trsf::translation(Pnt::new(0.0, 0.0, -5.0)));
        let d = subtract(&a, tool.mesh()).welded(1e-6);
        let expected = 1000.0 - std::f64::consts::PI * 4.0 * 10.0;
        let ratio = d.volume() / expected;
        assert!(
            ratio > 0.99 && ratio < 1.01,
            "vol={} ratio={ratio}",
            d.volume()
        );
    }

    #[test]
    fn intersection_of_overlapping_boxes() {
        // 2x2x2 box ∩ a box shifted by (1,1,1) -> a 1x1x1 overlap, volume 1.
        let a = make_box(Pnt::origin(), 2.0, 2.0, 2.0).mesh().clone();
        let b = make_box(Pnt::new(1.0, 1.0, 1.0), 2.0, 2.0, 2.0)
            .mesh()
            .clone();
        let i = intersect(&a, &b).welded(1e-6);
        assert!((i.volume() - 1.0).abs() < 1e-6, "vol={}", i.volume());
    }
}
