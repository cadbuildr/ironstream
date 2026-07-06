// FILE: xsdrawobj.rs
// occt: XSDRAWOBJ

//! DRAW commands for OBJ file I/O.
//! Original: Draw/TKXSDRAWOBJ/XSDRAWOBJ/XSDRAWOBJ.hxx
//!
//! Provides DRAW commands for reading and writing OBJ (Wavefront) mesh files.

use std::collections::HashMap;

/// OBJ command handler for DRAW.
/// Manages commands for OBJ (Wavefront) file I/O and mesh conversion.
#[derive(Clone, Debug)]
pub struct XSDRAWOBJ {
    obj_commands: Vec<String>,
    obj_options: HashMap<String, String>,
}

impl XSDRAWOBJ {
    /// Creates a new OBJ command handler.
    pub fn new() -> Self {
        Self {
            obj_commands: Vec::new(),
            obj_options: HashMap::new(),
        }
    }

    /// Registers an OBJ command.
    pub fn register_obj_command(&mut self, cmd_name: String) {
        self.obj_commands.push(cmd_name);
    }

    /// Sets an OBJ export option.
    pub fn set_option(&mut self, option_name: String, option_value: String) {
        self.obj_options.insert(option_name, option_value);
    }

    /// Gets an OBJ option value.
    pub fn get_option(&self, option_name: &str) -> Option<&str> {
        self.obj_options.get(option_name).map(|s| s.as_str())
    }

    /// Returns the list of registered OBJ commands.
    pub fn obj_commands(&self) -> &[String] {
        &self.obj_commands
    }

    /// Returns the number of configured options.
    pub fn option_count(&self) -> usize {
        self.obj_options.len()
    }

    /// Clears all commands and options.
    pub fn clear(&mut self) {
        self.obj_commands.clear();
        self.obj_options.clear();
    }

    /// Initializes standard OBJ commands.
    pub fn init_standard_obj_commands(&mut self) {
        self.obj_commands.push("read_obj".to_string());
        self.obj_commands.push("write_obj".to_string());

        // Set default OBJ options
        self.obj_options
            .insert("triangulate".to_string(), "on".to_string());
        self.obj_options
            .insert("export_normals".to_string(), "on".to_string());
        self.obj_options
            .insert("export_textures".to_string(), "off".to_string());
    }
}

impl Default for XSDRAWOBJ {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_obj_handler() {
        let handler = XSDRAWOBJ::new();
        assert_eq!(handler.obj_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }

    #[test]
    fn test_register_obj_command() {
        let mut handler = XSDRAWOBJ::new();
        handler.register_obj_command("read_obj".to_string());
        handler.register_obj_command("write_obj".to_string());
        assert_eq!(handler.obj_commands().len(), 2);
    }

    #[test]
    fn test_set_and_get_option() {
        let mut handler = XSDRAWOBJ::new();
        handler.set_option("triangulate".to_string(), "off".to_string());
        assert_eq!(handler.get_option("triangulate"), Some("off"));
        assert_eq!(handler.get_option("missing"), None);
    }

    #[test]
    fn test_init_standard_obj_commands() {
        let mut handler = XSDRAWOBJ::new();
        handler.init_standard_obj_commands();
        assert_eq!(handler.obj_commands().len(), 2);
        assert_eq!(handler.option_count(), 3);
        assert_eq!(handler.get_option("export_normals"), Some("on"));
    }

    #[test]
    fn test_clear() {
        let mut handler = XSDRAWOBJ::new();
        handler.init_standard_obj_commands();
        assert!(handler.obj_commands().len() > 0);
        handler.clear();
        assert_eq!(handler.obj_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }
}
