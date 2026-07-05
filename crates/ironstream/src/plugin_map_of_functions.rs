// FILE: plugin_map_of_functions.rs
// occt: Plugin_MapOfFunctions

//! Deprecated: Plugin_MapOfFunctions is a map type alias for NCollection_Map.

use std::collections::HashMap;

/// Function entry placeholder
#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    id: u32,
}

impl Function {
    pub fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Function {}

impl std::hash::Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Map of functions
#[derive(Debug, Clone)]
pub struct Map {
    functions: HashMap<u32, Function>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn add(&mut self, func: Function) -> bool {
        if self.functions.contains_key(&func.id()) {
            false
        } else {
            self.functions.insert(func.id(), func);
            true
        }
    }

    pub fn remove(&mut self, id: u32) -> bool {
        self.functions.remove(&id).is_some()
    }

    pub fn contains(&self, id: u32) -> bool {
        self.functions.contains_key(&id)
    }

    pub fn find(&self, id: u32) -> Option<&Function> {
        self.functions.get(&id)
    }

    pub fn len(&self) -> usize {
        self.functions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    pub fn clear(&mut self) {
        self.functions.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Function> {
        self.functions.values()
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

pub type PluginMapOfFunctions = Map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut map = Map::new();
        let func = Function::new("func1".to_string(), 1);
        assert!(map.add(func));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_add_duplicate() {
        let mut map = Map::new();
        let func = Function::new("func1".to_string(), 1);
        assert!(map.add(func.clone()));
        assert!(!map.add(func));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_find() {
        let mut map = Map::new();
        let func = Function::new("func1".to_string(), 42);
        map.add(func);

        assert!(map.find(42).is_some());
        assert_eq!(map.find(42).unwrap().name(), "func1");
    }

    #[test]
    fn test_contains() {
        let mut map = Map::new();
        map.add(Function::new("f".to_string(), 1));
        assert!(map.contains(1));
        assert!(!map.contains(2));
    }

    #[test]
    fn test_remove() {
        let mut map = Map::new();
        map.add(Function::new("f".to_string(), 1));
        assert!(map.remove(1));
        assert!(!map.contains(1));
    }

    #[test]
    fn test_clear() {
        let mut map = Map::new();
        map.add(Function::new("f".to_string(), 1));
        map.clear();
        assert!(map.is_empty());
    }
}
