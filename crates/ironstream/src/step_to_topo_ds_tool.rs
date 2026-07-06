// FILE: step_to_topo_ds_tool.rs
// occt: StepToTopoDS_Tool

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Local helper mirroring StepToTopoDS_PointPair (external plumbing).
/// OCCT: equality and hashing are symmetric in (P1, P2) — the pair is unordered.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq)]
pub struct StepToTopoDS_PointPair {
    p1: String,
    p2: String,
}

impl StepToTopoDS_PointPair {
    pub fn new(p1: String, p2: String) -> Self {
        StepToTopoDS_PointPair { p1, p2 }
    }

    pub fn get_point1(&self) -> &str {
        &self.p1
    }

    pub fn get_point2(&self) -> &str {
        &self.p2
    }
}

impl PartialEq for StepToTopoDS_PointPair {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2)
            || (self.p1 == other.p2 && self.p2 == other.p1)
    }
}

impl Hash for StepToTopoDS_PointPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Order-independent: hash each point separately, combine sorted (as OCCT does).
        fn hash_one(s: &str) -> u64 {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            s.hash(&mut h);
            h.finish()
        }
        let mut a = hash_one(&self.p1);
        let mut b = hash_one(&self.p2);
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        a.hash(state);
        b.hash(state);
    }
}

/// This Tool Class provides Information to build a Cas.Cad BRep from a ProSTEP Shape model.
pub struct StepToTopoDS_Tool {
    data_map: HashMap<String, String>,
    vertex_map: HashMap<String, String>,
    edge_map: HashMap<StepToTopoDS_PointPair, String>,
    compute_pc: bool,
    trans_proc: Option<String>,
    nb_c0_surf: i32,
    nb_c1_surf: i32,
    nb_c2_surf: i32,
    nb_c0_cur2: i32,
    nb_c1_cur2: i32,
    nb_c2_cur2: i32,
    nb_c0_cur3: i32,
    nb_c1_cur3: i32,
    nb_c2_cur3: i32,
}

impl StepToTopoDS_Tool {
    pub fn new() -> Self {
        StepToTopoDS_Tool {
            data_map: HashMap::new(),
            vertex_map: HashMap::new(),
            edge_map: HashMap::new(),
            compute_pc: false,
            trans_proc: None,
            nb_c0_surf: 0,
            nb_c1_surf: 0,
            nb_c2_surf: 0,
            nb_c0_cur2: 0,
            nb_c1_cur2: 0,
            nb_c2_cur2: 0,
            nb_c0_cur3: 0,
            nb_c1_cur3: 0,
            nb_c2_cur3: 0,
        }
    }

    pub fn init(&mut self, data_map: HashMap<String, String>, trans_proc: Option<String>) {
        self.data_map = data_map;
        self.trans_proc = trans_proc;
    }

    pub fn is_bound(&self, key: &str) -> bool {
        self.data_map.contains_key(key)
    }

    pub fn bind(&mut self, key: String, shape: String) {
        self.data_map.insert(key, shape);
    }

    pub fn find(&self, key: &str) -> Option<&String> {
        self.data_map.get(key)
    }

    pub fn clear_edge_map(&mut self) {
        self.edge_map.clear();
    }

    pub fn is_edge_bound(&self, pp: &StepToTopoDS_PointPair) -> bool {
        self.edge_map.contains_key(pp)
    }

    pub fn bind_edge(&mut self, pp: StepToTopoDS_PointPair, edge: String) {
        self.edge_map.insert(pp, edge);
    }

    pub fn find_edge(&self, pp: &StepToTopoDS_PointPair) -> Option<&String> {
        self.edge_map.get(pp)
    }

    pub fn clear_vertex_map(&mut self) {
        self.vertex_map.clear();
    }

    pub fn is_vertex_bound(&self, key: &str) -> bool {
        self.vertex_map.contains_key(key)
    }

    pub fn bind_vertex(&mut self, key: String, vertex: String) {
        self.vertex_map.insert(key, vertex);
    }

    pub fn find_vertex(&self, key: &str) -> Option<&String> {
        self.vertex_map.get(key)
    }

    pub fn set_compute_pcurve(&mut self, b: bool) {
        self.compute_pc = b;
    }

    pub fn compute_pcurve(&self) -> bool {
        self.compute_pc
    }

    pub fn transient_process(&self) -> Option<&String> {
        self.trans_proc.as_ref()
    }

    pub fn add_continuity_surf(&mut self) {
        self.nb_c0_surf += 1;
    }

    pub fn add_continuity_curve(&mut self) {
        self.nb_c0_cur3 += 1;
    }

    pub fn add_continuity_curve2d(&mut self) {
        self.nb_c0_cur2 += 1;
    }

    pub fn c0_surf(&self) -> i32 {
        self.nb_c0_surf
    }

    pub fn c1_surf(&self) -> i32 {
        self.nb_c1_surf
    }

    pub fn c2_surf(&self) -> i32 {
        self.nb_c2_surf
    }

    pub fn c0_cur2(&self) -> i32 {
        self.nb_c0_cur2
    }

    pub fn c1_cur2(&self) -> i32 {
        self.nb_c1_cur2
    }

    pub fn c2_cur2(&self) -> i32 {
        self.nb_c2_cur2
    }

    pub fn c0_cur3(&self) -> i32 {
        self.nb_c0_cur3
    }

    pub fn c1_cur3(&self) -> i32 {
        self.nb_c1_cur3
    }

    pub fn c2_cur3(&self) -> i32 {
        self.nb_c2_cur3
    }
}

impl Default for StepToTopoDS_Tool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = StepToTopoDS_Tool::new();
        assert!(!tool.compute_pcurve());
        assert_eq!(tool.c0_surf(), 0);
    }

    #[test]
    fn test_bind_and_find() {
        let mut tool = StepToTopoDS_Tool::new();
        tool.bind("key1".to_string(), "shape1".to_string());
        assert!(tool.is_bound("key1"));
        assert_eq!(tool.find("key1"), Some(&"shape1".to_string()));
    }

    #[test]
    fn test_vertex_operations() {
        let mut tool = StepToTopoDS_Tool::new();
        tool.bind_vertex("v1".to_string(), "vertex1".to_string());
        assert!(tool.is_vertex_bound("v1"));
        assert_eq!(tool.find_vertex("v1"), Some(&"vertex1".to_string()));
    }

    #[test]
    fn test_edge_operations() {
        let mut tool = StepToTopoDS_Tool::new();
        let pp = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        tool.bind_edge(pp.clone(), "edge1".to_string());
        assert!(tool.is_edge_bound(&pp));
        assert_eq!(tool.find_edge(&pp), Some(&"edge1".to_string()));
    }

    #[test]
    fn test_compute_pcurve() {
        let mut tool = StepToTopoDS_Tool::new();
        tool.set_compute_pcurve(true);
        assert!(tool.compute_pcurve());
    }

    #[test]
    fn test_continuity_counters() {
        let mut tool = StepToTopoDS_Tool::new();
        tool.add_continuity_surf();
        assert_eq!(tool.c0_surf(), 1);
    }

    #[test]
    fn test_clear_maps() {
        let mut tool = StepToTopoDS_Tool::new();
        let pp = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        tool.bind_edge(pp.clone(), "edge1".to_string());
        tool.clear_edge_map();
        assert!(!tool.is_edge_bound(&pp));
    }
}
