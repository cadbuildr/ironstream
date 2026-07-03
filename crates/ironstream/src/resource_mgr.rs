// FILE: resource_mgr.rs
// occt: Resource_Manager, Resource_DataMapOfAsciiStringAsciiString

use std::collections::HashMap;

// occt: Resource_Manager
#[derive(Clone, Debug, Default)]
pub struct ResourceManager {
    pub name: String,
    pub data: HashMap<String, String>,
    pub verbose: bool,
}

impl ResourceManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), data: HashMap::new(), verbose: false }
    }

    pub fn set_value(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn value(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn integer_value(&self, key: &str) -> Option<i64> {
        self.data.get(key)?.parse().ok()
    }

    pub fn real_value(&self, key: &str) -> Option<f64> {
        self.data.get(key)?.parse().ok()
    }

    pub fn boolean_value(&self, key: &str) -> Option<bool> {
        match self.data.get(key)?.to_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        }
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn nb_resources(&self) -> usize {
        self.data.len()
    }

    pub fn keys(&self) -> Vec<&str> {
        let mut ks: Vec<&str> = self.data.keys().map(|s| s.as_str()).collect();
        ks.sort();
        ks
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn merge(&mut self, other: &ResourceManager) {
        for (k, v) in &other.data {
            self.data.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }

    pub fn override_with(&mut self, other: &ResourceManager) {
        for (k, v) in &other.data {
            self.data.insert(k.clone(), v.clone());
        }
    }
}

// occt: Resource_DataMapOfAsciiStringAsciiString
pub type ResourceDataMap = HashMap<String, String>;

pub fn resource_data_map_new() -> ResourceDataMap { HashMap::new() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_set_get() {
        let mut rm = ResourceManager::new("test");
        rm.set_value("key", "value");
        assert_eq!(rm.value("key"), Some("value"));
        assert!(rm.has("key"));
        assert!(!rm.has("other"));
    }

    #[test]
    fn resource_typed_values() {
        let mut rm = ResourceManager::new("test");
        rm.set_value("count", "42");
        rm.set_value("scale", "3.14");
        rm.set_value("flag", "true");
        assert_eq!(rm.integer_value("count"), Some(42));
        assert!((rm.real_value("scale").unwrap() - 3.14).abs() < 1e-10);
        assert_eq!(rm.boolean_value("flag"), Some(true));
    }

    #[test]
    fn resource_keys_sorted() {
        let mut rm = ResourceManager::new("test");
        rm.set_value("b", "2");
        rm.set_value("a", "1");
        let keys = rm.keys();
        assert_eq!(keys[0], "a");
        assert_eq!(keys[1], "b");
    }

    #[test]
    fn resource_merge() {
        let mut rm1 = ResourceManager::new("r1");
        rm1.set_value("x", "1");
        let mut rm2 = ResourceManager::new("r2");
        rm2.set_value("y", "2");
        rm1.merge(&rm2);
        assert!(rm1.has("y"));
    }
}
