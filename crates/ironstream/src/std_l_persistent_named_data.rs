// FILE: std_l_persistent_named_data.rs
// occt: StdLPersistent_NamedData

use std::collections::HashMap;

/// Persistent named data with mixed types
pub struct StdLPersistentNamedData {
    ints: HashMap<String, i32>,
    reals: HashMap<String, f64>,
    strings: HashMap<String, String>,
}

impl StdLPersistentNamedData {
    /// Create empty named data
    pub fn new() -> Self {
        StdLPersistentNamedData {
            ints: HashMap::new(),
            reals: HashMap::new(),
            strings: HashMap::new(),
        }
    }

    /// Get integer value
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.ints.get(key).copied()
    }

    /// Set integer value
    pub fn set_int(&mut self, key: &str, value: i32) {
        self.ints.insert(key.to_string(), value);
    }

    /// Get real value
    pub fn get_real(&self, key: &str) -> Option<f64> {
        self.reals.get(key).copied()
    }

    /// Set real value
    pub fn set_real(&mut self, key: &str, value: f64) {
        self.reals.insert(key.to_string(), value);
    }

    /// Get string value
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.strings.get(key).map(|s| s.as_str())
    }

    /// Set string value
    pub fn set_string(&mut self, key: &str, value: &str) {
        self.strings.insert(key.to_string(), value.to_string());
    }
}

impl Default for StdLPersistentNamedData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let data = StdLPersistentNamedData::new();
        assert!(data.ints.is_empty());
    }

    #[test]
    fn test_int_values() {
        let mut data = StdLPersistentNamedData::new();
        data.set_int("count", 42);
        assert_eq!(data.get_int("count"), Some(42));
    }

    #[test]
    fn test_real_values() {
        let mut data = StdLPersistentNamedData::new();
        data.set_real("pi", 3.14159);
        assert_eq!(data.get_real("pi"), Some(3.14159));
    }

    #[test]
    fn test_string_values() {
        let mut data = StdLPersistentNamedData::new();
        data.set_string("name", "test");
        assert_eq!(data.get_string("name"), Some("test"));
    }
}
