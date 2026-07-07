// FILE: bin_mdf_type_id_map.rs
// occt: BinMDF_TypeIdMap

use std::collections::HashMap;

pub struct BinmdfTypeIdMap {
    data: HashMap<String, i32>,
}

impl BinmdfTypeIdMap {
    pub fn new() -> Self {
        BinmdfTypeIdMap {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, type_name: String, type_id: i32) {
        self.data.insert(type_name, type_id);
    }

    pub fn get(&self, type_name: &str) -> Option<i32> {
        self.data.get(type_name).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BinmdfTypeIdMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut map = BinmdfTypeIdMap::new();
        map.add("Integer".to_string(), 1);
        assert_eq!(map.get("Integer"), Some(1));
    }

    #[test]
    fn test_len() {
        let mut map = BinmdfTypeIdMap::new();
        map.add("Type1".to_string(), 1);
        map.add("Type2".to_string(), 2);
        assert_eq!(map.len(), 2);
    }
}
