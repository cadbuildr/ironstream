// FILE: xdedraw_shapes.rs
// occt: XDEDRAW_Shapes

//! DRAW commands for shape management in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_Shapes.hxx
//!
//! Provides commands to add, retrieve, and manipulate shapes in XDE documents.

use std::collections::HashMap;

/// Shape command handler for XDE documents.
#[derive(Clone, Debug)]
pub struct XDEDRAWShapes {
    shapes: HashMap<String, String>, // Shape name -> Shape reference
    shape_commands: Vec<String>,
}

impl XDEDRAWShapes {
    /// Creates a new shape command handler.
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            shape_commands: Vec::new(),
        }
    }

    /// Registers a shape command.
    pub fn register_shape_command(&mut self, cmd_name: String) {
        self.shape_commands.push(cmd_name);
    }

    /// Adds a shape to the document. Returns true if added successfully.
    pub fn add_shape(&mut self, shape_name: String, shape_ref: String) -> bool {
        self.shapes.insert(shape_name, shape_ref).is_none()
    }

    /// Retrieves a shape by name.
    pub fn get_shape(&self, shape_name: &str) -> Option<&str> {
        self.shapes.get(shape_name).map(|s| s.as_str())
    }

    /// Checks if a shape exists.
    pub fn shape_exists(&self, shape_name: &str) -> bool {
        self.shapes.contains_key(shape_name)
    }

    /// Removes a shape by name. Returns true if it existed.
    pub fn remove_shape(&mut self, shape_name: &str) -> bool {
        self.shapes.remove(shape_name).is_some()
    }

    /// Returns the list of registered shape commands.
    pub fn shape_commands(&self) -> &[String] {
        &self.shape_commands
    }

    /// Returns the number of shapes.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Lists all shape names.
    pub fn list_shapes(&self) -> Vec<&str> {
        self.shapes.keys().map(|s| s.as_str()).collect()
    }

    /// Clears all shapes and commands.
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.shape_commands.clear();
    }

    /// Initializes standard shape commands.
    pub fn init_standard_shape_commands(&mut self) {
        self.shape_commands.push("xde_add_shape".to_string());
        self.shape_commands.push("xde_get_shape".to_string());
        self.shape_commands.push("xde_remove_shape".to_string());
        self.shape_commands.push("xde_list_shapes".to_string());
    }
}

impl Default for XDEDRAWShapes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shape_handler() {
        let handler = XDEDRAWShapes::new();
        assert_eq!(handler.shape_count(), 0);
    }

    #[test]
    fn test_add_shape() {
        let mut handler = XDEDRAWShapes::new();
        assert!(handler.add_shape("box".to_string(), "box_ref".to_string()));
        assert!(handler.shape_exists("box"));
        assert!(!handler.add_shape("box".to_string(), "other_ref".to_string())); // Already exists
    }

    #[test]
    fn test_get_shape() {
        let mut handler = XDEDRAWShapes::new();
        handler.add_shape("cylinder".to_string(), "cyl_ref".to_string());
        assert_eq!(handler.get_shape("cylinder"), Some("cyl_ref"));
        assert_eq!(handler.get_shape("missing"), None);
    }

    #[test]
    fn test_remove_shape() {
        let mut handler = XDEDRAWShapes::new();
        handler.add_shape("sphere".to_string(), "sph_ref".to_string());
        assert!(handler.remove_shape("sphere"));
        assert!(!handler.shape_exists("sphere"));
        assert!(!handler.remove_shape("sphere")); // Already removed
    }

    #[test]
    fn test_list_shapes() {
        let mut handler = XDEDRAWShapes::new();
        handler.add_shape("shape1".to_string(), "ref1".to_string());
        handler.add_shape("shape2".to_string(), "ref2".to_string());
        handler.add_shape("shape3".to_string(), "ref3".to_string());

        let list = handler.list_shapes();
        assert_eq!(list.len(), 3);
        assert!(list.contains(&"shape1"));
        assert!(list.contains(&"shape2"));
        assert!(list.contains(&"shape3"));
    }

    #[test]
    fn test_register_commands() {
        let mut handler = XDEDRAWShapes::new();
        handler.register_shape_command("cmd1".to_string());
        handler.register_shape_command("cmd2".to_string());
        assert_eq!(handler.shape_commands().len(), 2);
    }

    #[test]
    fn test_init_standard_shape_commands() {
        let mut handler = XDEDRAWShapes::new();
        handler.init_standard_shape_commands();
        assert_eq!(handler.shape_commands().len(), 4);
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWShapes::new();
        handler.add_shape("s".to_string(), "ref".to_string());
        handler.register_shape_command("cmd".to_string());
        assert_eq!(handler.shape_count(), 1);
        handler.clear();
        assert_eq!(handler.shape_count(), 0);
        assert_eq!(handler.shape_commands().len(), 0);
    }
}
