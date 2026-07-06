// FILE: xsdraw_vars.rs
// occt: XSDRAW_Vars
//
// Faithful port of OCCT XSDRAW_Vars (Draw/TKXSDRAW/XSDRAW/XSDRAW_Vars.hxx),
// a namespace-like holder for XSDRAW global variables and their management
// (e.g. current shape, current assembly, Draw variable dictionary access).
// Minimal implementation models the variable registry and query interface.

use std::collections::HashMap;

/// Local helper: wrapper for a Draw variable (can be a shape, assembly, or scalar).
#[derive(Debug, Clone)]
pub enum XsdrawVarValue {
    String(String),
    Integer(i32),
    Real(f64),
}

/// Namespace-like holder for XSDRAW variables and their management.
#[derive(Debug, Default)]
pub struct XsdrawVars {
    vars: HashMap<String, XsdrawVarValue>,
}

impl XsdrawVars {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    /// Set a variable to a string value.
    pub fn set_string(&mut self, name: &str, value: &str) {
        self.vars.insert(name.to_string(), XsdrawVarValue::String(value.to_string()));
    }

    /// Set a variable to an integer value.
    pub fn set_integer(&mut self, name: &str, value: i32) {
        self.vars.insert(name.to_string(), XsdrawVarValue::Integer(value));
    }

    /// Set a variable to a real (floating-point) value.
    pub fn set_real(&mut self, name: &str, value: f64) {
        self.vars.insert(name.to_string(), XsdrawVarValue::Real(value));
    }

    /// Retrieve a variable value by name.
    pub fn get(&self, name: &str) -> Option<&XsdrawVarValue> {
        self.vars.get(name)
    }

    /// Check if a variable exists.
    pub fn exists(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    /// Clear all variables.
    pub fn clear(&mut self) {
        self.vars.clear();
    }

    /// Remove a single variable.
    pub fn remove(&mut self, name: &str) -> Option<XsdrawVarValue> {
        self.vars.remove(name)
    }

    /// List all variable names.
    pub fn names(&self) -> Vec<String> {
        self.vars.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xsdraw_vars_new() {
        let vars = XsdrawVars::new();
        assert_eq!(vars.names().len(), 0);
    }

    #[test]
    fn test_set_and_get_string() {
        let mut vars = XsdrawVars::new();
        vars.set_string("myvar", "hello");
        assert!(vars.exists("myvar"));
        match vars.get("myvar") {
            Some(XsdrawVarValue::String(s)) => assert_eq!(s, "hello"),
            _ => panic!("Expected String value"),
        }
    }

    #[test]
    fn test_set_and_get_integer() {
        let mut vars = XsdrawVars::new();
        vars.set_integer("count", 42);
        match vars.get("count") {
            Some(XsdrawVarValue::Integer(v)) => assert_eq!(*v, 42),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_set_and_get_real() {
        let mut vars = XsdrawVars::new();
        vars.set_real("pi", 3.14159);
        match vars.get("pi") {
            Some(XsdrawVarValue::Real(v)) => assert!((v - 3.14159).abs() < 1e-5),
            _ => panic!("Expected Real value"),
        }
    }

    #[test]
    fn test_remove() {
        let mut vars = XsdrawVars::new();
        vars.set_string("temp", "value");
        assert!(vars.exists("temp"));
        vars.remove("temp");
        assert!(!vars.exists("temp"));
    }

    #[test]
    fn test_clear() {
        let mut vars = XsdrawVars::new();
        vars.set_string("v1", "a");
        vars.set_integer("v2", 1);
        vars.clear();
        assert_eq!(vars.names().len(), 0);
    }

    #[test]
    fn test_names() {
        let mut vars = XsdrawVars::new();
        vars.set_string("x", "");
        vars.set_string("y", "");
        vars.set_string("z", "");
        let names = vars.names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"x".to_string()));
    }
}
