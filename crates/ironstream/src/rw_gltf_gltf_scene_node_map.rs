// FILE: rw_gltf_gltf_scene_node_map.rs
// occt: RWGltf_GltfSceneNodeMap

//! Scene node mapping for glTF.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SceneNodeMap {
    nodes: HashMap<u32, String>,
}

impl SceneNodeMap {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add(&mut self, id: u32, name: String) {
        self.nodes.insert(id, name);
    }

    pub fn get(&self, id: u32) -> Option<&str> {
        self.nodes.get(&id).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for SceneNodeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut map = SceneNodeMap::new();
        map.add(0, "Node0".to_string());
        assert!(map.get(0).is_some());
    }
}
