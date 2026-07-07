// FILE: shape_extend_data_map_of_transient_list_of_msg.rs
// occt: ShapeExtend_DataMapOfTransientListOfMsg

use std::collections::BTreeMap;

pub struct ShapeExtendDataMapOfTransientListOfMsg {
    data: BTreeMap<String, Vec<String>>,
}

impl ShapeExtendDataMapOfTransientListOfMsg {
    pub fn new() -> Self {
        ShapeExtendDataMapOfTransientListOfMsg {
            data: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: String, value: Vec<String>) {
        self.data.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<Vec<String>> {
        self.data.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        self.data.remove(key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for ShapeExtendDataMapOfTransientListOfMsg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = ShapeExtendDataMapOfTransientListOfMsg::new();
        map.bind("key".to_string(), vec!["msg".to_string()]);
        assert!(map.find("key").is_some());
    }
}
