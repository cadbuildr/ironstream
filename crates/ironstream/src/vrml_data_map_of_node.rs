// FILE: vrml_data_map_of_node.rs
// occt: VrmlData_MapOfNode

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct VrmlDataMapOfNode {
    nodes: HashMap<String, String>,
}

impl VrmlDataMapOfNode {
    pub fn new() -> Self {
        VrmlDataMapOfNode {
            nodes: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, node: &str) {
        self.nodes.insert(key.to_string(), node.to_string());
    }

    pub fn find(&self, key: &str) -> Option<&str> {
        self.nodes.get(key).map(|s| s.as_str())
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}

impl Default for VrmlDataMapOfNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let map = VrmlDataMapOfNode::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_add_find() {
        let mut map = VrmlDataMapOfNode::new();
        map.add("key1", "node1");
        assert_eq!(map.find("key1"), Some("node1"));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_clear() {
        let mut map = VrmlDataMapOfNode::new();
        map.add("k", "v");
        map.clear();
        assert!(map.is_empty());
    }
}
