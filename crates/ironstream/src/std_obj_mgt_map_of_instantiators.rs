// FILE: std_obj_mgt_map_of_instantiators.rs
// occt: StdObjMgt_MapOfInstantiators

use std::collections::HashMap;

/// Function type for creating persistent objects
pub type Instantiator = fn() -> usize;

/// A map of type names to instantiator functions.
/// Maps ASCII type names to factory functions that create corresponding persistent objects.
pub struct StdObjMgtMapOfInstantiators {
    map: HashMap<String, Instantiator>,
}

impl StdObjMgtMapOfInstantiators {
    /// Create a new empty map of instantiators
    pub fn new() -> Self {
        StdObjMgtMapOfInstantiators {
            map: HashMap::new(),
        }
    }

    /// Bind a type name to an instantiator function
    pub fn bind(&mut self, type_name: &str, instantiator: Instantiator) {
        self.map.insert(type_name.to_string(), instantiator);
    }

    /// Find an instantiator by type name
    pub fn find(&self, type_name: &str) -> Option<Instantiator> {
        self.map.get(type_name).copied()
    }

    /// Check if a type name is registered
    pub fn contains(&self, type_name: &str) -> bool {
        self.map.contains_key(type_name)
    }

    /// Get the number of registered instantiators
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clear all instantiators
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Get all registered type names
    pub fn type_names(&self) -> Vec<&str> {
        self.map.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for StdObjMgtMapOfInstantiators {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_instantiator() -> usize {
        42
    }

    fn another_instantiator() -> usize {
        99
    }

    #[test]
    fn test_create_empty_map() {
        let map = StdObjMgtMapOfInstantiators::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        map.bind("TestType", dummy_instantiator);

        let instantiator = map.find("TestType");
        assert!(instantiator.is_some());
        assert_eq!(instantiator.unwrap()(), 42);
    }

    #[test]
    fn test_find_nonexistent() {
        let map = StdObjMgtMapOfInstantiators::new();
        assert!(map.find("NonExistent").is_none());
    }

    #[test]
    fn test_contains() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        assert!(!map.contains("TestType"));

        map.bind("TestType", dummy_instantiator);
        assert!(map.contains("TestType"));
    }

    #[test]
    fn test_multiple_bindings() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        map.bind("Type1", dummy_instantiator);
        map.bind("Type2", another_instantiator);

        assert_eq!(map.len(), 2);
        assert!(map.contains("Type1"));
        assert!(map.contains("Type2"));

        assert_eq!(map.find("Type1").unwrap()(), 42);
        assert_eq!(map.find("Type2").unwrap()(), 99);
    }

    #[test]
    fn test_clear() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        map.bind("Type1", dummy_instantiator);
        map.bind("Type2", another_instantiator);

        assert_eq!(map.len(), 2);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_type_names() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        map.bind("Type1", dummy_instantiator);
        map.bind("Type2", another_instantiator);

        let names = map.type_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Type1"));
        assert!(names.contains(&"Type2"));
    }

    #[test]
    fn test_overwrite_binding() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        map.bind("Type1", dummy_instantiator);
        assert_eq!(map.find("Type1").unwrap()(), 42);

        map.bind("Type1", another_instantiator);
        assert_eq!(map.find("Type1").unwrap()(), 99);
        assert_eq!(map.len(), 1);
    }
}
