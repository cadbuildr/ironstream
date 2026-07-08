//! Geometry helpers shared by the node handlers: profile/loop resolution,
//! quaternion frames, arc discretization and sketch-local lookups.

use crate::dag::{dep_id, dep_ids, DagNode};
use crate::state::{Curve2, State, P2};
use ironstream::prelude::*;
use std::f64::consts::PI;

/// Collect the ordered sketch curves (lines/arcs) that make up a closed shape.
pub fn gather_profile_curves(state: &State, node: &DagNode) -> Vec<Curve2> {
    let mut ids: Vec<String> = Vec::new();
    for key in ["primitives", "lines", "elements", "curves", "edges"] {
        let got = dep_ids(&node.deps, key);
        if !got.is_empty() {
            ids = got;
            break;
        }
    }
    ids.iter()
        .filter_map(|i| state.curves.get(i).cloned())
        .collect()
}

/// Explicit point-id loop for shapes given as a raw point list.
pub fn explicit_point_loop(node: &DagNode) -> Option<Vec<String>> {
    let pts = dep_ids(&node.deps, "points");
    if pts.is_empty() {
        None
    } else {
        Some(pts)
    }
}

/// Build a `gp::Trsf` for a Frame node from its `position` + `quaternion`
/// params. Quaternion order is `[w, x, y, z]` (foundation convention).
pub fn frame_local_trsf(node: &DagNode) -> Trsf {
    let pos = node
        .params
        .get("position")
        .and_then(|v| v.as_array())
        .map(|a| {
            [
                a.first().and_then(|x| x.as_f64()).unwrap_or(0.0),
                a.get(1).and_then(|x| x.as_f64()).unwrap_or(0.0),
                a.get(2).and_then(|x| x.as_f64()).unwrap_or(0.0),
            ]
        })
        .unwrap_or([0.0, 0.0, 0.0]);
    let q = node
        .params
        .get("quaternion")
        .and_then(|v| v.as_array())
        .map(|a| {
            [
                a.first().and_then(|x| x.as_f64()).unwrap_or(1.0),
                a.get(1).and_then(|x| x.as_f64()).unwrap_or(0.0),
                a.get(2).and_then(|x| x.as_f64()).unwrap_or(0.0),
                a.get(3).and_then(|x| x.as_f64()).unwrap_or(0.0),
            ]
        })
        .unwrap_or([1.0, 0.0, 0.0, 0.0]);
    quat_trsf(q, pos)
}

/// Rotation matrix (from `[w,x,y,z]`) + translation -> `Trsf`.
fn quat_trsf(q: [f64; 4], t: [f64; 3]) -> Trsf {
    let (w, x, y, z) = (q[0], q[1], q[2], q[3]);
    let n = (w * w + x * x + y * y + z * z).sqrt();
    let (w, x, y, z) = if n > 1e-12 {
        (w / n, x / n, y / n, z / n)
    } else {
        (1.0, 0.0, 0.0, 0.0)
    };
    let m = [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ];
    Trsf { m, t }
}

/// The sketch origin of an operation, in sketch-local 2D coordinates.
pub fn sketch_origin_local(state: &State, node: &DagNode) -> P2 {
    dep_id(&node.deps, "sketch")
        .and_then(|s| state.sketches.get(&s))
        .and_then(|si| si.origin_id.as_ref())
        .and_then(|o| state.points2d.get(o).copied())
        .unwrap_or(P2 { x: 0.0, y: 0.0 })
}

/// First resolvable number among a set of candidate dependency keys.
pub fn first_num(state: &State, node: &DagNode, keys: &[&str]) -> Option<f64> {
    for k in keys {
        if let Some(v) = state.resolve_num(&dep_id(&node.deps, k)) {
            return Some(v);
        }
    }
    None
}

/// Arc bulge (signed perpendicular offset of the arc midpoint from the chord),
/// in sketch-local units. Derived from a through/mid point if the node has one;
/// otherwise 0 (the arc degrades to a chord).
///
/// The through point is anywhere ON the arc, not necessarily above the chord
/// midpoint — so the sagitta comes from the circumcircle of (p1, through, p2):
/// the circle crosses the chord's perpendicular bisector at `h ± r` (with `h`
/// the center's signed offset along the chord normal); the arc midpoint is the
/// crossing on the through point's side.
pub fn arc_bulge(state: &State, node: &DagNode, p1: &str, p2: &str, mid_keys: &[&str]) -> f64 {
    let mid_id = mid_keys.iter().find_map(|key| dep_id(&node.deps, key));
    let (a, b) = match (state.points2d.get(p1), state.points2d.get(p2)) {
        (Some(a), Some(b)) => (*a, *b),
        _ => return 0.0,
    };
    let mid = match mid_id.and_then(|m| state.points2d.get(&m).copied()) {
        Some(m) => m,
        None => return 0.0,
    };
    let chord_mid = P2 {
        x: (a.x + b.x) * 0.5,
        y: (a.y + b.y) * 0.5,
    };
    let dir = P2 {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let len = (dir.x * dir.x + dir.y * dir.y).sqrt();
    if len < 1e-9 {
        return 0.0;
    }
    // Perpendicular unit (rotate chord dir +90°).
    let perp = P2 {
        x: -dir.y / len,
        y: dir.x / len,
    };
    // Circumcenter of (a, mid, b); collinear degrades to a chord.
    let d = 2.0 * (a.x * (mid.y - b.y) + mid.x * (b.y - a.y) + b.x * (a.y - mid.y));
    if d.abs() < 1e-12 {
        return 0.0;
    }
    let a2 = a.x * a.x + a.y * a.y;
    let m2 = mid.x * mid.x + mid.y * mid.y;
    let b2 = b.x * b.x + b.y * b.y;
    let cx = (a2 * (mid.y - b.y) + m2 * (b.y - a.y) + b2 * (a.y - mid.y)) / d;
    let cy = (a2 * (b.x - mid.x) + m2 * (a.x - b.x) + b2 * (mid.x - a.x)) / d;
    let r = ((a.x - cx).powi(2) + (a.y - cy).powi(2)).sqrt();
    let h = (cx - chord_mid.x) * perp.x + (cy - chord_mid.y) * perp.y;
    let side = (mid.x - chord_mid.x) * perp.x + (mid.y - chord_mid.y) * perp.y;
    if side >= 0.0 {
        h + r
    } else {
        h - r
    }
}

/// Resolve a closed shape (Polygon/CustomClosedShape/Circle) into a world-space
/// loop of points, using the sketch `plane` transform.
pub fn resolve_shape_loop(state: &State, shape_id: &str, plane: &Trsf) -> Option<Vec<Pnt>> {
    if let Some(profile) = state.profiles.get(shape_id) {
        let local = profile_local_loop(state, profile);
        if local.len() >= 3 {
            return Some(
                local
                    .iter()
                    .map(|p| plane.apply_point(Pnt::new(p.x, p.y, 0.0)))
                    .collect(),
            );
        }
        return None;
    }
    if let Some((center_id, radius_id)) = state.circles.get(shape_id) {
        let c = center_id
            .as_ref()
            .and_then(|c| state.points2d.get(c).copied())
            .unwrap_or(P2 { x: 0.0, y: 0.0 });
        let r = radius_id
            .as_ref()
            .and_then(|r| state.numbers.get(r).copied())
            .unwrap_or(0.0);
        if r <= 0.0 {
            return None;
        }
        let n = 64usize;
        let loop_pts = (0..n)
            .map(|i| {
                let a = 2.0 * PI * i as f64 / n as f64;
                plane.apply_point(Pnt::new(c.x + r * a.cos(), c.y + r * a.sin(), 0.0))
            })
            .collect();
        return Some(loop_pts);
    }
    if let Some((center_id, a_id, b_id)) = state.ellipses.get(shape_id) {
        let cx = center_id
            .as_ref()
            .and_then(|c| state.points2d.get(c))
            .map(|p| p.x)
            .unwrap_or(0.0);
        let cy = center_id
            .as_ref()
            .and_then(|c| state.points2d.get(c))
            .map(|p| p.y)
            .unwrap_or(0.0);
        let a = a_id
            .as_ref()
            .and_then(|a| state.numbers.get(a).copied())
            .unwrap_or(1.0);
        let b = b_id
            .as_ref()
            .and_then(|b| state.numbers.get(b).copied())
            .unwrap_or(1.0);
        if a <= 0.0 || b <= 0.0 {
            return None;
        }
        let n = 64usize;
        let loop_pts = (0..n)
            .map(|i| {
                let ang = 2.0 * PI * i as f64 / n as f64;
                plane.apply_point(Pnt::new(cx + a * ang.cos(), cy + b * ang.sin(), 0.0))
            })
            .collect();
        return Some(loop_pts);
    }
    None
}

/// Build the sketch-local 2D loop for a profile (lines + discretized arcs).
fn profile_local_loop(state: &State, profile: &crate::state::Profile) -> Vec<P2> {
    if let Some(ids) = &profile.explicit_points {
        return ids
            .iter()
            .filter_map(|i| state.points2d.get(i).copied())
            .collect();
    }
    let mut out: Vec<P2> = Vec::new();
    for c in &profile.curves {
        match c {
            Curve2::Line { p1, .. } => {
                if let Some(p) = state.points2d.get(p1) {
                    out.push(*p);
                }
            }
            Curve2::Arc { p1, p2, bulge } => {
                if let (Some(a), Some(b)) = (state.points2d.get(p1), state.points2d.get(p2)) {
                    out.push(*a);
                    if bulge.abs() > 1e-9 {
                        out.extend(discretize_arc(*a, *b, *bulge));
                    }
                }
            }
        }
    }
    out
}

/// Discretize an arc from `a` to `b` whose midpoint is offset perpendicular to
/// the chord by `bulge`. Returns the interior points (endpoints excluded).
fn discretize_arc(a: P2, b: P2, bulge: f64) -> Vec<P2> {
    let chord_mid = P2 {
        x: (a.x + b.x) * 0.5,
        y: (a.y + b.y) * 0.5,
    };
    let dir = P2 {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let chord_len = (dir.x * dir.x + dir.y * dir.y).sqrt();
    if chord_len < 1e-9 {
        return Vec::new();
    }
    let perp = P2 {
        x: -dir.y / chord_len,
        y: dir.x / chord_len,
    };
    let apex = P2 {
        x: chord_mid.x + perp.x * bulge,
        y: chord_mid.y + perp.y * bulge,
    };
    // Circle through a, apex, b.
    let (cx, cy, r) = match circle_through(a, apex, b) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let ang = |p: P2| (p.y - cy).atan2(p.x - cx);
    let a0 = ang(a);
    let a1 = ang(b);
    let amid = ang(apex);
    // Sweep from a to b in the direction that passes through the apex. A major
    // arc legitimately sweeps more than PI — never fold the sweep back into
    // (-PI, PI], that replaces it with its minor complement.
    let ccw = |from: f64, to: f64| {
        let mut d = to - from;
        while d < 0.0 {
            d += 2.0 * PI;
        }
        d
    };
    let sweep_ccw = ccw(a0, a1);
    let sweep = if ccw(a0, amid) <= sweep_ccw {
        sweep_ccw
    } else {
        sweep_ccw - 2.0 * PI
    };
    let start = a0;
    let steps = ((r * sweep.abs()).ceil() as usize).clamp(4, 64);
    (1..steps)
        .map(|i| {
            let t = start + sweep * i as f64 / steps as f64;
            P2 {
                x: cx + r * t.cos(),
                y: cy + r * t.sin(),
            }
        })
        .collect()
}

fn circle_through(a: P2, b: P2, c: P2) -> Option<(f64, f64, f64)> {
    let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
    if d.abs() < 1e-12 {
        return None;
    }
    let a2 = a.x * a.x + a.y * a.y;
    let b2 = b.x * b.x + b.y * b.y;
    let c2 = c.x * c.x + c.y * c.y;
    let ux = (a2 * (b.y - c.y) + b2 * (c.y - a.y) + c2 * (a.y - b.y)) / d;
    let uy = (a2 * (c.x - b.x) + b2 * (a.x - c.x) + c2 * (b.x - a.x)) / d;
    let r = ((a.x - ux).powi(2) + (a.y - uy).powi(2)).sqrt();
    Some((ux, uy, r))
}

/// Clip a closed profile loop so it only contains the half-space on one side
/// of the revolution axis. This converts a full closed profile (e.g. an ellipse)
/// into an open arc suitable for `make_revol`.
///
/// For each world-space point we compute the signed radial distance from the
/// axis: we project the point onto a consistent reference half-plane defined by
/// the axis and a chosen perpendicular. Points on the negative side are clipped
/// and intersection points are inserted where the profile crosses the axis.
pub fn clip_profile_to_axis(pts: &[Pnt], axis: &Ax1) -> Vec<Pnt> {
    if pts.len() < 2 {
        return pts.to_vec();
    }
    let dir = axis.direction.normalized();

    // Build a perpendicular reference direction (the "0°" radial direction).
    // We use the global Y-axis unless it is nearly parallel to the axis.
    let ref_perp = {
        let aux = if dir.y.abs() < 0.9 {
            Pnt::new(0.0, 1.0, 0.0)
        } else {
            Pnt::new(0.0, 0.0, 1.0)
        };
        let v = aux - dir * aux.dot(dir);
        if v.norm() > 1e-9 { v.normalized() } else { aux }
    };

    // Signed distance = component of the radial vector along ref_perp.
    // Positive means the point is on the "ref_perp" side of the axis.
    let signed_dist = |p: Pnt| -> f64 {
        let rel = p - axis.location;
        let along = dir * rel.dot(dir);
        let radial = rel - along;
        radial.dot(ref_perp)
    };

    let dists: Vec<f64> = pts.iter().map(|p| signed_dist(*p)).collect();
    let max_d = dists.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_d = dists.iter().cloned().fold(f64::INFINITY, f64::min);

    // If all points are already on one side (profile is naturally an open half),
    // just return it unchanged.
    if min_d >= -1e-6 || max_d <= 1e-6 {
        return pts.to_vec();
    }

    // Mixed: clip to the positive half.
    let n = pts.len();
    let mut out: Vec<Pnt> = Vec::new();
    for i in 0..n {
        let j = (i + 1) % n;
        let di = dists[i];
        let dj = dists[j];
        let pi = pts[i];
        let pj = pts[j];
        if di >= -1e-9 {
            out.push(pi);
        }
        // Insert intersection point where the edge crosses the axis plane.
        if (di < -1e-9 && dj > 1e-9) || (di > 1e-9 && dj < -1e-9) {
            let t = di / (di - dj);
            let ip = Pnt::new(
                pi.x + t * (pj.x - pi.x),
                pi.y + t * (pj.y - pi.y),
                pi.z + t * (pj.z - pi.z),
            );
            out.push(ip);
        }
    }
    if out.len() < 3 {
        return pts.to_vec();
    }
    out
}

