// FILE: interface_static.rs
// occt: Interface_Static

use std::sync::Arc;

/// Manages meaningful static variables used as "global" parameters
pub struct InterfaceStatic {
    family: String,
    name: String,
    value: String,
    param_type: i32,
    wildcard: Option<Arc<InterfaceStatic>>,
}

impl InterfaceStatic {
    pub fn new(family: &str, name: &str, param_type: i32, init: &str) -> Self {
        InterfaceStatic {
            family: family.to_string(),
            name: name.to_string(),
            value: init.to_string(),
            param_type,
            wildcard: None,
        }
    }

    pub fn from_other(family: &str, name: &str, other: &InterfaceStatic) -> Self {
        InterfaceStatic {
            family: family.to_string(),
            name: name.to_string(),
            value: other.value.clone(),
            param_type: other.param_type,
            wildcard: None,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn set_wild(&mut self, wildcard: Arc<InterfaceStatic>) {
        self.wildcard = Some(wildcard);
    }

    pub fn wild(&self) -> Option<Arc<InterfaceStatic>> {
        self.wildcard.clone()
    }

    pub fn int_value(&self) -> Option<i32> {
        self.value.parse().ok()
    }

    pub fn real_value(&self) -> Option<f64> {
        self.value.parse().ok()
    }
}

impl Default for InterfaceStatic {
    fn default() -> Self {
        InterfaceStatic {
            family: String::new(),
            name: String::new(),
            value: String::new(),
            param_type: 0,
            wildcard: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let stat = InterfaceStatic::new("test", "param", 0, "value");
        assert_eq!(stat.family(), "test");
        assert_eq!(stat.name(), "param");
        assert_eq!(stat.value(), "value");
    }

    #[test]
    fn test_int_value() {
        let stat = InterfaceStatic::new("fam", "name", 1, "42");
        assert_eq!(stat.int_value(), Some(42));
    }

    #[test]
    fn test_real_value() {
        let stat = InterfaceStatic::new("fam", "name", 2, "3.14");
        assert!(stat.real_value().is_some());
    }

    #[test]
    fn test_wild() {
        let mut stat1 = InterfaceStatic::new("fam", "name1", 0, "val1");
        let stat2 = Arc::new(InterfaceStatic::new("fam", "name2", 0, "val2"));
        stat1.set_wild(stat2.clone());
        assert!(stat1.wild().is_some());
    }
}
