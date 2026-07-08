//! Node handlers — the node-by-node DAG processor.
//!
//! This mirrors the dispatch structure of the replicad and truck kernel
//! adapters: walk the DAG in dependency order and, for each node, run a handler
//! keyed by the node's **type name** (resolved per-DAG via `serializableNodes`).
//! Handlers resolve their dependencies from [`State`] and write their result
//! back under the node id.

use crate::dag::{dep_id, dep_ids, CompilerInputDag, DagNode};
use crate::state::{Curve2, OpResult, PartResult, Profile, SketchInfo, SmBody, State, P2};
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
        | "Helix3D"
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
    let offset = normal * start;
    let vec = normal * (end - start);
    let mut solids: Vec<Solid> = Vec::new();
    for sid in &shape_ids {
        match resolve_shape_loop(state, sid, &plane) {
            Some(loop_pts) if loop_pts.len() >= 3 => {
                let shifted: Vec<Pnt> = loop_pts.iter().map(|p| *p + offset).collect();
                let face = make_face(make_polygon(&shifted));
                solids.push(make_prism(&face, vec));
            }
            _ => state.error(format!("Extrusion#{id}: shape {sid} unresolved")),
        }
    }
    if solids.is_empty() {
        return;
    }
    let solid = fuse_all(&solids);
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
fn loft_sections(sections: &[Vec<Pnt>]) -> Solid {
    if sections.len() < 2 {
        return Solid::empty();
    }
    let n = sections.iter().map(|s| s.len()).min().unwrap_or(0);
    if n < 3 {
        return Solid::empty();
    }
    let resampled: Vec<Vec<Pnt>> = sections.iter().map(|s| resample_loop(s, n)).collect();
    let mut m = TriMesh::new();
    // Side quads between consecutive sections.
    for w in resampled.windows(2) {
        let (s0, s1) = (&w[0], &w[1]);
        for i in 0..n {
            let j = (i + 1) % n;
            m.push_triangle(s0[i], s1[i], s0[j]);
            m.push_triangle(s1[i], s1[j], s0[j]);
        }
    }
    // End caps.
    let first_wire = make_polygon(&resampled[0]);
    let first_face = Face::new(first_wire);
    for tri in first_face.triangulate() {
        m.push_triangle(tri[0], tri[2], tri[1]); // reversed for outward normal
    }
    let last = &resampled[resampled.len() - 1];
    let last_wire = make_polygon(last);
    let last_face = Face::new(last_wire);
    for tri in last_face.triangulate() {
        m.push_triangle(tri[0], tri[1], tri[2]);
    }
    Solid::from_mesh(m.welded(1e-7))
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
    // Profile points in identity plane (local 2D coords lifted to z=0).
    let profile_pts_local = match resolve_shape_loop(state, &profile_id, &Trsf::identity()) {
        Some(p) if p.len() >= 3 => p,
        _ => {
            state.error(format!("Sweep#{id}: profile unresolved"));
            return;
        }
    };
    let path_pts = state.spline3ds.get(&path_id).cloned().unwrap_or_default();
    if path_pts.len() < 2 {
        state.error(format!("Sweep#{id}: path has < 2 points"));
        return;
    }
    let frames = build_path_frames(&path_pts);
    let sections: Vec<Vec<Pnt>> = frames
        .iter()
        .map(|frame| {
            profile_pts_local
                .iter()
                .map(|p| frame.apply_point(*p))
                .collect()
        })
        .collect();
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
    let mut body = Solid::empty();
    for op_id in &op_ids {
        if let Some(op) = state.operations.get(op_id).cloned() {
            if op.cut {
                body = cut(&body, &op.solid);
            } else {
                body = fuse(&body, &op.solid);
            }
        }
    }
    // Place the part by its frame.
    if let Some(frame) = dep_id(&node.deps, "frame").and_then(|f| state.frames.get(&f).copied()) {
        body = transform(&body, &frame);
    }
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
    for id in input.topo_order() {
        if let Some(node) = input.node(&id) {
            let type_name = input.type_name_of(node, &names);
            dispatch(state, &type_name, &id, node);
        }
    }
}
