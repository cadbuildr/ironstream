//! # ironstream-wasm
//!
//! Browser surface of the IronStream kernel: takes a serialized CADbuildr
//! foundation DAG (JSON) and returns tessellated parts ready for three.js
//! (`positions` as a flat f32 array, `indices` as a flat u32 array).
//!
//! This is the kernel backbone of the CADmium rewrite — the UI never talks to
//! the kernel except through this boundary.

use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct JsPart {
    pub name: String,
    /// Flat vertex positions: [x0, y0, z0, x1, y1, z1, ...]
    pub positions: Vec<f32>,
    /// Flat triangle indices: [i0, j0, k0, i1, j1, k1, ...]
    pub indices: Vec<u32>,
    /// Exact volume (divergence theorem), independent of tessellation density.
    pub volume: f64,
}

#[derive(Serialize)]
pub struct JsCompileResult {
    pub parts: Vec<JsPart>,
    pub errors: Vec<String>,
    pub unsupported: Vec<String>,
}

/// Surface Rust panics as readable console errors instead of `unreachable`.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Compile a foundation DAG JSON string into tessellated parts.
#[wasm_bindgen]
pub fn compile(dag_json: &str) -> Result<JsValue, JsValue> {
    let result = ironstream_dag::compile_json(dag_json).map_err(|e| JsValue::from_str(&e))?;

    let parts = result
        .parts
        .iter()
        .map(|part| {
            let mesh = part.solid.mesh();
            JsPart {
                name: part.name.clone(),
                positions: mesh
                    .verts
                    .iter()
                    .flat_map(|v| [v.x as f32, v.y as f32, v.z as f32])
                    .collect(),
                indices: mesh
                    .tris
                    .iter()
                    .flat_map(|t| t.map(|i| i as u32))
                    .collect(),
                volume: part.solid.volume(),
            }
        })
        .collect();

    let js = JsCompileResult {
        parts,
        errors: result.errors,
        unsupported: result.unsupported,
    };
    serde_wasm_bindgen::to_value(&js).map_err(|e| JsValue::from_str(&e.to_string()))
}
