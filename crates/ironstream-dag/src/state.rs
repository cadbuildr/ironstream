//! Compile state — the per-run accumulation of resolved DAG values, structured
//! like the replicad/truck `parameterStore` but typed by category.
//!
//! Each handler resolves its dependencies out of these maps and writes its own
//! result back under the node id, so later nodes can pick it up. This is the
//! direct Rust analogue of replicad's `parameterStore[node.id] = ...`.

use ironstream::prelude::*;
use std::collections::HashMap;

/// A sheet-metal body: the accumulated solid plus the original thickness and
/// base plane (used by EdgeFlange to compute perpendicular flanges).
#[derive(Clone, Debug)]
pub struct SmBody {
    pub solid: Solid,
    pub thickness: f64,
    pub plane: Trsf,
}

/// A 2D point in some sketch's local plane coordinates.
#[derive(Clone, Copy, Debug)]
pub struct P2 {
    pub x: f64,
    pub y: f64,
}

/// A sketch entity that contributes ordered loop points (line, arc, ...).
#[derive(Clone, Debug)]
pub enum Curve2 {
    /// A straight edge p1 -> p2 (point ids).
    Line { p1: String, p2: String },
    /// A circular arc from p1 to p2 bulging through a sagitta `bulge`
    /// (perpendicular offset of the midpoint), discretized on demand.
    Arc { p1: String, p2: String, bulge: f64 },
}

/// A closed 2D profile: an ordered list of sketch curves forming a loop.
#[derive(Clone, Debug)]
pub struct Profile {
    pub curves: Vec<Curve2>,
    /// Direct point loop (used by `CustomClosedShape`/`Polygon` when given as
    /// an explicit point list rather than edges).
    pub explicit_points: Option<Vec<String>>,
}

/// A sketch: which plane it lives on, plus its origin point id.
#[derive(Clone, Debug)]
pub struct SketchInfo {
    pub plane_id: Option<String>,
    pub origin_id: Option<String>,
}

/// The result of an operation: a solid plus whether it subtracts (cut) from or
/// adds to the running part body.
#[derive(Clone, Debug)]
pub struct OpResult {
    pub solid: Solid,
    pub cut: bool,
}

/// A finished part: its combined solid and display name.
#[derive(Clone, Debug)]
pub struct PartResult {
    pub name: String,
    pub solid: Solid,
}

#[derive(Default)]
pub struct State {
    pub numbers: HashMap<String, f64>,
    pub bools: HashMap<String, bool>,
    pub strings: HashMap<String, String>,
    pub points2d: HashMap<String, P2>,
    pub points3d: HashMap<String, Pnt>,
    pub curves: HashMap<String, Curve2>,
    pub profiles: HashMap<String, Profile>,
    /// circle id -> (center point id, radius number id) — a closed shape.
    pub circles: HashMap<String, (Option<String>, Option<String>)>,
    pub frames: HashMap<String, Trsf>,
    pub planes: HashMap<String, Trsf>,
    pub sketches: HashMap<String, SketchInfo>,
    /// axis id -> (p1 point id, p2 point id), resolved against a sketch plane.
    pub axes: HashMap<String, (String, String)>,
    pub operations: HashMap<String, OpResult>,
    pub parts: HashMap<String, PartResult>,
    pub part_order: Vec<String>,
    /// Tessellation resolution for curved primitives/revolutions.
    pub mesh_params: MeshParams,
    /// Non-fatal processing errors, surfaced in the binary manifest.
    pub errors: Vec<String>,
    /// Node type names seen with no handler (for coverage reporting).
    pub unsupported: Vec<String>,
    /// ellipse id -> (center_pt_id, a_number_id, b_number_id)
    pub ellipses: HashMap<String, (Option<String>, Option<String>, Option<String>)>,
    /// spline3d id -> ordered 3D control points
    pub spline3ds: HashMap<String, Vec<Pnt>>,
    /// sheet-metal body id -> accumulated SmBody
    pub sm_bodies: HashMap<String, SmBody>,
}

impl State {
    pub fn new() -> Self {
        Self {
            mesh_params: MeshParams::default(),
            ..Default::default()
        }
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }

    /// Resolve a value that may be a FloatParameter/IntParameter id (in deps) to
    /// an f64.
    pub fn resolve_num(&self, id: &Option<String>) -> Option<f64> {
        id.as_ref().and_then(|i| self.numbers.get(i).copied())
    }

    pub fn resolve_bool(&self, id: &Option<String>) -> bool {
        id.as_ref()
            .and_then(|i| self.bools.get(i).copied())
            .unwrap_or(false)
    }

    /// Map a sketch-local 2D point id into world space using its sketch plane.
    pub fn world_point(&self, point_id: &str, plane: &Trsf) -> Option<Pnt> {
        if let Some(p) = self.points2d.get(point_id) {
            Some(plane.apply_point(Pnt::new(p.x, p.y, 0.0)))
        } else {
            self.points3d.get(point_id).copied()
        }
    }
}
