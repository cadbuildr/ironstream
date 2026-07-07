// FILE: xdedraw_props.rs
// occt: XDEDRAW_Props

//! DRAW commands for property management in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_Props.hxx
//!
//! Provides commands to set, get, and modify shape properties.

use std::collections::HashMap;

/// Property command handler for XDE documents.
#[derive(Clone, Debug)]
pub struct XDEDRAWProps {
    properties: HashMap<String, HashMap<String, String>>, // Shape -> (Property -> Value)
    prop_commands: Vec<String>,
}

impl XDEDRAWProps {
    /// Creates a new property command handler.
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prop_commands: Vec::new(),
        }
    }

    /// Registers a property command.
    pub fn register_prop_command(&mut self, cmd_name: String) {
        self.prop_commands.push(cmd_name);
    }

    /// Sets a property for a shape. Returns true if set.
    pub fn set_property(&mut self, shape_ref: String, prop_name: String, prop_value: String) -> bool {
        self.properties
            .entry(shape_ref)
            .or_insert_with(HashMap::new)
            .insert(prop_name, prop_value);
        true
    }

    /// Gets a property value for a shape.
    pub fn get_property(&self, shape_ref: &str, prop_name: &str) -> Option<&str> {
        self.properties
            .get(shape_ref)
            .and_then(|props| props.get(prop_name))
            .map(|s| s.as_str())
    }

    /// Removes a property from a shape. Returns true if it existed.
    pub fn remove_property(&mut self, shape_ref: &str, prop_name: &str) -> bool {
        if let Some(props) = self.properties.get_mut(shape_ref) {
            props.remove(prop_name).is_some()
        } else {
            false
        }
    }

    /// Returns the list of registered property commands.
    pub fn prop_commands(&self) -> &[String] {
        &self.prop_commands
    }

    /// Returns the number of shapes with properties.
    pub fn shape_count(&self) -> usize {
        self.properties.len()
    }

    /// Clears all properties and commands.
    pub fn clear(&mut self) {
        self.properties.clear();
        self.prop_commands.clear();
    }

    /// Initializes standard property commands.
    pub fn init_standard_prop_commands(&mut self) {
        self.prop_commands.push("xde_set_property".to_string());
        self.prop_commands.push("xde_get_property".to_string());
        self.prop_commands.push("xde_remove_property".to_string());
        self.prop_commands.push("xde_list_properties".to_string());
    }
}

impl Default for XDEDRAWProps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_prop_handler() {
        let handler = XDEDRAWProps::new();
        assert_eq!(handler.shape_count(), 0);
    }

    #[test]
    fn test_set_property() {
        let mut handler = XDEDRAWProps::new();
        assert!(handler.set_property(
            "shape1".to_string(),
            "material".to_string(),
            "steel".to_string()
        ));
        assert_eq!(handler.shape_count(), 1);
    }

    #[test]
    fn test_get_property() {
        let mut handler = XDEDRAWProps::new();
        handler.set_property(
            "box".to_string(),
            "color".to_string(),
            "red".to_string(),
        );
        assert_eq!(handler.get_property("box", "color"), Some("red"));
        assert_eq!(handler.get_property("box", "missing"), None);
    }

    #[test]
    fn test_remove_property() {
        let mut handler = XDEDRAWProps::new();
        handler.set_property(
            "shape".to_string(),
            "prop".to_string(),
            "value".to_string(),
        );
        assert!(handler.remove_property("shape", "prop"));
        assert_eq!(handler.get_property("shape", "prop"), None);
        assert!(!handler.remove_property("shape", "nonexistent"));
    }

    #[test]
    fn test_multiple_properties() {
        let mut handler = XDEDRAWProps::new();
        handler.set_property("s1".to_string(), "p1".to_string(), "v1".to_string());
        handler.set_property("s1".to_string(), "p2".to_string(), "v2".to_string());
        handler.set_property("s2".to_string(), "p1".to_string(), "v3".to_string());
        assert_eq!(handler.shape_count(), 2);
        assert_eq!(handler.get_property("s1", "p1"), Some("v1"));
        assert_eq!(handler.get_property("s1", "p2"), Some("v2"));
    }

    #[test]
    fn test_register_commands() {
        let mut handler = XDEDRAWProps::new();
        handler.register_prop_command("cmd1".to_string());
        assert_eq!(handler.prop_commands().len(), 1);
    }

    #[test]
    fn test_init_standard_prop_commands() {
        let mut handler = XDEDRAWProps::new();
        handler.init_standard_prop_commands();
        assert_eq!(handler.prop_commands().len(), 4);
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWProps::new();
        handler.set_property("s".to_string(), "p".to_string(), "v".to_string());
        handler.register_prop_command("cmd".to_string());
        handler.clear();
        assert_eq!(handler.shape_count(), 0);
        assert_eq!(handler.prop_commands().len(), 0);
    }
}
