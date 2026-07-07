// FILE: step_shape_loop_and_path.rs
// occt: StepShape_LoopAndPath

//! Representation of STEP entity LoopAndPath

#[derive(Clone, Debug)]
pub struct LoopAndPath {
    name: String,
    loop_obj: Option<String>,
    path: Option<String>,
    edge_list: Vec<String>,
}

impl LoopAndPath {
    /// Returns a LoopAndPath
    pub fn new() -> Self {
        LoopAndPath {
            name: String::new(),
            loop_obj: None,
            path: None,
            edge_list: Vec::new(),
        }
    }

    /// Initialize all fields from Loop and Path
    pub fn init_from_loop_path(&mut self, name: String, loop_obj: Option<String>, path: Option<String>) {
        self.name = name;
        self.loop_obj = loop_obj;
        self.path = path;
        self.edge_list = Vec::new();
    }

    /// Initialize all fields from EdgeList
    pub fn init_from_edge_list(&mut self, name: String, edge_list: Vec<String>) {
        self.name = name;
        self.edge_list = edge_list;
        self.loop_obj = None;
        self.path = None;
    }

    /// Set Loop
    pub fn set_loop(&mut self, loop_obj: Option<String>) {
        self.loop_obj = loop_obj;
    }

    /// Returns Loop
    pub fn loop_obj(&self) -> &Option<String> {
        &self.loop_obj
    }

    /// Set Path
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// Returns Path
    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    /// Set EdgeList
    pub fn set_edge_list(&mut self, edge_list: Vec<String>) {
        self.edge_list = edge_list;
    }

    /// Returns EdgeList
    pub fn edge_list(&self) -> &[String] {
        &self.edge_list
    }

    /// Returns value at index (1-based)
    pub fn edge_list_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.edge_list.len() {
            Some(&self.edge_list[num - 1])
        } else {
            None
        }
    }

    /// Returns number of edges
    pub fn nb_edge_list(&self) -> usize {
        self.edge_list.len()
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for LoopAndPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lap = LoopAndPath::new();
        assert_eq!(lap.name(), "");
        assert!(lap.loop_obj().is_none());
        assert!(lap.path().is_none());
    }

    #[test]
    fn test_init_from_loop_path() {
        let mut lap = LoopAndPath::new();
        lap.init_from_loop_path(
            "LAP1".to_string(),
            Some("loop1".to_string()),
            Some("path1".to_string()),
        );
        assert_eq!(lap.name(), "LAP1");
        assert!(lap.loop_obj().is_some());
    }

    #[test]
    fn test_init_from_edge_list() {
        let mut lap = LoopAndPath::new();
        lap.init_from_edge_list(
            "LAP2".to_string(),
            vec!["edge1".to_string(), "edge2".to_string()],
        );
        assert_eq!(lap.name(), "LAP2");
        assert_eq!(lap.nb_edge_list(), 2);
    }
}
