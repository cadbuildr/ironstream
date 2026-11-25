//! Part instancing and the cross-compile caches.
//!
//! Two content-hash keys are computed before the walk:
//!
//! - an **operation key** per node: a hash of the node's type, params and the
//!   keys of everything it depends on. Two operations with the same key
//!   produce the same world-placed solid, so an operation whose key was built
//!   by an earlier compile is reused instead of rebuilt.
//! - a **part key** per part: the same hash over the part's sub-DAG, but with
//!   the part's own frame replaced by a placeholder. Two parts with the same
//!   part key are the same shape at different placements (102 identical beads
//!   on a cuff, repeated LEGO bricks). The first is built; the others reuse its
//!   part-local solid moved by their own frame, and their operations are not
//!   built at all.
//!
//! Names, display flags, materials and colours never change geometry, so they
//! are left out of the keys (each part still reports its own name; the viewer
//! reads materials per part).

use crate::dag::{dep_id, CompilerInputDag};
use ironstream::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub type Key = u128;

/// Node types whose `name` / `display` dependencies are labels only.
const LABELLED: &[&str] = &["Frame", "Plane", "Sketch", "Part", "PartRoot", "Anchor"];
const LABEL_DEPS: &[&str] = &["name", "display"];
/// Appearance-only node types: one constant key whatever their content.
const APPEARANCE: &[&str] = &["Material", "Color"];

/// Node types that build solids: the work instancing and the caches avoid.
pub const SOLID_OPS: &[&str] = &[
    "Extrusion",
    "Hole",
    "Lathe",
    "Box",
    "Sphere",
    "Cylinder",
    "Cone",
    "Torus",
    "Mirror",
    "Scale",
    "Loft",
    "SurfaceLoft",
    "MultiSectionSweep",
    "Sweep",
    "Shell",
    "Stitch",
    "Thicken",
    "LinearPattern",
    "PolarPattern",
    "MoveBody",
    "BooleanOperation",
];

fn h128<T: Hash + ?Sized>(v: &T) -> Key {
    let mut a = std::collections::hash_map::DefaultHasher::new();
    0u8.hash(&mut a);
    v.hash(&mut a);
    let mut b = std::collections::hash_map::DefaultHasher::new();
    1u8.hash(&mut b);
    v.hash(&mut b);
    ((a.finish() as u128) << 64) | b.finish() as u128
}

struct Hashing<'a> {
    input: &'a CompilerInputDag,
    names: &'a HashMap<u32, String>,
    /// The part frame replaced by a placeholder (None: hash absolute).
    part_frame: Option<&'a str>,
    memo: HashMap<String, Key>,
    /// False once the sub-DAG reaches a frame that is not below the part frame.
    relative: bool,
    closure: Vec<String>,
}

impl<'a> Hashing<'a> {
    fn new(
        input: &'a CompilerInputDag,
        names: &'a HashMap<u32, String>,
        part_frame: Option<&'a str>,
    ) -> Self {
        Self {
            input,
            names,
            part_frame,
            memo: HashMap::new(),
            relative: true,
            closure: Vec::new(),
        }
    }

    fn key(&mut self, id: &str) -> Key {
        if Some(id) == self.part_frame {
            return h128("<part-frame>");
        }
        if let Some(k) = self.memo.get(id) {
            return *k;
        }
        let Some(node) = self.input.node(id) else {
            return h128(&("<missing>", id));
        };
        // provisional entry: breaks (invalid) cycles
        self.memo.insert(id.to_string(), h128(&("<cycle>", id)));
        self.closure.push(id.to_string());
        let tn = self.input.type_name_of(node, self.names);
        if APPEARANCE.contains(&tn.as_str()) {
            let k = h128("<appearance>");
            self.memo.insert(id.to_string(), k);
            return k;
        }
        if tn == "Frame" && dep_id(&node.deps, "top_frame").is_none() {
            self.relative = false;
        }
        let labelled = LABELLED.contains(&tn.as_str());
        let mut children: Vec<(String, Vec<Key>)> = Vec::new();
        if let Some(obj) = node.deps.as_object() {
            let mut ks: Vec<&String> = obj.keys().collect();
            ks.sort();
            for k in ks {
                if labelled && LABEL_DEPS.contains(&k.as_str()) {
                    continue;
                }
                let ids: Vec<String> = match &obj[k] {
                    serde_json::Value::String(s) => vec![s.clone()],
                    serde_json::Value::Array(a) => a
                        .iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect(),
                    _ => continue,
                };
                let keys = ids.iter().map(|d| self.key(d)).collect();
                children.push((k.clone(), keys));
            }
        }
        let params = serde_json::to_string(&node.params).unwrap_or_default();
        let k = h128(&(tn.as_str(), params.as_str(), &children));
        self.memo.insert(id.to_string(), k);
        k
    }
}

/// What the walk does with one part.
#[derive(Clone, Copy, Debug)]
pub enum Role {
    /// Build it, then remember its part-local solid under the key.
    Build(Key),
    /// Same shape as a built or cached part: place the cached solid.
    Reuse(Key),
}

#[derive(Default)]
pub struct Plan {
    pub roles: HashMap<String, Role>,
    /// Operation nodes that only reused parts need: never built.
    pub skip: HashSet<String>,
    /// Absolute content key of every solid operation.
    pub op_keys: HashMap<String, Key>,
}

pub fn plan(
    input: &CompilerInputDag,
    names: &HashMap<u32, String>,
    cached_parts: &HashSet<Key>,
) -> Plan {
    let mut plan = Plan::default();
    let order = input.topo_order();

    // absolute keys for the operation cache
    let mut abs = Hashing::new(input, names, None);
    for id in &order {
        let Some(node) = input.node(id) else { continue };
        if SOLID_OPS.contains(&input.type_name_of(node, names).as_str()) {
            let k = abs.key(id);
            plan.op_keys.insert(id.clone(), k);
        }
    }

    let mut first_built: HashSet<Key> = HashSet::new();
    let mut keep: HashSet<String> = HashSet::new();
    let mut reused: HashSet<String> = HashSet::new();
    for id in &order {
        let Some(node) = input.node(id) else { continue };
        let tn = input.type_name_of(node, names);
        if tn != "Part" && tn != "PartRoot" {
            continue;
        }
        let frame = dep_id(&node.deps, "frame");
        let mut h = Hashing::new(input, names, frame.as_deref());
        let mut children: Vec<(String, Vec<Key>)> = Vec::new();
        if let Some(obj) = node.deps.as_object() {
            let mut ks: Vec<&String> = obj.keys().collect();
            ks.sort();
            for k in ks {
                if k == "name" || k == "frame" {
                    continue;
                }
                let ids: Vec<String> = match &obj[k] {
                    serde_json::Value::String(s) => vec![s.clone()],
                    serde_json::Value::Array(a) => a
                        .iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect(),
                    _ => continue,
                };
                let keys = ids.iter().map(|d| h.key(d)).collect();
                children.push((k.clone(), keys));
            }
        }
        let key = h128(&("part", tn.as_str(), &children));
        let instanceable = frame.is_some() && h.relative;
        let role = if !instanceable {
            None
        } else if cached_parts.contains(&key) || first_built.contains(&key) {
            Some(Role::Reuse(key))
        } else {
            first_built.insert(key);
            Some(Role::Build(key))
        };
        match role {
            Some(Role::Reuse(_)) => reused.extend(h.closure),
            _ => keep.extend(h.closure),
        }
        if let Some(r) = role {
            plan.roles.insert(id.clone(), r);
        }
    }
    for id in reused {
        if keep.contains(&id) {
            continue;
        }
        let Some(node) = input.node(&id) else {
            continue;
        };
        if SOLID_OPS.contains(&input.type_name_of(node, names).as_str()) {
            plan.skip.insert(id);
        }
    }
    plan
}

/// Inverse of an affine transform (rigid frames in practice).
pub fn inverse(t: &Trsf) -> Trsf {
    let m = t.m;
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    let d = if det.abs() < 1e-300 { 1.0 } else { 1.0 / det };
    let inv = [
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * d,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * d,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * d,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * d,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * d,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * d,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * d,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * d,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * d,
        ],
    ];
    let tt = t.t;
    let t2 = [
        -(inv[0][0] * tt[0] + inv[0][1] * tt[1] + inv[0][2] * tt[2]),
        -(inv[1][0] * tt[0] + inv[1][1] * tt[1] + inv[1][2] * tt[2]),
        -(inv[2][0] * tt[0] + inv[2][1] * tt[1] + inv[2][2] * tt[2]),
    ];
    Trsf { m: inv, t: t2 }
}
