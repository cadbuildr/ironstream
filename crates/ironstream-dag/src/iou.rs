//! IOU (intersection-over-union) metrics for comparing IronStream output to a
//! reference.
//!
//! Two flavours:
//!  * **bbox IOU** — exactly the metric the CADbuildr parity harness computes
//!    from STL bounding boxes (`parity_metric.ts`).
//!  * **voxel solid IOU** — a stronger, from-scratch volumetric IOU: sample a
//!    grid over the union bounding box and classify each sample as inside/out
//!    of each solid via ray-cast point-in-mesh. This is a true 3D IOU, not just
//!    a box overlap.

use ironstream::prelude::*;

/// A bundle of comparison metrics.
#[derive(Clone, Copy, Debug)]
pub struct IouReport {
    pub bbox_iou: f64,
    pub volume_ratio: f64,
    pub voxel_iou: f64,
}

/// Ratio of two scalars in `[0, 1]` (1 when equal, symmetric).
pub fn ratio(a: f64, b: f64) -> f64 {
    if a == 0.0 && b == 0.0 {
        return 1.0;
    }
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    lo / hi
}

/// Möller–Trumbore ray/triangle test for a ray from `o` in direction `d`.
/// Returns the positive ray parameter `t` of the hit, if any.
fn ray_tri(o: Pnt, d: Pnt, a: Pnt, b: Pnt, c: Pnt) -> Option<f64> {
    let eps = 1e-12;
    let e1 = b - a;
    let e2 = c - a;
    let p = d.cross(e2);
    let det = e1.dot(p);
    if det.abs() < eps {
        return None;
    }
    let inv = 1.0 / det;
    let tvec = o - a;
    let u = tvec.dot(p) * inv;
    if u < 0.0 || u > 1.0 {
        return None;
    }
    let q = tvec.cross(e1);
    let v = d.dot(q) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = e2.dot(q) * inv;
    if t > eps {
        Some(t)
    } else {
        None
    }
}

/// Point-in-solid test by parity of ray crossings. Uses a slightly skewed ray
/// direction so it (almost surely) avoids hitting shared edges/vertices.
pub fn point_in_mesh(mesh: &TriMesh, pt: Pnt) -> bool {
    let dir = Pnt::new(1.0, 0.013_711, 0.007_317).normalized();
    let mut crossings = 0usize;
    for t in &mesh.tris {
        if ray_tri(
            pt,
            dir,
            mesh.verts[t[0]],
            mesh.verts[t[1]],
            mesh.verts[t[2]],
        )
        .is_some()
        {
            crossings += 1;
        }
    }
    crossings % 2 == 1
}

fn union_bbox(a: &BBox, b: &BBox) -> BBox {
    let mut out = BBox::empty();
    if !a.is_empty() {
        out.add(Pnt::from_array(a.min));
        out.add(Pnt::from_array(a.max));
    }
    if !b.is_empty() {
        out.add(Pnt::from_array(b.min));
        out.add(Pnt::from_array(b.max));
    }
    out
}

/// Voxel IOU between two meshes at roughly `res` samples per axis.
pub fn voxel_iou_meshes(a: &TriMesh, b: &TriMesh, res: usize) -> f64 {
    let bb = union_bbox(&a.bbox(), &b.bbox());
    if bb.is_empty() {
        return 1.0;
    }
    sample_iou(res, &bb, |p| point_in_mesh(a, p), |p| point_in_mesh(b, p))
}

/// Voxel IOU between a mesh and an analytic reference predicate.
pub fn voxel_iou_mesh_pred<F>(mesh: &TriMesh, ref_bbox: &BBox, res: usize, pred: F) -> f64
where
    F: Fn(Pnt) -> bool,
{
    let bb = union_bbox(&mesh.bbox(), ref_bbox);
    if bb.is_empty() {
        return 1.0;
    }
    sample_iou(res, &bb, |p| point_in_mesh(mesh, p), pred)
}

fn sample_iou<F, G>(res: usize, bb: &BBox, in_a: F, in_b: G) -> f64
where
    F: Fn(Pnt) -> bool,
    G: Fn(Pnt) -> bool,
{
    let res = res.max(4);
    // Pad slightly so boundary samples land cleanly inside/outside.
    let pad = [
        (bb.max[0] - bb.min[0]).max(1e-6) * 0.02,
        (bb.max[1] - bb.min[1]).max(1e-6) * 0.02,
        (bb.max[2] - bb.min[2]).max(1e-6) * 0.02,
    ];
    let lo = [bb.min[0] - pad[0], bb.min[1] - pad[1], bb.min[2] - pad[2]];
    let hi = [bb.max[0] + pad[0], bb.max[1] + pad[1], bb.max[2] + pad[2]];
    let step = [
        (hi[0] - lo[0]) / res as f64,
        (hi[1] - lo[1]) / res as f64,
        (hi[2] - lo[2]) / res as f64,
    ];
    let mut inter = 0u64;
    let mut uni = 0u64;
    for i in 0..res {
        for j in 0..res {
            for k in 0..res {
                // Sample cell centers.
                let p = Pnt::new(
                    lo[0] + (i as f64 + 0.5) * step[0],
                    lo[1] + (j as f64 + 0.5) * step[1],
                    lo[2] + (k as f64 + 0.5) * step[2],
                );
                let a = in_a(p);
                let b = in_b(p);
                if a && b {
                    inter += 1;
                }
                if a || b {
                    uni += 1;
                }
            }
        }
    }
    if uni == 0 {
        1.0
    } else {
        inter as f64 / uni as f64
    }
}

/// Full report comparing a mesh to a reference mesh.
pub fn compare_meshes(out: &TriMesh, reference: &TriMesh, res: usize) -> IouReport {
    IouReport {
        bbox_iou: out.bbox().iou(&reference.bbox()),
        volume_ratio: ratio(out.volume(), reference.volume()),
        voxel_iou: voxel_iou_meshes(out, reference, res),
    }
}
