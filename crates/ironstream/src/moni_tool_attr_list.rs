// FILE: moni_tool_attr_list.rs
// occt: MoniTool_AttrList

use std::collections::HashMap;
use std::sync::Arc;

/// Records a list of attributes as Transients which can be edited and changed
pub struct MoniToolAttrList {
    attributes: HashMap<String, Arc<dyn std::any::Any>>,
}

impl MoniToolAttrList {
    pub fn new() -> Self {
        MoniToolAttrList {
            attributes: HashMap::new(),
        }
    }

    pub fn set_attribute(&mut self, name: &str, val: Arc<dyn std::any::Any>) {
        self.attributes.insert(name.to_string(), val);
    }

    pub fn remove_attribute(&mut self, name: &str) -> bool {
        self.attributes.remove(name).is_some()
    }

    pub fn attribute(&self, name: &str) -> Option<Arc<dyn std::any::Any>> {
        self.attributes.get(name).cloned()
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    pub fn set_integer_attribute(&mut self, name: &str, val: i32) {
        self.attributes.insert(name.to_string(), Arc::new(val));
    }

    pub fn integer_attribute(&self, name: &str) -> Option<i32> {
        self.attributes
            .get(name)
            .and_then(|v| (**v).downcast_ref::<i32>())
            .copied()
    }

    pub fn set_real_attribute(&mut self, name: &str, val: f64) {
        self.attributes.insert(name.to_string(), Arc::new(val));
    }

    pub fn real_attribute(&self, name: &str) -> Option<f64> {
        self.attributes
            .get(name)
            .and_then(|v| (**v).downcast_ref::<f64>())
            .copied()
    }
}

impl Default for MoniToolAttrList {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MoniToolAttrList {
    fn clone(&self) -> Self {
        MoniToolAttrList {
            attributes: self.attributes.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = MoniToolAttrList::new();
        assert!(!list.has_attribute("test"));
    }

    #[test]
    fn test_set_attribute() {
        let mut list = MoniToolAttrList::new();
        let val = Arc::new(42);
        list.set_attribute("test", val);
        assert!(list.has_attribute("test"));
    }

    #[test]
    fn test_remove_attribute() {
        let mut list = MoniToolAttrList::new();
        list.set_attribute("test", Arc::new(1));
        assert!(list.remove_attribute("test"));
        assert!(!list.has_attribute("test"));
    }

    #[test]
    fn test_integer_attribute() {
        let mut list = MoniToolAttrList::new();
        list.set_integer_attribute("int", 42);
        assert_eq!(list.integer_attribute("int"), Some(42));
    }

    #[test]
    fn test_real_attribute() {
        let mut list = MoniToolAttrList::new();
        list.set_real_attribute("real", 3.14);
        assert!(list.real_attribute("real").is_some());
    }
}
