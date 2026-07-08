//! Deserialization of a CADbuildr foundation DAG (the `CompilerInputDAG` wire
//! format) and topological ordering.
//!
//! The DAG is the kernel-agnostic representation produced by the foundation
//! (`show_dag(...)` on the Python side, `CompilerInputDAG` on the TS side).
//! Node type IDs are **per-DAG** — defined by `serializableNodes` — so we
//! always dispatch on the type *name*, exactly like the replicad and truck
//! adapters do.

use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};

/// One DAG node: a numeric type tag, literal params, and dependency edges.
#[derive(Debug, Clone, Deserialize)]
pub struct DagNode {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_id: u32,
    #[serde(default)]
    pub deps: serde_json::Value,
    #[serde(default)]
    pub params: serde_json::Value,
}

/// The full compiler input DAG. Mirrors `@buildr/dag-utils::CompilerInputDAG`.
/// `rootNodeId` (TS) and `root_id` (older Python) are both accepted.
#[derive(Debug, Clone, Deserialize)]
pub struct CompilerInputDag {
    #[serde(rename = "DAG")]
    pub dag: BTreeMap<String, DagNode>,
    #[serde(rename = "rootNodeId", alias = "root_id", default)]
    pub root_node_id: String,
    #[serde(rename = "serializableNodes", default)]
    pub serializable_nodes: BTreeMap<String, u32>,
    #[serde(default)]
    pub version: Option<String>,
}

impl CompilerInputDag {
    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| format!("DAG parse error: {e}"))
    }

    pub fn node(&self, id: &str) -> Option<&DagNode> {
        self.dag.get(id)
    }

    /// Inverse of `serializableNodes`: type id -> type name.
    pub fn type_names(&self) -> HashMap<u32, String> {
        self.serializable_nodes
            .iter()
            .map(|(name, id)| (*id, name.clone()))
            .collect()
    }

    pub fn type_name_of(&self, node: &DagNode, names: &HashMap<u32, String>) -> String {
        names
            .get(&node.type_id)
            .cloned()
            .unwrap_or_else(|| format!("<type {}>", node.type_id))
    }

    /// Dependency-first ordering (children before parents) via DFS post-order.
    /// Cycles are broken defensively (a node is emitted at most once).
    pub fn topo_order(&self) -> Vec<String> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        let mut order: Vec<String> = Vec::new();

        // Iterative post-order DFS to avoid recursion limits on big DAGs.
        let mut roots: Vec<String> = vec![];
        if self.dag.contains_key(&self.root_node_id) {
            roots.push(self.root_node_id.clone());
        }
        // Include every node so disconnected params still get processed.
        for id in self.dag.keys() {
            roots.push(id.clone());
        }

        for root in roots {
            if visited.contains(&root) {
                continue;
            }
            let mut stack: Vec<(String, bool)> = vec![(root, false)];
            while let Some((id, processed)) = stack.pop() {
                if processed {
                    on_stack.remove(&id);
                    if visited.insert(id.clone()) {
                        order.push(id);
                    }
                    continue;
                }
                if visited.contains(&id) || on_stack.contains(&id) {
                    continue;
                }
                on_stack.insert(id.clone());
                stack.push((id.clone(), true));
                if let Some(node) = self.dag.get(&id) {
                    for dep in dep_ids_all(&node.deps) {
                        if !visited.contains(&dep) && !on_stack.contains(&dep) {
                            stack.push((dep, false));
                        }
                    }
                }
            }
        }
        order
    }
}

/// Collect all dependency node-ids referenced by a `deps` JSON object
/// (handles both `"k": "id"` and `"k": ["id", ...]` shapes).
pub fn dep_ids_all(deps: &serde_json::Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(map) = deps.as_object() {
        for v in map.values() {
            match v {
                serde_json::Value::String(s) => out.push(s.clone()),
                serde_json::Value::Array(a) => {
                    for e in a {
                        if let Some(s) = e.as_str() {
                            out.push(s.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    out
}

/// Single dependency id for `key`, if present and scalar (or first of array).
pub fn dep_id(deps: &serde_json::Value, key: &str) -> Option<String> {
    match deps.get(key) {
        Some(serde_json::Value::String(s)) => Some(s.clone()),
        Some(serde_json::Value::Array(a)) => a.first().and_then(|e| e.as_str()).map(String::from),
        _ => None,
    }
}

/// All dependency ids for an array-valued `key`.
pub fn dep_ids(deps: &serde_json::Value, key: &str) -> Vec<String> {
    match deps.get(key) {
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        Some(serde_json::Value::Array(a)) => a
            .iter()
            .filter_map(|e| e.as_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}
