//! # ironstream-wasm
//!
//! Browser surface of the IronStream kernel: takes a serialized CADbuildr
//! foundation DAG (JSON) and returns tessellated parts ready for three.js
//! (`positions` as a flat f32 array, `indices` as a flat u32 array).
//!
//! Two entry points:
//!
//! - [`compile`]: one-shot, stateless (what the kernel benchmark drives).
//! - [`Kernel`]: a long-lived compiler for a viewer worker. It keeps a
//!   content-hash cache of operations and parts between compiles, so a
//!   parameter edit rebuilds only what the edit changed; identical parts are
//!   built once and reported as instances (`instanceKey` + `placement`), so a
//!   viewer can upload one mesh per key; and it exports STL / STEP per part
//!   from the last compile.

use ironstream::prelude::MeshParams;
use ironstream_dag::{CompileResult, Compiler};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsPart {
    pub name: String,
    /// Flat vertex positions: [x0, y0, z0, x1, y1, z1, ...] (world). Empty
    /// for an instanced part when the compile asked for `instanced` output.
    pub positions: Vec<f32>,
    /// Flat triangle indices: [i0, j0, k0, i1, j1, k1, ...]
    pub indices: Vec<u32>,
    /// Exact volume (divergence theorem), independent of tessellation density.
    pub volume: f64,
    /// Parts with the same key are the same shape (see `meshes`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_key: Option<String>,
    /// Part-local -> world, a column-major 4x4 (three.js `Matrix4.fromArray`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Vec<f64>>,
}

#[derive(Serialize)]
pub struct JsMesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
}

#[derive(Serialize)]
pub struct JsStats {
    pub parts_built: usize,
    pub parts_instanced: usize,
    pub parts_cached: usize,
    pub ops_built: usize,
    pub ops_cached: usize,
    pub ops_skipped: usize,
}

#[derive(Serialize)]
pub struct JsCompileResult {
    pub parts: Vec<JsPart>,
    pub errors: Vec<String>,
    pub unsupported: Vec<String>,
    /// Part-local mesh per instance key (only with `instanced` output).
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub meshes: std::collections::BTreeMap<String, JsMesh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<JsStats>,
}

/// Compile options (all optional).
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CompileOptions {
    /// Send each instanced shape once in `meshes` and leave the instanced
    /// parts' own positions empty.
    pub instanced: bool,
}

/// Mesh options (all optional; defaults match the OCCT kernel's look).
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MeshOptions {
    pub circle_segments: Option<usize>,
    pub min_edge: Option<f64>,
}

/// Surface Rust panics as readable console errors instead of `unreachable`.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

fn mesh_arrays(m: &ironstream::mesh::TriMesh) -> (Vec<f32>, Vec<u32>) {
    (
        m.verts
            .iter()
            .flat_map(|v| [v.x as f32, v.y as f32, v.z as f32])
            .collect(),
        m.tris.iter().flat_map(|t| t.map(|i| i as u32)).collect(),
    )
}

fn column_major(t: &ironstream::gp::Trsf) -> Vec<f64> {
    let m = t.m;
    vec![
        m[0][0], m[1][0], m[2][0], 0.0, m[0][1], m[1][1], m[2][1], 0.0, m[0][2], m[1][2], m[2][2],
        0.0, t.t[0], t.t[1], t.t[2], 1.0,
    ]
}

fn to_js(
    result: &CompileResult,
    compiler: Option<&Compiler>,
    opts: &CompileOptions,
) -> JsCompileResult {
    let mut meshes = std::collections::BTreeMap::new();
    let parts = result
        .parts
        .iter()
        .map(|part| {
            let inst = part.instance.as_ref();
            let shared = opts.instanced
                && inst.is_some()
                && compiler
                    .and_then(|c| c.part_local(&inst.unwrap().key))
                    .is_some();
            let (positions, indices) = if shared {
                let key = &inst.unwrap().key;
                if !meshes.contains_key(key) {
                    let local = compiler.unwrap().part_local(key).unwrap();
                    let (positions, indices) = mesh_arrays(local.mesh());
                    meshes.insert(key.clone(), JsMesh { positions, indices });
                }
                (Vec::new(), Vec::new())
            } else {
                mesh_arrays(part.solid.mesh())
            };
            JsPart {
                name: part.name.clone(),
                positions,
                indices,
                volume: part.solid.volume(),
                instance_key: inst.map(|i| i.key.clone()),
                placement: inst.map(|i| column_major(&i.placement)),
            }
        })
        .collect();
    let s = &result.stats;
    JsCompileResult {
        parts,
        errors: result.errors.clone(),
        unsupported: result.unsupported.clone(),
        meshes,
        stats: Some(JsStats {
            parts_built: s.parts_built,
            parts_instanced: s.parts_instanced,
            parts_cached: s.parts_cached,
            ops_built: s.ops_built,
            ops_cached: s.ops_cached,
            ops_skipped: s.ops_skipped,
        }),
    }
}

fn js_err(e: impl ToString) -> JsValue {
    JsValue::from_str(&e.to_string())
}

/// Compile a foundation DAG JSON string into tessellated parts (stateless).
#[wasm_bindgen]
pub fn compile(dag_json: &str) -> Result<JsValue, JsValue> {
    let result = ironstream_dag::compile_json(dag_json).map_err(|e| JsValue::from_str(&e))?;
    let js = to_js(&result, None, &CompileOptions::default());
    serde_wasm_bindgen::to_value(&js).map_err(js_err)
}

/// A long-lived compiler with caches, instancing and export (the adapter
/// contract of cadbuildr/monorepo#1248).
///
/// `compile(dagJson)` returns
/// `{ meshes: [{ key, positions, indices, volume, reused }],
///    instances: [{ name, node_id, mesh_key, transform }],
///    errors, unsupported, cache: { hits, misses, bytes } }`:
/// one part-local mesh per key (a key is the content hash of a part's
/// sub-DAG, frame excluded, so it is stable across compiles and always meshes
/// the same), one instance per part with its local -> world `transform`
/// (16 floats, column-major). A mesh whose key was in the previous result is
/// sent as `reused: true` without arrays.
#[wasm_bindgen]
pub struct Kernel {
    compiler: Compiler,
    last: Option<CompileResult>,
    sent: std::collections::HashSet<String>,
}

#[wasm_bindgen]
impl Kernel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Kernel {
        Kernel {
            compiler: Compiler::new(MeshParams::default()),
            last: None,
            sent: Default::default(),
        }
    }

    /// `{ circleSegments?, minEdge? }`. Changing them clears the caches.
    #[wasm_bindgen(js_name = setMeshOptions)]
    pub fn set_mesh_options(&mut self, options: JsValue) -> Result<(), JsValue> {
        let o: MeshOptions = if options.is_undefined() || options.is_null() {
            MeshOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options).map_err(js_err)?
        };
        let mut mp = MeshParams::default();
        if let Some(n) = o.circle_segments {
            mp.circle_segments = n.max(8);
            mp.lat_segments = (n / 2).max(4);
        }
        if let Some(e) = o.min_edge {
            mp.min_edge = e.max(0.0);
        }
        self.compiler.set_mesh_params(mp);
        self.sent.clear();
        Ok(())
    }

    pub fn compile(&mut self, dag_json: &str) -> Result<JsValue, JsValue> {
        use js_sys::{Array, Float32Array, Float64Array, Object, Reflect, Uint32Array};
        let input = ironstream_dag::dag::CompilerInputDag::from_json(dag_json).map_err(js_err)?;
        let result = self.compiler.compile(&input);
        let set = |o: &Object, k: &str, v: &JsValue| {
            Reflect::set(o, &JsValue::from_str(k), v).map(|_| ())
        };
        let identity = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let meshes = Array::new();
        let instances = Array::new();
        let mut emitted: std::collections::HashSet<String> = Default::default();
        for part in &result.parts {
            let local = part
                .instance
                .as_ref()
                .and_then(|i| self.compiler.part_local(&i.key).map(|s| (i, s)));
            let (key, solid, transform) = match local {
                Some((i, s)) => (i.key.clone(), s, column_major(&i.placement)),
                None => (
                    format!("w{:032x}", mesh_hash(part.solid.mesh())),
                    &part.solid,
                    identity.to_vec(),
                ),
            };
            if emitted.insert(key.clone()) {
                let o = Object::new();
                set(&o, "key", &JsValue::from_str(&key))?;
                set(&o, "volume", &JsValue::from_f64(solid.volume()))?;
                let reused = self.sent.contains(&key);
                set(&o, "reused", &JsValue::from_bool(reused))?;
                if !reused {
                    let (p, ix) = mesh_arrays(solid.mesh());
                    set(&o, "positions", &Float32Array::from(&p[..]).into())?;
                    set(&o, "indices", &Uint32Array::from(&ix[..]).into())?;
                }
                meshes.push(&o);
            }
            let o = Object::new();
            set(&o, "name", &JsValue::from_str(&part.name))?;
            set(&o, "node_id", &JsValue::from_str(&part.node_id))?;
            set(&o, "mesh_key", &JsValue::from_str(&key))?;
            set(&o, "transform", &Float64Array::from(&transform[..]).into())?;
            instances.push(&o);
        }
        self.sent = emitted;
        let out = Object::new();
        set(&out, "meshes", &meshes)?;
        set(&out, "instances", &instances)?;
        let strs = |v: &[String]| v.iter().map(|s| JsValue::from_str(s)).collect::<Array>();
        set(&out, "errors", &strs(&result.errors).into())?;
        set(&out, "unsupported", &strs(&result.unsupported).into())?;
        let s = &result.stats;
        let cache = Object::new();
        set(
            &cache,
            "hits",
            &JsValue::from_f64((s.parts_cached + s.parts_instanced) as f64),
        )?;
        set(&cache, "misses", &JsValue::from_f64(s.parts_built as f64))?;
        set(
            &cache,
            "bytes",
            &JsValue::from_f64(self.compiler.cache_bytes() as f64),
        )?;
        set(&cache, "ops_built", &JsValue::from_f64(s.ops_built as f64))?;
        set(
            &cache,
            "ops_cached",
            &JsValue::from_f64(s.ops_cached as f64),
        )?;
        set(&out, "cache", &cache)?;
        self.last = Some(result);
        Ok(out.into())
    }

    /// AP214 STEP of the given instances (Part node ids) of the last compile,
    /// one solid each; an empty list exports the whole model. No recompile.
    pub fn export_step(&self, node_ids: Vec<String>) -> Result<String, JsValue> {
        let r = self
            .last
            .as_ref()
            .ok_or_else(|| js_err("nothing compiled yet"))?;
        let parts: Vec<&ironstream_dag::state::PartResult> = r
            .parts
            .iter()
            .filter(|p| node_ids.is_empty() || node_ids.contains(&p.node_id))
            .collect();
        if parts.is_empty() {
            return Err(js_err("no matching parts"));
        }
        let name = if parts.len() == 1 {
            parts[0].name.clone()
        } else {
            "model".to_string()
        };
        Ok(ironstream_dag::parts_step(&parts, &name))
    }

    /// Binary STL of the given instances (empty = whole model).
    pub fn export_stl(&self, node_ids: Vec<String>) -> Result<Vec<u8>, JsValue> {
        let r = self
            .last
            .as_ref()
            .ok_or_else(|| js_err("nothing compiled yet"))?;
        let mut all = ironstream::mesh::TriMesh::new();
        for p in r
            .parts
            .iter()
            .filter(|p| node_ids.is_empty() || node_ids.contains(&p.node_id))
        {
            let m = p.solid.mesh();
            let base = all.verts.len();
            all.verts.extend_from_slice(&m.verts);
            all.tris.extend(
                m.tris
                    .iter()
                    .map(|t| [t[0] + base, t[1] + base, t[2] + base]),
            );
        }
        Ok(ironstream::mesh_io::write_binary_stl(&all))
    }

    /// Drop the caches (the next compile builds and sends everything).
    pub fn clear_cache(&mut self) {
        self.compiler.clear();
        self.sent.clear();
    }
}

/// Content fingerprint of a mesh (for parts that are not instanced).
fn mesh_hash(m: &ironstream::mesh::TriMesh) -> u128 {
    use std::hash::{Hash, Hasher};
    let mut a = std::collections::hash_map::DefaultHasher::new();
    let mut b = std::collections::hash_map::DefaultHasher::new();
    1u8.hash(&mut b);
    for v in &m.verts {
        let q = (v.x.to_bits(), v.y.to_bits(), v.z.to_bits());
        q.hash(&mut a);
        q.hash(&mut b);
    }
    m.tris.hash(&mut a);
    m.tris.hash(&mut b);
    ((a.finish() as u128) << 64) | b.finish() as u128
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}
