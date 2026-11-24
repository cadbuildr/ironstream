//! Node handlers — the node-by-node DAG processor.
//!
//! This mirrors the dispatch structure of the replicad and truck kernel
//! adapters: walk the DAG in dependency order and, for each node, run a handler
//! keyed by the node's **type name** (resolved per-DAG via `serializableNodes`).
//! Handlers resolve their dependencies from [`State`] and write their result
//! back under the node id.

use crate::dag::{dep_id, dep_ids, CompilerInputDag, DagNode};
use crate::state::{Curve2, Helix, OpResult, PartResult, Profile, SketchInfo, SmBody, State, P2};
use ironstream::prelude::*;
use std::f64::consts::PI;

mod geom_helpers;
use geom_helpers::*;

/// Dispatch a single node to its handler. Unknown node types are recorded for
/// coverage reporting but never abort the compile.
pub fn dispatch(state: &mut State, type_name: &str, id: &str, node: &DagNode) {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle(state, type_name, id, node);
    }));
    if r.is_err() {
        state.error(format!("{type_name}#{id}: panicked during processing"));
    }
}

fn handle(state: &mut State, type_name: &str, id: &str, node: &DagNode) {
    match type_name {
        // ---- parameters -------------------------------------------------
        "FloatParameter" | "IntParameter" => {
            if let Some(v) = node.params.get("value").and_then(|v| v.as_f64()) {
                state.numbers.insert(id.to_string(), v);
            }
        }
        "BoolParameter" => {
            if let Some(v) = node.params.get("value").and_then(|v| v.as_bool()) {
                state.bools.insert(id.to_string(), v);
            }
        }
        "StringParameter" => {
            if let Some(v) = node.params.get("value").and_then(|v| v.as_str()) {
                state.strings.insert(id.to_string(), v.to_string());
            }
        }

        // ---- 2D geometry -------------------------------------------------
        "Point" => {
            let x = state.resolve_num(&dep_id(&node.deps, "x")).unwrap_or(0.0);
            let y = state.resolve_num(&dep_id(&node.deps, "y")).unwrap_or(0.0);
            state.points2d.insert(id.to_string(), P2 { x, y });
        }
        "Point3D" => {
            let x = state.resolve_num(&dep_id(&node.deps, "x")).unwrap_or(0.0);
            let y = state.resolve_num(&dep_id(&node.deps, "y")).unwrap_or(0.0);
            let z = state.resolve_num(&dep_id(&node.deps, "z")).unwrap_or(0.0);
            state.points3d.insert(id.to_string(), Pnt::new(x, y, z));
        }
        "Line" => {
            if let (Some(p1), Some(p2)) = (dep_id(&node.deps, "p1"), dep_id(&node.deps, "p2")) {
                state.curves.insert(id.to_string(), Curve2::Line { p1, p2 });
            }
        }
        "Arc" | "EllipseArc" => {
            // Foundation Arc semantics (schema + reference kernel): p1 = start,
            // p2 = THROUGH point on the arc, p3 = end — so the chord is p1->p3
            // and the bulge comes from p2. Nodes without a p3 degrade to the
            // legacy p1->p2 chord with an optional "mid"/"through" dep; with no
            // through point at all the arc degrades to a chord (bulge 0).
            if let (Some(p1), Some(p3)) = (dep_id(&node.deps, "p1"), dep_id(&node.deps, "p3")) {
                let bulge = arc_bulge(state, node, &p1, &p3, &["p2", "mid", "through"]);
                state
                    .curves
                    .insert(id.to_string(), Curve2::Arc { p1, p2: p3, bulge });
            } else if let (Some(p1), Some(p2)) = (dep_id(&node.deps, "p1"), dep_id(&node.deps, "p2")) {
                let bulge = arc_bulge(state, node, &p1, &p2, &["mid", "through"]);
                state
                    .curves
                    .insert(id.to_string(), Curve2::Arc { p1, p2, bulge });
            }
        }
        "Circle" => {
            let center = dep_id(&node.deps, "center");
            let radius = dep_id(&node.deps, "radius");
            state.circles.insert(id.to_string(), (center, radius));
        }
        "Ellipse" => {
            let center = dep_id(&node.deps, "center");
            let a = dep_id(&node.deps, "a");
            let b = dep_id(&node.deps, "b");
            state.ellipses.insert(id.to_string(), (center, a, b));
        }
        "Helix3D" => op_helix3d(state, id, node),
        // assembly metadata: placement is already chained through frames
        "Anchor" | "RigidJoint" | "RevoluteJoint" | "SliderJoint" | "JointLimits"
        | "Connection" | "BallJoint" | "CylindricalJoint" | "PinSlotJoint" | "PlanarJoint"
        | "ScrewJoint" => {}
        "Spline3D" => {
            let pts: Vec<Pnt> = dep_ids(&node.deps, "points")
                .iter()
                .filter_map(|pid| state.points3d.get(pid).copied())
                .collect();
            state.spline3ds.insert(id.to_string(), pts);
        }
        "Polygon" | "CustomClosedShape" | "Rectangle" | "Square" | "Hexagon" | "RegularPolygon"
        | "Triangle" | "Trapezoid" | "CustomOpenShape" => {
            let curves = gather_profile_curves(state, node);
            state.profiles.insert(
                id.to_string(),
                Profile {
                    curves,
                    explicit_points: explicit_point_loop(node),
                },
            );
        }

        // ---- construction geometry --------------------------------------
        "Frame" => {
            let local = frame_local_trsf(node);
            let parent = dep_id(&node.deps, "top_frame")
                .and_then(|p| state.frames.get(&p).copied())
                .unwrap_or_else(Trsf::identity);
            state.frames.insert(id.to_string(), local.then(&parent));
        }
        "Plane" => {
            let t = dep_id(&node.deps, "frame")
                .and_then(|f| state.frames.get(&f).copied())
                .unwrap_or_else(Trsf::identity);
            state.planes.insert(id.to_string(), t);
        }
        "Sketch" => {
            state.sketches.insert(
                id.to_string(),
                SketchInfo {
                    plane_id: dep_id(&node.deps, "plane"),
                    origin_id: dep_id(&node.deps, "origin"),
                },
            );
        }
        "Axis" => {
            // Axis can be given as a Line dep, or directly via p1/p2 points.
            if let Some(line_id) = dep_id(&node.deps, "line") {
                if let Some(Curve2::Line { p1, p2 }) = state.curves.get(&line_id).cloned() {
                    state.axes.insert(id.to_string(), (p1, p2));
                }
            } else if let (Some(p1), Some(p2)) =
                (dep_id(&node.deps, "p1"), dep_id(&node.deps, "p2"))
            {
                state.axes.insert(id.to_string(), (p1, p2));
            }
        }

        // ---- 3D operations ----------------------------------------------
        "Extrusion" => op_extrusion(state, id, node),
        "Hole" => op_hole(state, id, node),
        "Lathe" => op_lathe(state, id, node),
        "Box" => op_box(state, id, node),
        "Sphere" => op_sphere(state, id, node),
        "Cylinder" => op_cylinder(state, id, node),
        "Cone" => op_cone(state, id, node),
        "Torus" => op_torus(state, id, node),
        "Mirror" => op_mirror(state, id, node),
        "Scale" => op_scale(state, id, node),
        "Loft" | "SurfaceLoft" | "MultiSectionSweep" => op_loft(state, id, node),
        "Sweep" => op_sweep(state, id, node),
        "Shell" => op_shell(state, id, node),
        "Stitch" => op_stitch(state, id, node),
        "Thicken" => op_thicken(state, id, node),
        "SheetMetalBaseFlange" => op_sm_base_flange(state, id, node),
        "SheetMetalToSolid" => op_sm_to_solid(state, id, node),
        "SheetMetalEdgeFlange" => op_sm_edge_flange(state, id, node),
        "SheetMetalBend"
        | "SheetMetalContourFlange"
        | "SheetMetalCornerSeam"
        | "SheetMetalHem"
        | "SheetMetalMiterFlange"
        | "SheetMetalSketchedBend"
        | "SheetMetalTab"
        | "Unfold" => op_sm_passthrough(state, id, node),
        // no-op construction nodes that produce no geometry
        "EdgeFinder"
        | "InPlaneFinderRule"
        | "FixedTranslationConstraint"
        | "AssemblyInterface"
        | "InterfaceGridSpec" => {}

        // ---- assembly nodes ---------------------------------------------
        "PartRoot" | "Part" => finalize_part(state, id, node),
        "AssemblyRoot" | "Assembly" => { /* components already registered as parts */ }

        // ---- pure metadata / not geometry-bearing -----------------------
        "Material" | "SketchOrigin" | "Color" | "BoundingBox" => {}

        other => {
            if !state.unsupported.iter().any(|u| u == other) {
                state.unsupported.push(other.to_string());
            }
        }
    }
}

// ===================================================================
// Operations
// ===================================================================

/// Resolve the sketch plane transform for an operation (identity if absent).
fn op_plane(state: &State, node: &DagNode) -> Trsf {
    dep_id(&node.deps, "sketch")
        .and_then(|s| state.sketches.get(&s))
        .and_then(|si| si.plane_id.as_ref())
        .and_then(|p| state.planes.get(p).copied())
        .unwrap_or_else(Trsf::identity)
}

fn op_extrusion(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let normal = plane.apply_vector(Pnt::new(0.0, 0.0, 1.0)).normalized();
    let start = state
        .resolve_num(&dep_id(&node.deps, "start"))
        .unwrap_or(0.0);
    let end = state.resolve_num(&dep_id(&node.deps, "end")).unwrap_or(0.0);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));

    let mut shape_ids = dep_ids(&node.deps, "shape");
    shape_ids.extend(dep_ids(&node.deps, "shapes"));
    if shape_ids.is_empty() {
        state.error(format!("Extrusion#{id}: no shape"));
        return;
    }
    let taper = state.resolve_num(&dep_id(&node.deps, "taper")).unwrap_or(0.0);
    let offset = normal * start;
    let vec = normal * (end - start);
    // shape[0] is the outer loop, the rest are holes (foundation / OCCT).
    let mut loops: Vec<Vec<Pnt>> = Vec::new();
    for sid in &shape_ids {
        match resolve_shape_loop(state, sid, &plane) {
            Some(loop_pts) if loop_pts.len() >= 3 => {
                loops.push(loop_pts.iter().map(|p| *p + offset).collect());
            }
            _ => state.error(format!("Extrusion#{id}: shape {sid} unresolved")),
        }
    }
    if loops.is_empty() {
        return;
    }
    let solid = if taper != 0.0 && loops.len() == 1 {
        // draft: the profile scales about its centroid from 1 at the base to
        // 1 - taper at the top (replicad's linear extrusion profile)
        let base = &loops[0];
        let c = base.iter().fold(Pnt::origin(), |a, p| a + *p) * (1.0 / base.len() as f64);
        let f = 1.0 - taper;
        let top: Vec<Pnt> = base.iter().map(|p| c + (*p - c) * f + vec).collect();
        loft_sections(&[base.clone(), top])
    } else {
        let outer = make_polygon(&loops[0]);
        let holes: Vec<_> = loops[1..].iter().map(|l| make_polygon(l)).collect();
        let face = if holes.is_empty() { make_face(outer) } else { Face::with_holes(outer, holes) };
        make_prism(&face, vec)
    };
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut });
}

/// `Hole` — a subtractive feature. Handled as a cut prism/cylinder when it
/// carries a shape+sketch, otherwise recorded as unsupported.
fn op_hole(state: &mut State, id: &str, node: &DagNode) {
    let has_shape =
        !dep_ids(&node.deps, "shape").is_empty() || !dep_ids(&node.deps, "shapes").is_empty();
    if has_shape {
        op_extrusion(state, id, node);
        if let Some(op) = state.operations.get_mut(id) {
            op.cut = true; // a hole always subtracts
        }
    } else if !state.unsupported.iter().any(|u| u == "Hole") {
        state.unsupported.push("Hole".to_string());
    }
}

fn op_lathe(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let shape_id = match dep_id(&node.deps, "shape") {
        Some(s) => s,
        None => {
            state.error(format!("Lathe#{id}: no shape"));
            return;
        }
    };
    let loop_pts = match resolve_shape_loop(state, &shape_id, &plane) {
        Some(p) if p.len() >= 3 => p,
        _ => {
            state.error(format!("Lathe#{id}: shape unresolved"));
            return;
        }
    };
    let axis = match dep_id(&node.deps, "axis").and_then(|a| state.axes.get(&a).cloned()) {
        Some((p1, p2)) => {
            let a = state.world_point(&p1, &plane);
            let b = state.world_point(&p2, &plane);
            match (a, b) {
                (Some(a), Some(b)) => Ax1::new(a, b - a),
                _ => {
                    state.error(format!("Lathe#{id}: axis points unresolved"));
                    return;
                }
            }
        }
        None => {
            state.error(format!("Lathe#{id}: no axis"));
            return;
        }
    };
    // Clip the profile to one half-plane relative to the revolution axis so that
    // a closed shape (e.g. a full ellipse) is automatically trimmed to the half
    // that produces a valid solid when revolved.
    let profile_pts = clip_profile_to_axis(&loop_pts, &axis);
    let wire = make_polygon(&profile_pts);
    let solid = make_revol(&wire, axis, 2.0 * PI, state.mesh_params);
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut });
}

fn op_box(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let w = state
        .resolve_num(&dep_id(&node.deps, "w"))
        .unwrap_or(0.0)
        .abs();
    let h = state
        .resolve_num(&dep_id(&node.deps, "h"))
        .unwrap_or(0.0)
        .abs();
    let d = state
        .resolve_num(&dep_id(&node.deps, "d"))
        .unwrap_or(0.0)
        .abs();
    let center = sketch_origin_local(state, node);
    // Local box centered in X/Y on the sketch origin, growing +Z.
    let local = make_box_centered_xy(Pnt::new(center.x, center.y, 0.0), w, h, d);
    let solid = transform(&local, &plane);
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut });
}

fn op_sphere(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let r = state
        .resolve_num(&dep_id(&node.deps, "radius"))
        .unwrap_or(0.0);
    let center_world = dep_id(&node.deps, "center")
        .and_then(|c| state.world_point(&c, &plane))
        .unwrap_or_else(|| plane.apply_point(Pnt::origin()));
    let mut solid = make_sphere(r, state.mesh_params);
    solid.transform(&Trsf::translation(center_world));
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut });
}

fn op_cylinder(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let r = first_num(state, node, &["radius", "r"]).unwrap_or(0.0);
    let h = first_num(state, node, &["height", "h", "depth"]).unwrap_or(0.0);
    let center = sketch_origin_local(state, node);
    let local = make_cylinder(r, h, state.mesh_params);
    let placed = transform(
        &local,
        &Trsf::translation(Pnt::new(center.x, center.y, 0.0)).then(&plane),
    );
    state
        .operations
        .insert(id.to_string(), OpResult { solid: placed, cut });
}

fn op_cone(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let r1 = first_num(state, node, &["radius", "radius1", "r1", "bottom_radius"]).unwrap_or(0.0);
    let r2 = first_num(state, node, &["radius2", "r2", "top_radius"]).unwrap_or(0.0);
    let h = first_num(state, node, &["height", "h"]).unwrap_or(0.0);
    let center = sketch_origin_local(state, node);
    let local = make_cone(r1, r2, h, state.mesh_params);
    let placed = transform(
        &local,
        &Trsf::translation(Pnt::new(center.x, center.y, 0.0)).then(&plane),
    );
    state
        .operations
        .insert(id.to_string(), OpResult { solid: placed, cut });
}

fn op_torus(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let major = first_num(state, node, &["major_radius", "radius1", "major", "r1"]).unwrap_or(0.0);
    let minor = first_num(state, node, &["minor_radius", "radius2", "minor", "r2"]).unwrap_or(0.0);
    let center = sketch_origin_local(state, node);
    let local = make_torus(major, minor, state.mesh_params);
    let placed = transform(
        &local,
        &Trsf::translation(Pnt::new(center.x, center.y, 0.0)).then(&plane),
    );
    state
        .operations
        .insert(id.to_string(), OpResult { solid: placed, cut });
}

fn op_mirror(state: &mut State, id: &str, node: &DagNode) {
    // Mirror a referenced operation across the YZ plane of its sketch (default
    // X mirror). Adds the mirrored copy as a new additive operation.
    let src = dep_id(&node.deps, "operation")
        .or_else(|| dep_id(&node.deps, "shape"))
        .and_then(|s| state.operations.get(&s).cloned());
    if let Some(op) = src {
        let m = Trsf::scale_xyz(-1.0, 1.0, 1.0);
        let solid = transform(&op.solid, &m);
        state
            .operations
            .insert(id.to_string(), OpResult { solid, cut: op.cut });
    } else if !state.unsupported.iter().any(|u| u == "Mirror") {
        state.unsupported.push("Mirror".to_string());
    }
}

fn op_scale(state: &mut State, id: &str, node: &DagNode) {
    let f = first_num(state, node, &["factor", "scale", "value"]).unwrap_or(1.0);
    let src = dep_id(&node.deps, "operation")
        .or_else(|| dep_id(&node.deps, "shape"))
        .and_then(|s| state.operations.get(&s).cloned());
    if let Some(op) = src {
        let solid = transform(&op.solid, &Trsf::scale_uniform(f));
        state
            .operations
            .insert(id.to_string(), OpResult { solid, cut: op.cut });
    } else if !state.unsupported.iter().any(|u| u == "Scale") {
        state.unsupported.push("Scale".to_string());
    }
}

// ===================================================================
// Loft / Sweep / Shell helpers
// ===================================================================

/// Resolve the sketch plane for a section in a loft/sweep — first try it
/// directly as a plane, then as a sketch whose `plane_id` points to a plane.
fn resolve_section_plane(state: &State, sketch_id: &str) -> Trsf {
    if let Some(t) = state.planes.get(sketch_id) {
        return *t;
    }
    if let Some(si) = state.sketches.get(sketch_id) {
        if let Some(pid) = &si.plane_id {
            if let Some(t) = state.planes.get(pid) {
                return *t;
            }
        }
    }
    Trsf::identity()
}

/// Resample a closed polygon loop to exactly `n` evenly-spaced points by arc
/// length. Used to ensure matching vertex counts across loft sections.
fn resample_loop(pts: &[Pnt], n: usize) -> Vec<Pnt> {
    if pts.is_empty() || n == 0 {
        return Vec::new();
    }
    let m = pts.len();
    let mut lengths = vec![0.0f64; m + 1];
    for i in 0..m {
        lengths[i + 1] = lengths[i] + pts[i].distance(pts[(i + 1) % m]);
    }
    let total = lengths[m];
    if total < 1e-12 {
        return vec![pts[0]; n];
    }
    let mut out = Vec::with_capacity(n);
    let mut seg = 0usize;
    for k in 0..n {
        let t = total * k as f64 / n as f64;
        while seg + 1 < m && lengths[seg + 1] < t {
            seg += 1;
        }
        let seg_start = lengths[seg];
        let seg_end = lengths[seg + 1];
        let seg_len = seg_end - seg_start;
        let frac = if seg_len > 1e-12 {
            (t - seg_start) / seg_len
        } else {
            0.0
        };
        let a = pts[seg];
        let b = pts[(seg + 1) % m];
        out.push(Pnt::new(
            a.x + frac * (b.x - a.x),
            a.y + frac * (b.y - a.y),
            a.z + frac * (b.z - a.z),
        ));
    }
    out
}

/// Build a solid by linearly skinning between a sequence of closed-loop sections.
/// Newell normal of a closed 3D loop (unnormalised).
fn newell(pts: &[Pnt]) -> Pnt {
    let mut n = Pnt::origin();
    let m = pts.len();
    for i in 0..m {
        let (a, b) = (pts[i], pts[(i + 1) % m]);
        n = n + Pnt::new(
            (a.y - b.y) * (a.z + b.z),
            (a.z - b.z) * (a.x + b.x),
            (a.x - b.x) * (a.y + b.y),
        );
    }
    n
}

/// Grow a closed loop to `n` points by splitting its longest edge; the
/// original vertices (a polygon's corners) are kept.
fn upsample_loop(pts: &[Pnt], n: usize) -> Vec<Pnt> {
    let mut v = pts.to_vec();
    while v.len() < n {
        let m = v.len();
        let k = (0..m)
            .max_by(|&i, &j| {
                let di = v[i].distance(v[(i + 1) % m]);
                let dj = v[j].distance(v[(j + 1) % m]);
                di.partial_cmp(&dj).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0);
        let mid = (v[k] + v[(k + 1) % m]) * 0.5;
        v.insert(k + 1, mid);
    }
    v
}

/// Skin a closed solid through section loops (loft / sweep). Sections keep
/// their own vertices (shorter ones are upsampled), each is oriented CCW about
/// its local loft direction keeping vertex 0 (so vertex j pairs with vertex j
/// along a curling spine such as a helix), and the result is flipped outward
/// by signed volume. Mirrors castiron's `loft_solid`.
fn loft_sections(sections: &[Vec<Pnt>]) -> Solid {
    let secs: Vec<&Vec<Pnt>> = sections.iter().filter(|s| s.len() >= 3).collect();
    if secs.len() < 2 {
        return Solid::empty();
    }
    let n = secs.iter().map(|s| s.len()).max().unwrap_or(0);
    let secs: Vec<Vec<Pnt>> = secs
        .iter()
        .map(|s| if s.len() == n { (*s).clone() } else { upsample_loop(s, n) })
        .collect();
    let cents: Vec<Pnt> = secs
        .iter()
        .map(|s| s.iter().fold(Pnt::origin(), |a, p| a + *p) * (1.0 / s.len() as f64))
        .collect();
    let k = secs.len();
    let secs: Vec<Vec<Pnt>> = secs
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let mut d = if i + 1 < k { cents[i + 1] - cents[i] } else { cents[i] - cents[i - 1] };
            if d.norm() < 1e-12 {
                d = newell(&sections[0]);
            }
            if newell(&s).dot(d) < 0.0 {
                let mut r = vec![s[0]];
                r.extend(s[1..].iter().rev().cloned());
                r
            } else {
                s
            }
        })
        .collect();
    let mut m = TriMesh::new();
    for w in secs.windows(2) {
        let (s0, s1) = (&w[0], &w[1]);
        for i in 0..n {
            let j = (i + 1) % n;
            // CCW sections about the ascent: (v00, v11, v10), (v00, v01, v11)
            m.push_triangle(s0[i], s1[j], s1[i]);
            m.push_triangle(s0[i], s0[j], s1[j]);
        }
    }
    // caps: first faces back along the loft, last faces forward
    let cap = |m: &mut TriMesh, loop_: &[Pnt], forward: bool| {
        let face = Face::new(make_polygon(loop_));
        let fwd_n = newell(loop_);
        for t in face.triangulate() {
            let tn = (t[1] - t[0]).cross(t[2] - t[0]);
            let same = tn.dot(fwd_n) >= 0.0;
            if same == forward {
                m.push_triangle(t[0], t[1], t[2]);
            } else {
                m.push_triangle(t[0], t[2], t[1]);
            }
        }
    };
    cap(&mut m, &secs[0], false);
    cap(&mut m, &secs[k - 1], true);
    let mut m = m.welded(1e-7);
    if m.signed_volume() < 0.0 {
        m.flip();
    }
    Solid::from_mesh(m)
}

fn op_loft(state: &mut State, id: &str, node: &DagNode) {
    let is_cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    // shapes/profiles are the profile ids; sketchs/sketches are the plane ids.
    let shape_ids: Vec<String> = {
        let mut v = dep_ids(&node.deps, "shapes");
        v.extend(dep_ids(&node.deps, "profiles"));
        v
    };
    let sketch_ids: Vec<String> = {
        let mut v = dep_ids(&node.deps, "sketchs");
        v.extend(dep_ids(&node.deps, "sketches"));
        v
    };
    if shape_ids.len() < 2 {
        state
            .unsupported
            .push(format!("Loft<2sections#{id}"));
        return;
    }
    let mut sections: Vec<Vec<Pnt>> = Vec::new();
    for (i, sid) in shape_ids.iter().enumerate() {
        let plane = if i < sketch_ids.len() {
            resolve_section_plane(state, &sketch_ids[i])
        } else {
            Trsf::identity()
        };
        if let Some(pts) = resolve_shape_loop(state, sid, &plane) {
            if pts.len() >= 3 {
                sections.push(pts);
            }
        }
    }
    if sections.len() < 2 {
        state.error(format!("Loft#{id}: fewer than 2 resolved sections"));
        return;
    }
    let solid = loft_sections(&sections);
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut: is_cut });
}

/// Build two perpendicular axes for a given forward direction.
fn perpendicular_basis(fwd: Pnt) -> (Pnt, Pnt) {
    let not_n = if fwd.x.abs() < 0.9 {
        Pnt::new(1.0, 0.0, 0.0)
    } else {
        Pnt::new(0.0, 1.0, 0.0)
    };
    let tx = fwd.cross(not_n).normalized();
    let ty = fwd.cross(tx).normalized();
    (tx, ty)
}

/// Rotation-minimizing path frames for sweeping a profile along a polyline.
fn build_path_frames(path: &[Pnt]) -> Vec<Trsf> {
    let n = path.len();
    if n < 2 {
        return Vec::new();
    }
    let mut frames = Vec::with_capacity(n);
    let fwd = (path[1] - path[0]).normalized();
    let (mut tx, mut ty) = perpendicular_basis(fwd);
    frames.push(Trsf {
        m: [
            [tx.x, ty.x, fwd.x],
            [tx.y, ty.y, fwd.y],
            [tx.z, ty.z, fwd.z],
        ],
        t: [path[0].x, path[0].y, path[0].z],
    });
    for i in 1..n {
        let new_fwd = if i + 1 < n {
            (path[i + 1] - path[i - 1]).normalized()
        } else {
            (path[i] - path[i - 1]).normalized()
        };
        // Project tx onto plane perpendicular to new_fwd (rotation-minimizing transport).
        let dot = tx.dot(new_fwd);
        let new_tx_raw = tx - new_fwd * dot;
        let new_tx = if new_tx_raw.norm() > 1e-9 {
            new_tx_raw.normalized()
        } else {
            tx
        };
        let new_ty_raw = new_fwd.cross(new_tx);
        let new_ty = if new_ty_raw.norm() > 1e-9 {
            new_ty_raw.normalized()
        } else {
            ty
        };
        tx = new_tx;
        ty = new_ty;
        frames.push(Trsf {
            m: [
                [tx.x, ty.x, new_fwd.x],
                [tx.y, ty.y, new_fwd.y],
                [tx.z, ty.z, new_fwd.z],
            ],
            t: [path[i].x, path[i].y, path[i].z],
        });
    }
    frames
}

/// The X direction OCCT's `gp_Ax2(P, V)` picks for main direction `V`, so a
/// helix on `gp_Ax3(center, dir)` starts where replicad's `makeHelix` does.
fn ax2_xdir(v: Pnt) -> Pnt {
    let (a, b, c) = (v.x, v.y, v.z);
    let (aa, ba, ca) = (a.abs(), b.abs(), c.abs());
    let x = if ba <= aa && ba <= ca {
        if aa > ca { Pnt::new(-c, 0.0, a) } else { Pnt::new(c, 0.0, -a) }
    } else if aa <= ba && aa <= ca {
        if ba > ca { Pnt::new(0.0, -c, b) } else { Pnt::new(0.0, c, -b) }
    } else if aa > ba {
        Pnt::new(-b, a, 0.0)
    } else {
        Pnt::new(b, -a, 0.0)
    };
    x.normalized()
}

const HELIX_SEG_PER_TURN: f64 = 36.0;

fn op_helix3d(state: &mut State, id: &str, node: &DagNode) {
    let num = |k: &str, d: f64| state.resolve_num(&dep_id(&node.deps, k)).unwrap_or(d);
    let (pitch, height, radius) = (num("pitch", 1.0), num("height", 1.0), num("radius", 1.0));
    let center = dep_id(&node.deps, "center")
        .and_then(|c| state.points3d.get(&c).copied())
        .unwrap_or_else(Pnt::origin);
    let axis = dep_id(&node.deps, "dir")
        .and_then(|c| state.points3d.get(&c).copied())
        .unwrap_or(Pnt::new(0.0, 0.0, 1.0))
        .normalized();
    let sign = if state.resolve_bool(&dep_id(&node.deps, "lefthand")) { -1.0 } else { 1.0 };
    let e1 = ax2_xdir(axis);
    let e2 = axis.cross(e1);
    let turns = if pitch != 0.0 { height / pitch } else { 0.0 };
    let npts = ((turns * HELIX_SEG_PER_TURN).ceil() as usize).clamp(2, 6000);
    let pts: Vec<Pnt> = (0..=npts)
        .map(|i| {
            let f = i as f64 / npts as f64;
            let a = sign * 2.0 * PI * turns * f;
            center + (e1 * a.cos() + e2 * a.sin()) * radius + axis * (height * f)
        })
        .collect();
    state.spline3ds.insert(id.to_string(), pts);
    state
        .helices
        .insert(id.to_string(), Helix { center, axis, radius, pitch, height, sign });
}

/// Rodrigues rotation of vector `v` about unit `k` by `ang`.
fn rot_vec(v: Pnt, k: Pnt, ang: f64) -> Pnt {
    let (c, s) = (ang.cos(), ang.sin());
    v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
}

/// Rotation-minimising frames (double reflection) along a polyline:
/// (tangent, r) per point.
fn rmf(path: &[Pnt]) -> Vec<(Pnt, Pnt)> {
    let m = path.len();
    let tans: Vec<Pnt> = (0..m)
        .map(|i| (path[(i + 1).min(m - 1)] - path[i.saturating_sub(1)]).normalized())
        .collect();
    let (mut r, _) = perpendicular_basis(tans[0]);
    let mut out = vec![(tans[0], r)];
    for i in 0..m - 1 {
        let v1 = path[i + 1] - path[i];
        let c1 = v1.dot(v1);
        if c1 < 1e-24 {
            out.push((tans[i + 1], r));
            continue;
        }
        let rl = r - v1 * (2.0 / c1 * v1.dot(r));
        let tl = tans[i] - v1 * (2.0 / c1 * v1.dot(tans[i]));
        let v2 = tans[i + 1] - tl;
        let c2 = v2.dot(v2);
        r = if c2 < 1e-24 { rl } else { rl - v2 * (2.0 / c2 * v2.dot(rl)) }.normalized();
        out.push((tans[i + 1], r));
    }
    out
}

/// Sweep the way the OCCT kernel does (replicad `genericSweep` with
/// `forceProfileSpineOthogonality`): the profile is drawn on its sketch
/// plane's axes re-origined at the path start, rotated about that point to be
/// orthogonal to the tangent, then moved by the path law — the exact helical
/// motion for a Helix3D, rotation-minimising frames otherwise.
fn op_sweep(state: &mut State, id: &str, node: &DagNode) {
    let is_cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let profile_id = match dep_id(&node.deps, "profile").or_else(|| dep_id(&node.deps, "shape")) {
        Some(p) => p,
        None => {
            state.error(format!("Sweep#{id}: no profile"));
            return;
        }
    };
    let path_id = match dep_id(&node.deps, "path") {
        Some(p) => p,
        None => {
            state.error(format!("Sweep#{id}: no path"));
            return;
        }
    };
    let local = match resolve_shape_loop(state, &profile_id, &Trsf::identity()) {
        Some(p) if p.len() >= 3 => p,
        _ => {
            state.error(format!("Sweep#{id}: profile unresolved"));
            return;
        }
    };
    let path = state.spline3ds.get(&path_id).cloned().unwrap_or_default();
    if path.len() < 2 {
        state.error(format!("Sweep#{id}: path has < 2 points"));
        return;
    }
    let twist = state.resolve_num(&dep_id(&node.deps, "twist")).unwrap_or(0.0).to_radians();
    // Point3D-built paths are in the owning part's frame; sketches are already
    // world-placed through their frame chain.
    let part = state.owner_frame.get(id).and_then(|f| state.frames.get(f)).copied();
    let (path, helix_local) = match part {
        Some(t) => (
            path.iter().map(|p| t.apply_point(*p)).collect::<Vec<Pnt>>(),
            state.helices.get(&path_id).map(|h| Helix {
                center: t.apply_point(h.center),
                axis: t.apply_vector(h.axis).normalized(),
                ..*h
            }),
        ),
        None => (path, state.helices.get(&path_id).copied()),
    };
    let plane = op_plane(state, node);
    let xd = plane.apply_vector(Pnt::new(1.0, 0.0, 0.0)).normalized();
    let nd = plane.apply_vector(Pnt::new(0.0, 0.0, 1.0)).normalized();
    let yd = nd.cross(xd);
    let origin = path[0];
    let mut base: Vec<Pnt> = local.iter().map(|p| origin + xd * p.x + yd * p.y).collect();

    let helix = helix_local;
    let frames = if helix.is_none() { rmf(&path) } else { Vec::new() };
    let t0 = match helix {
        Some(h) => {
            let e1 = (path[0] - h.center).normalized();
            let e2 = h.axis.cross(e1);
            (e2 * (h.sign * 2.0 * PI * h.radius) + h.axis * h.pitch).normalized()
        }
        None => frames[0].0,
    };
    let n0 = if nd.dot(t0) >= 0.0 { nd } else { -nd };
    let ax = n0.cross(t0);
    let (sn, cs) = (ax.norm(), n0.dot(t0).clamp(-1.0, 1.0));
    if sn > 1e-12 {
        let (k, ang) = (ax * (1.0 / sn), sn.atan2(cs));
        base = base.iter().map(|p| origin + rot_vec(*p - origin, k, ang)).collect();
    }

    let last = (path.len() - 1).max(1) as f64;
    let sections: Vec<Vec<Pnt>> = match helix {
        Some(h) => {
            let turns = if h.pitch != 0.0 { h.height / h.pitch } else { 0.0 };
            (0..path.len())
                .map(|i| {
                    let f = i as f64 / last;
                    let ang = h.sign * 2.0 * PI * turns * f;
                    base.iter()
                        .map(|p| {
                            let p = if twist != 0.0 { origin + rot_vec(*p - origin, t0, twist * f) } else { *p };
                            h.center + rot_vec(p - h.center, h.axis, ang) + h.axis * (h.height * f)
                        })
                        .collect()
                })
                .collect()
        }
        None => {
            let (t_0, r_0) = frames[0];
            let b_0 = t_0.cross(r_0);
            let loc: Vec<(f64, f64, f64)> = base
                .iter()
                .map(|p| {
                    let d = *p - origin;
                    (d.dot(r_0), d.dot(b_0), d.dot(t_0))
                })
                .collect();
            frames
                .iter()
                .enumerate()
                .map(|(i, &(t, r))| {
                    let b = t.cross(r);
                    let tw = twist * i as f64 / last;
                    let (c, s) = (tw.cos(), tw.sin());
                    loc.iter()
                        .map(|&(u, v, w)| path[i] + r * (u * c - v * s) + b * (u * s + v * c) + t * w)
                        .collect()
                })
                .collect()
        }
    };
    let solid = loft_sections(&sections);
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut: is_cut });
}

fn op_shell(state: &mut State, id: &str, node: &DagNode) {
    let is_cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    let thickness = state
        .resolve_num(&dep_id(&node.deps, "thickness"))
        .unwrap_or(1.0);
    let solid_id = match dep_id(&node.deps, "solid")
        .or_else(|| dep_id(&node.deps, "body"))
        .or_else(|| dep_id(&node.deps, "operation"))
    {
        Some(s) => s,
        None => {
            state.error(format!("Shell#{id}: no solid"));
            return;
        }
    };
    let outer = match state.operations.get(&solid_id).map(|op| op.solid.clone()) {
        Some(s) => s,
        None => {
            state.error(format!("Shell#{id}: solid {solid_id} not found"));
            return;
        }
    };
    let bb = outer.bbox();
    let t = thickness;
    let ix = (bb.max[0] - bb.min[0] - 2.0 * t).max(0.0);
    let iy = (bb.max[1] - bb.min[1] - 2.0 * t).max(0.0);
    let iz = (bb.max[2] - bb.min[2] - t).max(0.0); // open on top
    if ix < 1e-9 || iy < 1e-9 || iz < 1e-9 {
        state
            .operations
            .insert(id.to_string(), OpResult { solid: outer, cut: is_cut });
        return;
    }
    let inner = make_box(
        Pnt::new(bb.min[0] + t, bb.min[1] + t, bb.min[2] + t),
        ix,
        iy,
        iz,
    );
    let hollow = cut(&outer, &inner);
    state
        .operations
        .insert(id.to_string(), OpResult { solid: hollow, cut: is_cut });
}

fn op_thicken(state: &mut State, id: &str, node: &DagNode) {
    // Thicken passes the surface/shape dep through (it's already a solid from SurfaceLoft).
    let solid_id = match dep_id(&node.deps, "surface")
        .or_else(|| dep_id(&node.deps, "shape"))
        .or_else(|| dep_id(&node.deps, "solid"))
    {
        Some(s) => s,
        None => return,
    };
    let is_cut = state.resolve_bool(&dep_id(&node.deps, "cut"));
    if let Some(op) = state.operations.get(&solid_id).cloned() {
        state
            .operations
            .insert(id.to_string(), OpResult { solid: op.solid, cut: is_cut });
    }
}

fn op_stitch(state: &mut State, id: &str, node: &DagNode) {
    // Stitch joins a list of surfaces (operations) into a single solid via fuse_all.
    let surface_ids = dep_ids(&node.deps, "surfaces");
    if surface_ids.is_empty() {
        // Fall back to treating it like a Shell if a solid dep is present.
        op_shell(state, id, node);
        return;
    }
    let solids: Vec<Solid> = surface_ids
        .iter()
        .filter_map(|sid| state.operations.get(sid).map(|op| op.solid.clone()))
        .filter(|s| !s.is_empty())
        .collect();
    if solids.is_empty() {
        state.error(format!("Stitch#{id}: no resolved surfaces"));
        return;
    }
    let solid = fuse_all(&solids);
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut: false });
}

// ===================================================================
// Sheet-metal operations
// ===================================================================

fn op_sm_base_flange(state: &mut State, id: &str, node: &DagNode) {
    let plane = op_plane(state, node);
    let thickness = state
        .resolve_num(&dep_id(&node.deps, "thickness"))
        .unwrap_or(1.0);
    let direction = dep_id(&node.deps, "direction")
        .and_then(|d| state.strings.get(&d).cloned())
        .unwrap_or_else(|| "positive".to_string());
    let profile_id = match dep_id(&node.deps, "profile").or_else(|| dep_id(&node.deps, "shape")) {
        Some(p) => p,
        None => return,
    };
    let loop_pts = match resolve_shape_loop(state, &profile_id, &plane) {
        Some(p) if p.len() >= 3 => p,
        _ => return,
    };
    let normal = plane.apply_vector(Pnt::new(0.0, 0.0, 1.0)).normalized();
    let (start_off, end_off) = match direction.as_str() {
        "negative" => (-thickness, 0.0),
        "both" => (-thickness / 2.0, thickness / 2.0),
        _ => (0.0, thickness), // "positive"
    };
    let offset = normal * start_off;
    let vec = normal * (end_off - start_off);
    let shifted: Vec<Pnt> = loop_pts.iter().map(|p| *p + offset).collect();
    let face = make_face(make_polygon(&shifted));
    let solid = make_prism(&face, vec);
    state.sm_bodies.insert(
        id.to_string(),
        SmBody {
            solid: solid.clone(),
            thickness,
            plane,
        },
    );
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut: false });
}

fn op_sm_to_solid(state: &mut State, id: &str, node: &DagNode) {
    let body_id = match dep_id(&node.deps, "body") {
        Some(b) => b,
        None => return,
    };
    let solid = if let Some(sm) = state.sm_bodies.get(&body_id).cloned() {
        sm.solid
    } else if let Some(op) = state.operations.get(&body_id).cloned() {
        op.solid
    } else {
        return;
    };
    state
        .operations
        .insert(id.to_string(), OpResult { solid, cut: false });
}

fn op_sm_edge_flange(state: &mut State, id: &str, node: &DagNode) {
    let body_id = match dep_id(&node.deps, "body") {
        Some(b) => b,
        None => return,
    };
    let length = state
        .resolve_num(&dep_id(&node.deps, "length"))
        .unwrap_or(20.0);
    let bend_angle_deg = state
        .resolve_num(&dep_id(&node.deps, "bend_angle"))
        .unwrap_or(90.0);

    // Retrieve the parent sm body (or promote an operation to one).
    let parent = if let Some(sm) = state.sm_bodies.get(&body_id).cloned() {
        sm
    } else if let Some(op) = state.operations.get(&body_id).cloned() {
        let t = 2.0; // default thickness guess
        SmBody {
            solid: op.solid,
            thickness: t,
            plane: Trsf::identity(),
        }
    } else {
        return;
    };

    let bb = parent.solid.bbox();
    let t = parent.thickness;
    let dx = bb.max[0] - bb.min[0];
    let dy = bb.max[1] - bb.min[1];
    let bend_rad = bend_angle_deg.to_radians();
    let flange_z = (length * bend_rad.sin()).max(1e-3);

    // Attach a simple perpendicular flange to the longer horizontal edge.
    let flange = if dy >= dx {
        make_box(
            Pnt::new(bb.max[0] - t, bb.min[1], bb.max[2]),
            t,
            dy,
            flange_z,
        )
    } else {
        make_box(
            Pnt::new(bb.min[0], bb.max[1] - t, bb.max[2]),
            dx,
            t,
            flange_z,
        )
    };

    let new_solid = fuse(&parent.solid, &flange);
    state.sm_bodies.insert(
        id.to_string(),
        SmBody {
            solid: new_solid.clone(),
            thickness: t,
            plane: parent.plane,
        },
    );
    state
        .operations
        .insert(id.to_string(), OpResult { solid: new_solid, cut: false });
}

fn op_sm_passthrough(state: &mut State, id: &str, node: &DagNode) {
    let body_id = dep_id(&node.deps, "body")
        .or_else(|| dep_id(&node.deps, "solid"))
        .or_else(|| dep_id(&node.deps, "operation"));
    if let Some(bid) = body_id {
        if let Some(sm) = state.sm_bodies.get(&bid).cloned() {
            state.sm_bodies.insert(id.to_string(), sm.clone());
            state.operations.insert(
                id.to_string(),
                OpResult {
                    solid: sm.solid,
                    cut: false,
                },
            );
        } else if let Some(op) = state.operations.get(&bid).cloned() {
            state.operations.insert(id.to_string(), op);
        }
    }
}

// ===================================================================
// Part assembly
// ===================================================================

fn finalize_part(state: &mut State, id: &str, node: &DagNode) {
    let op_ids = dep_ids(&node.deps, "operations");
    // Batch each run of consecutive same-direction operations into ONE
    // boolean (as castiron does): A op (B u C) equals the sequential form, the
    // union of disjoint tools is a cheap concatenation, and the body is cut or
    // fused once instead of being re-fragmented by every stud or hole.
    let ops: Vec<OpResult> = op_ids.iter().filter_map(|i| state.operations.get(i).cloned()).collect();
    let mut body = Solid::empty();
    let mut i = 0;
    while i < ops.len() {
        let is_cut = ops[i].cut;
        let mut j = i;
        let mut tools: Vec<Solid> = Vec::new();
        while j < ops.len() && ops[j].cut == is_cut {
            tools.push(ops[j].solid.clone());
            j += 1;
        }
        let tool = if tools.len() == 1 { tools.pop().unwrap() } else { fuse_all(&tools) };
        if is_cut {
            body = cut(&body, &tool);
        } else {
            body = fuse(&body, &tool);
        }
        i = j;
    }
    // No part-frame transform here: every sketch plane already chains through
    // `top_frame` up to the (assembly-positioned) part frame, so operation
    // solids are world-placed. Applying the part frame again put assembly
    // parts at twice their offset (LegoRow, gear_pair, Playground, PrusaI3).
    let name = dep_id(&node.deps, "name")
        .and_then(|n| state.strings.get(&n).cloned())
        .unwrap_or_else(|| format!("part_{id}"));
    state.parts.insert(
        id.to_string(),
        PartResult {
            name,
            solid: body.cleaned(1e-6),
        },
    );
    state.part_order.push(id.to_string());
}

// ===================================================================
// Top-level walk
// ===================================================================

/// Compile a whole DAG into the accumulated [`State`].
pub fn compile(input: &CompilerInputDag, state: &mut State) {
    let names = input.type_names();
    // owner part frame of every node reachable from a part's operations
    for id in input.topo_order() {
        let Some(node) = input.node(&id) else { continue };
        let tn = input.type_name_of(node, &names);
        if tn != "Part" && tn != "PartRoot" {
            continue;
        }
        let Some(frame) = dep_id(&node.deps, "frame") else { continue };
        let mut stack = dep_ids(&node.deps, "operations");
        let mut seen = std::collections::HashSet::new();
        while let Some(n) = stack.pop() {
            if !seen.insert(n.clone()) {
                continue;
            }
            let Some(nn) = input.node(&n) else { continue };
            let t = input.type_name_of(nn, &names);
            if t == "Part" || t == "PartRoot" {
                continue;
            }
            state.owner_frame.entry(n.clone()).or_insert_with(|| frame.clone());
            for v in nn.deps.as_object().into_iter().flat_map(|o| o.values()) {
                match v {
                    serde_json::Value::String(s) => stack.push(s.clone()),
                    serde_json::Value::Array(a) => {
                        stack.extend(a.iter().filter_map(|x| x.as_str().map(String::from)))
                    }
                    _ => {}
                }
            }
        }
    }
    for id in input.topo_order() {
        if let Some(node) = input.node(&id) {
            let type_name = input.type_name_of(node, &names);
            dispatch(state, &type_name, &id, node);
        }
    }
}
