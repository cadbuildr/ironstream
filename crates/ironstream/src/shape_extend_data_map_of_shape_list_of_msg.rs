// FILE: shape_extend_data_map_of_shape_list_of_msg.rs
// occt: ShapeExtend_DataMapOfShapeListOfMsg

use std::collections::BTreeMap;

pub struct ShapeExtendDataMapOfShapeListOfMsg {
    data: BTreeMap<String, Vec<String>>,
}

impl ShapeExtendDataMapOfShapeListOfMsg {
    pub fn new() -> Self {
        ShapeExtendDataMapOfShapeListOfMsg {
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

impl Default for ShapeExtendDataMapOfShapeListOfMsg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = ShapeExtendDataMapOfShapeListOfMsg::new();
        map.bind("shape".to_string(), vec!["msg1".to_string()]);
        assert_eq!(map.find("shape"), Some(vec!["msg1".to_string()]));
    }
}
