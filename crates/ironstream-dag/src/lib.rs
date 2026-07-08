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
pub mod iou;
pub mod nodes;
pub mod state;

use ironstream::prelude::MeshParams;
use state::{PartResult, State};

/// The outcome of compiling a DAG: the parts plus diagnostics.
pub struct CompileResult {
    pub parts: Vec<PartResult>,
    pub errors: Vec<String>,
    pub unsupported: Vec<String>,
}

/// Compile a parsed DAG into parts, using the given tessellation resolution.
pub fn compile(input: &dag::CompilerInputDag, mesh_params: MeshParams) -> CompileResult {
    let mut state = State::new();
    state.mesh_params = mesh_params;
    nodes::compile(input, &mut state);

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
    }
}

/// Convenience: parse a DAG JSON string and compile it with default resolution.
pub fn compile_json(json: &str) -> Result<CompileResult, String> {
    let input = dag::CompilerInputDag::from_json(json)?;
    Ok(compile(&input, MeshParams::default()))
}
