// FILE: interface_statics.rs
// occt: Interface_Statics

use std::collections::HashMap;
use std::sync::Arc;

/// Manages a collection of static variables
pub struct InterfaceStatics {
    statics: HashMap<String, Arc<InterfaceStaticItem>>,
}

pub struct InterfaceStaticItem {
    family: String,
    name: String,
    value: String,
}

impl InterfaceStatics {
    pub fn new() -> Self {
        InterfaceStatics {
            statics: HashMap::new(),
        }
    }

    pub fn add(&mut self, family: &str, name: &str, value: &str) {
        let key = format!("{}.{}", family, name);
        let item = Arc::new(InterfaceStaticItem {
            family: family.to_string(),
            name: name.to_string(),
            value: value.to_string(),
        });
        self.statics.insert(key, item);
    }

    pub fn get(&self, family: &str, name: &str) -> Option<String> {
        let key = format!("{}.{}", family, name);
        self.statics.get(&key).map(|item| item.value.clone())
    }

    pub fn set(&mut self, family: &str, name: &str, value: &str) {
        self.add(family, name, value);
    }

    pub fn count(&self) -> usize {
        self.statics.len()
    }
}

impl Default for InterfaceStatics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let statics = InterfaceStatics::new();
        assert_eq!(statics.count(), 0);
    }

    #[test]
    fn test_add_get() {
        let mut statics = InterfaceStatics::new();
        statics.add("fam", "name", "value");
        assert_eq!(statics.get("fam", "name"), Some("value".to_string()));
    }

    #[test]
    fn test_set() {
        let mut statics = InterfaceStatics::new();
        statics.set("fam", "name", "val1");
        statics.set("fam", "name", "val2");
        assert_eq!(statics.get("fam", "name"), Some("val2".to_string()));
    }

    #[test]
    fn test_count() {
        let mut statics = InterfaceStatics::new();
        statics.add("fam1", "name1", "val");
        statics.add("fam2", "name2", "val");
        assert_eq!(statics.count(), 2);
    }
}
