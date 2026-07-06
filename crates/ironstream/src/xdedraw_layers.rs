// FILE: xdedraw_layers.rs
// occt: XDEDRAW_Layers

//! DRAW commands for layer management in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_Layers.hxx
//!
//! Provides commands to create, modify, and query layers in XDE documents.

use std::collections::HashMap;

/// Layer command handler for XDE documents.
#[derive(Clone, Debug)]
pub struct XDEDRAWLayers {
    layers: HashMap<String, String>,
    layer_commands: Vec<String>,
}

impl XDEDRAWLayers {
    /// Creates a new layer command handler.
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            layer_commands: Vec::new(),
        }
    }

    /// Registers a layer command.
    pub fn register_layer_command(&mut self, cmd_name: String) {
        self.layer_commands.push(cmd_name);
    }

    /// Creates a new layer with the given name. Returns true if created successfully.
    pub fn create_layer(&mut self, layer_name: String) -> bool {
        self.layers.insert(layer_name, String::new()).is_none()
    }

    /// Returns true if the layer exists.
    pub fn layer_exists(&self, layer_name: &str) -> bool {
        self.layers.contains_key(layer_name)
    }

    /// Adds a shape to a layer.
    pub fn add_shape_to_layer(&mut self, layer_name: &str, shape_ref: String) -> bool {
        if let Some(layer) = self.layers.get_mut(layer_name) {
            layer.push_str(&format!("{},", shape_ref));
            true
        } else {
            false
        }
    }

    /// Returns the list of registered layer commands.
    pub fn layer_commands(&self) -> &[String] {
        &self.layer_commands
    }

    /// Returns the number of layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Clears all layers and commands.
    pub fn clear(&mut self) {
        self.layers.clear();
        self.layer_commands.clear();
    }

    /// Initializes standard layer commands.
    pub fn init_standard_layer_commands(&mut self) {
        self.layer_commands.push("xde_create_layer".to_string());
        self.layer_commands.push("xde_get_layer".to_string());
        self.layer_commands.push("xde_add_to_layer".to_string());
        self.layer_commands.push("xde_remove_from_layer".to_string());
    }
}

impl Default for XDEDRAWLayers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_layer_handler() {
        let handler = XDEDRAWLayers::new();
        assert_eq!(handler.layer_count(), 0);
    }

    #[test]
    fn test_create_layer() {
        let mut handler = XDEDRAWLayers::new();
        assert!(handler.create_layer("layer1".to_string()));
        assert!(handler.layer_exists("layer1"));
        assert!(!handler.create_layer("layer1".to_string())); // Already exists
    }

    #[test]
    fn test_add_shape_to_layer() {
        let mut handler = XDEDRAWLayers::new();
        handler.create_layer("layer1".to_string());
        assert!(handler.add_shape_to_layer("layer1", "shape1".to_string()));
        assert!(!handler.add_shape_to_layer("nonexistent", "shape2".to_string()));
    }

    #[test]
    fn test_register_commands() {
        let mut handler = XDEDRAWLayers::new();
        handler.register_layer_command("cmd1".to_string());
        handler.register_layer_command("cmd2".to_string());
        assert_eq!(handler.layer_commands().len(), 2);
    }

    #[test]
    fn test_init_standard_layer_commands() {
        let mut handler = XDEDRAWLayers::new();
        handler.init_standard_layer_commands();
        assert_eq!(handler.layer_commands().len(), 4);
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWLayers::new();
        handler.create_layer("layer1".to_string());
        handler.register_layer_command("cmd".to_string());
        assert_eq!(handler.layer_count(), 1);
        assert_eq!(handler.layer_commands().len(), 1);

        handler.clear();
        assert_eq!(handler.layer_count(), 0);
        assert_eq!(handler.layer_commands().len(), 0);
    }
}
