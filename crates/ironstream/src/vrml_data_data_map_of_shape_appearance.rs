// FILE: vrml_data_data_map_of_shape_appearance.rs
// occt: VrmlData_DataMapOfShapeAppearance

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct VrmlDataDataMapOfShapeAppearance {
    map: HashMap<u32, String>,
}

impl VrmlDataDataMapOfShapeAppearance {
    pub fn new() -> Self {
        VrmlDataDataMapOfShapeAppearance {
            map: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape_id: u32, appearance: &str) {
        self.map.insert(shape_id, appearance.to_string());
    }

    pub fn find(&self, shape_id: u32) -> Option<&str> {
        self.map.get(&shape_id).map(|s| s.as_str())
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }
}

impl Default for VrmlDataDataMapOfShapeAppearance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let map = VrmlDataDataMapOfShapeAppearance::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_bind_find() {
        let mut map = VrmlDataDataMapOfShapeAppearance::new();
        map.bind(1, "material");
        assert_eq!(map.find(1), Some("material"));
        assert_eq!(map.size(), 1);
    }
}
