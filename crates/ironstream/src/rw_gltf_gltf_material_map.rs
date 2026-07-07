// FILE: rw_gltf_gltf_material_map.rs
// occt: RWGltf_GltfMaterialMap

//! Material mapping for glTF.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MaterialEntry {
    id: u32,
    name: String,
}

#[derive(Debug, Clone)]
pub struct MaterialMap {
    materials: HashMap<u32, MaterialEntry>,
}

impl MaterialMap {
    pub fn new() -> Self {
        Self { materials: HashMap::new() }
    }

    pub fn add(&mut self, id: u32, name: String) {
        self.materials.insert(id, MaterialEntry { id, name });
    }

    pub fn get(&self, id: u32) -> Option<&MaterialEntry> {
        self.materials.get(&id)
    }

    pub fn len(&self) -> usize {
        self.materials.len()
    }
}

impl Default for MaterialMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut map = MaterialMap::new();
        map.add(1, "Material1".to_string());
        assert_eq!(map.len(), 1);
    }
}
