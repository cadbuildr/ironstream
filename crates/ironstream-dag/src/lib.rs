//! # ironstream-dag
//!
//! The integration layer between CADbuildr's kernel-agnostic **foundation DAG**
//! and the [`ironstream`] geometry kernel. It walks a serialized DAG
//! (`CompilerInputDAG`) node-by-node — exactly the processing model the replicad
//! and truck kernel adapters use — resolving each node's dependencies and
//! emitting IronStream solids, then combining per-part operations (fuse for
//! additive, cut for subtractive) into finished parts.
//!
//! ```no_run
//! let json = std::fs::read_to_string("cube.json").unwrap();
//! let result = ironstream_dag::compile_json(&json).unwrap();
//! for part in &result.parts {
//!     println!("{}: volume {:.3}", part.name, part.solid.volume());
//! }
//! ```

// Explicit angle comparisons read more clearly than RangeInclusive::contains
// in the arc/winding math.
#![allow(clippy::manual_range_contains)]

pub mod dag;
pub mod instance;
pub mod iou;
pub mod nodes;
pub mod state;
pub mod thread;

use ironstream::prelude::MeshParams;
use state::{CompileStats, PartResult, State};

/// The outcome of compiling a DAG: the parts plus diagnostics.
pub struct CompileResult {
    pub parts: Vec<PartResult>,
    pub errors: Vec<String>,
    pub unsupported: Vec<String>,
    pub stats: CompileStats,
}

/// A compiler that keeps its part and operation caches between compiles, so
/// re-compiling after a parameter edit only rebuilds what the edit changed.
/// Entries the latest compile did not use are dropped, so memory follows the
/// current model.
pub struct Compiler {
    mesh_params: MeshParams,
    part_cache: std::collections::HashMap<instance::Key, ironstream::prelude::Solid>,
    op_cache: std::collections::HashMap<instance::Key, state::OpResult>,
    prev_parts: std::collections::HashSet<instance::Key>,
    prev_ops: std::collections::HashSet<instance::Key>,
}

impl Compiler {
    pub fn new(mesh_params: MeshParams) -> Self {
        Self {
            mesh_params,
            part_cache: Default::default(),
            op_cache: Default::default(),
            prev_parts: Default::default(),
            prev_ops: Default::default(),
        }
    }

    /// Change the tessellation; cached solids were meshed with the old one.
    pub fn set_mesh_params(&mut self, mesh_params: MeshParams) {
        if format!("{:?}", mesh_params) != format!("{:?}", self.mesh_params) {
            self.clear();
        }
        self.mesh_params = mesh_params;
    }

    /// The part-local solid cached for an instance key (from `PartInstance`).
    pub fn part_local(&self, key_hex: &str) -> Option<&ironstream::prelude::Solid> {
        let k = u128::from_str_radix(key_hex, 16).ok()?;
        self.part_cache.get(&k)
    }

    /// Approximate bytes held by the part and operation caches.
    pub fn cache_bytes(&self) -> usize {
        let mesh =
            |s: &ironstream::prelude::Solid| s.mesh().verts.len() * 24 + s.mesh().tris.len() * 24;
        self.part_cache.values().map(mesh).sum::<usize>()
            + self
                .op_cache
                .values()
                .map(|o| mesh(&o.solid))
                .sum::<usize>()
    }

    pub fn clear(&mut self) {
        self.part_cache.clear();
        self.op_cache.clear();
        self.prev_parts.clear();
        self.prev_ops.clear();
    }

    pub fn compile(&mut self, input: &dag::CompilerInputDag) -> CompileResult {
        let mut state = State::new();
        state.mesh_params = self.mesh_params;
        state.part_cache = std::mem::take(&mut self.part_cache);
        state.op_cache = std::mem::take(&mut self.op_cache);
        nodes::compile(input, &mut state);

        // Keep what this compile and the one before used: an edit and its
        // undo both stay cached, while memory still follows the model.
        let used_parts = std::mem::take(&mut state.used_parts);
        let used_ops = std::mem::take(&mut state.used_ops);
        self.part_cache = std::mem::take(&mut state.part_cache);
        self.part_cache
            .retain(|k, _| used_parts.contains(k) || self.prev_parts.contains(k));
        self.op_cache = std::mem::take(&mut state.op_cache);
        self.op_cache
            .retain(|k, _| used_ops.contains(k) || self.prev_ops.contains(k));
        self.prev_parts = used_parts;
        self.prev_ops = used_ops;

        // Preserve discovery order of parts.
        let parts = state
            .part_order
            .iter()
            .filter_map(|id| state.parts.get(id).cloned())
            .collect::<Vec<_>>();

        CompileResult {
            parts,
            errors: state.errors,
            unsupported: state.unsupported,
            stats: state.stats,
        }
    }
}

/// Compile a parsed DAG into parts, using the given tessellation resolution.
pub fn compile(input: &dag::CompilerInputDag, mesh_params: MeshParams) -> CompileResult {
    Compiler::new(mesh_params).compile(input)
}

/// Convenience: parse a DAG JSON string and compile it with default resolution.
pub fn compile_json(json: &str) -> Result<CompileResult, String> {
    let input = dag::CompilerInputDag::from_json(json)?;
    Ok(compile(&input, MeshParams::default()))
}

/// One part as an ISO-10303-21 (STEP AP214) file. Faces whose provenance is
/// known (planes, cylinders, spheres, tori) are written as analytic faces;
/// the rest as planar facets, in one closed shell.
pub fn part_step(part: &PartResult) -> String {
    ironstream::analytic_step::write_step_analytic(&part.solid, &part.name)
}

/// Several parts in one STEP file, one solid each.
pub fn parts_step(parts: &[&PartResult], name: &str) -> String {
    let solids: Vec<(&ironstream::prelude::Solid, &str)> =
        parts.iter().map(|p| (&p.solid, p.name.as_str())).collect();
    ironstream::analytic_step::write_step_analytic_multi(&solids, name)
}
