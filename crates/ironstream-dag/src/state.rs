//! Compile state — the per-run accumulation of resolved DAG values, structured
//! like the replicad/truck `parameterStore` but typed by category.
//!
//! Each handler resolves its dependencies out of these maps and writes its own
//! result back under the node id, so later nodes can pick it up. This is the
//! direct Rust analogue of replicad's `parameterStore[node.id] = ...`.

use ironstream::prelude::*;
use std::collections::{HashMap, HashSet};

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
/// An analytic helix (foundation `Helix3D`), kept so a Sweep can move its
/// profile by the exact helical motion.
#[derive(Clone, Copy, Debug)]
pub struct Helix {
    pub center: Pnt,
    pub axis: Pnt,
    pub radius: f64,
    pub pitch: f64,
    pub height: f64,
    /// +1 right-handed, -1 left-handed
    pub sign: f64,
}

#[derive(Clone, Debug)]
pub struct OpResult {
    pub solid: Solid,
    pub cut: bool,
    /// How the solid was made, where a later step can use it exactly.
    pub shape: OpShape,
}

/// Construction record of an operation solid.
#[derive(Clone, Debug)]
pub enum OpShape {
    Other,
    /// Full revolution of a planar profile (world points, one side of the
    /// axis) about `axis`.
    Revolve {
        profile: Vec<Pnt>,
        axis: Ax1,
    },
    /// Straight extrusion of one circle: base centre, full extrusion vector,
    /// radius.
    Cylinder {
        base: Pnt,
        vec: Pnt,
        radius: f64,
    },
    /// Helical sweep of a planar section (no twist).
    Helical(crate::thread::HelicalSweep),
}

/// A finished part: its combined solid and display name.
#[derive(Clone, Debug)]
pub struct PartResult {
    pub name: String,
    /// The Part node's id in the DAG.
    pub node_id: String,
    pub solid: Solid,
    /// Parts with the same key are the same shape: `solid` equals the
    /// part-local solid of the key moved by `placement`. Lets a viewer upload
    /// one mesh per key and draw the rest as instances.
    pub instance: Option<PartInstance>,
}

#[derive(Clone, Debug)]
pub struct PartInstance {
    /// Hex content key of the part's sub-DAG (frame excluded).
    pub key: String,
    /// Part-local -> world.
    pub placement: Trsf,
}

/// What the walk built and what it reused (cache and instancing counters).
#[derive(Clone, Debug, Default)]
pub struct CompileStats {
    pub parts_built: usize,
    pub parts_instanced: usize,
    pub parts_cached: usize,
    pub ops_built: usize,
    pub ops_cached: usize,
    pub ops_skipped: usize,
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
    /// Helix3D id -> analytic helix (its samples also live in `spline3ds`).
    pub helices: HashMap<String, Helix>,
    /// node id -> frame id of the part that owns it (for part-local Point3D paths).
    pub owner_frame: HashMap<String, String>,
    /// sheet-metal body id -> accumulated SmBody
    pub sm_bodies: HashMap<String, SmBody>,
    /// Instancing plan for this compile (see `instance`).
    pub plan: crate::instance::Plan,
    /// part key -> part-local solid, kept across compiles by `Compiler`.
    pub part_cache: HashMap<crate::instance::Key, Solid>,
    /// operation key -> operation result, kept across compiles by `Compiler`.
    pub op_cache: HashMap<crate::instance::Key, OpResult>,
    /// Keys this compile used (the rest are evicted afterwards).
    pub used_parts: HashSet<crate::instance::Key>,
    pub used_ops: HashSet<crate::instance::Key>,
    /// Part keys that were already cached when this compile started.
    pub preexisting_parts: HashSet<crate::instance::Key>,
    pub stats: CompileStats,
    /// node id -> type name (for error messages).
    pub type_of: HashMap<String, String>,
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
