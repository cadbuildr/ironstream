// FILE: step_to_topo_ds_nm_tool.rs
// occt: StepToTopoDS_NMTool

use std::collections::HashMap;

/// Provides data to process non-manifold topology when reading from STEP.
pub struct StepToTopoDS_NMTool {
    // Using simple data structures for maps since we don't have full OCCT dependencies
    ri_map: HashMap<String, String>,
    ri_names_map: HashMap<String, String>,
    nm_edges: Vec<String>,
    ideas_case: bool,
    active_flag: bool,
}

impl StepToTopoDS_NMTool {
    pub fn new() -> Self {
        StepToTopoDS_NMTool {
            ri_map: HashMap::new(),
            ri_names_map: HashMap::new(),
            nm_edges: Vec::new(),
            ideas_case: false,
            active_flag: false,
        }
    }

    pub fn init(&mut self, ri_map: HashMap<String, String>, ri_names_map: HashMap<String, String>) {
        self.ri_map = ri_map;
        self.ri_names_map = ri_names_map;
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.active_flag = is_active;
    }

    pub fn is_active(&self) -> bool {
        self.active_flag
    }

    pub fn clean_up(&mut self) {
        self.ri_map.clear();
        self.ri_names_map.clear();
    }

    pub fn is_bound_ri(&self, ri_key: &str) -> bool {
        self.ri_map.contains_key(ri_key)
    }

    pub fn is_bound_name(&self, ri_name: &str) -> bool {
        self.ri_names_map.contains_key(ri_name)
    }

    pub fn bind_ri(&mut self, ri_key: String, shape: String) {
        self.ri_map.insert(ri_key, shape);
    }

    pub fn bind_name(&mut self, ri_name: String, shape: String) {
        self.ri_names_map.insert(ri_name, shape);
    }

    pub fn find_ri(&self, ri_key: &str) -> Option<&String> {
        self.ri_map.get(ri_key)
    }

    pub fn find_name(&self, ri_name: &str) -> Option<&String> {
        self.ri_names_map.get(ri_name)
    }

    pub fn register_nm_edge(&mut self, edge: String) {
        if !self.is_edge_registered_as_nm(&edge) {
            self.nm_edges.push(edge);
        }
    }

    pub fn is_suspected_as_closing(&self, base_shell: &str, suspected_shell: &str) -> bool {
        self.is_pure_nm_shell(suspected_shell) && self.is_adjacent_shell(base_shell, suspected_shell)
    }

    pub fn set_ideas_case(&mut self, ideas_case: bool) {
        self.ideas_case = ideas_case;
    }

    pub fn is_ideas_case(&self) -> bool {
        self.ideas_case
    }

    pub fn is_pure_nm_shell(&self, shell: &str) -> bool {
        // TODO: Actual shell exploration logic requires full TopoDS shape support
        true
    }

    fn is_edge_registered_as_nm(&self, edge: &str) -> bool {
        self.nm_edges.iter().any(|e| e == edge)
    }

    fn is_adjacent_shell(&self, shell_a: &str, shell_b: &str) -> bool {
        if shell_a == shell_b {
            return false;
        }
        // TODO: Actual adjacency check requires full TopoDS shape support
        false
    }
}

impl Default for StepToTopoDS_NMTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = StepToTopoDS_NMTool::new();
        assert!(!tool.is_active());
        assert!(!tool.is_ideas_case());
    }

    #[test]
    fn test_set_active() {
        let mut tool = StepToTopoDS_NMTool::new();
        tool.set_active(true);
        assert!(tool.is_active());
        tool.set_active(false);
        assert!(!tool.is_active());
    }

    #[test]
    fn test_bind_and_find_ri() {
        let mut tool = StepToTopoDS_NMTool::new();
        tool.bind_ri("key1".to_string(), "shape1".to_string());
        assert!(tool.is_bound_ri("key1"));
        assert_eq!(tool.find_ri("key1"), Some(&"shape1".to_string()));
        assert!(!tool.is_bound_ri("key2"));
    }

    #[test]
    fn test_bind_and_find_name() {
        let mut tool = StepToTopoDS_NMTool::new();
        tool.bind_name("name1".to_string(), "shape1".to_string());
        assert!(tool.is_bound_name("name1"));
        assert_eq!(tool.find_name("name1"), Some(&"shape1".to_string()));
    }

    #[test]
    fn test_register_nm_edge() {
        let mut tool = StepToTopoDS_NMTool::new();
        tool.register_nm_edge("edge1".to_string());
        assert!(tool.is_edge_registered_as_nm("edge1"));
        // Duplicate should not add twice
        tool.register_nm_edge("edge1".to_string());
        assert_eq!(tool.nm_edges.len(), 1);
    }

    #[test]
    fn test_clean_up() {
        let mut tool = StepToTopoDS_NMTool::new();
        tool.bind_ri("key1".to_string(), "shape1".to_string());
        tool.bind_name("name1".to_string(), "shape1".to_string());
        tool.clean_up();
        assert!(!tool.is_bound_ri("key1"));
        assert!(!tool.is_bound_name("name1"));
    }

    #[test]
    fn test_ideas_case() {
        let mut tool = StepToTopoDS_NMTool::new();
        tool.set_ideas_case(true);
        assert!(tool.is_ideas_case());
        tool.set_ideas_case(false);
        assert!(!tool.is_ideas_case());
    }
}
