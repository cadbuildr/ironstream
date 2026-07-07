// FILE: xsdrawgltf.rs
// occt: XSDRAWGLTF

//! DRAW commands for glTF file I/O.
//! Original: Draw/TKXSDRAWGLTF/XSDRAWGLTF/XSDRAWGLTF.hxx
//!
//! Provides DRAW commands for reading and writing glTF and GLB files.

use std::collections::HashMap;

/// glTF command handler for DRAW.
/// Manages commands for glTF and GLB file I/O and mesh conversion.
#[derive(Clone, Debug)]
pub struct XSDRAWGLTF {
    gltf_commands: Vec<String>,
    gltf_options: HashMap<String, String>,
}

impl XSDRAWGLTF {
    /// Creates a new glTF command handler.
    pub fn new() -> Self {
        Self {
            gltf_commands: Vec::new(),
            gltf_options: HashMap::new(),
        }
    }

    /// Registers a glTF command.
    pub fn register_gltf_command(&mut self, cmd_name: String) {
        self.gltf_commands.push(cmd_name);
    }

    /// Sets a glTF export option.
    pub fn set_option(&mut self, option_name: String, option_value: String) {
        self.gltf_options.insert(option_name, option_value);
    }

    /// Gets a glTF option value.
    pub fn get_option(&self, option_name: &str) -> Option<&str> {
        self.gltf_options.get(option_name).map(|s| s.as_str())
    }

    /// Returns the list of registered glTF commands.
    pub fn gltf_commands(&self) -> &[String] {
        &self.gltf_commands
    }

    /// Returns the number of configured options.
    pub fn option_count(&self) -> usize {
        self.gltf_options.len()
    }

    /// Clears all commands and options.
    pub fn clear(&mut self) {
        self.gltf_commands.clear();
        self.gltf_options.clear();
    }

    /// Initializes standard glTF commands.
    pub fn init_standard_gltf_commands(&mut self) {
        self.gltf_commands.push("read_gltf".to_string());
        self.gltf_commands.push("write_gltf".to_string());
        self.gltf_commands.push("read_glb".to_string());
        self.gltf_commands.push("write_glb".to_string());

        // Set default options
        self.gltf_options
            .insert("export_normals".to_string(), "on".to_string());
        self.gltf_options
            .insert("export_colors".to_string(), "on".to_string());
        self.gltf_options
            .insert("export_uv".to_string(), "off".to_string());
    }
}

impl Default for XSDRAWGLTF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_gltf_handler() {
        let handler = XSDRAWGLTF::new();
        assert_eq!(handler.gltf_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }

    #[test]
    fn test_register_gltf_command() {
        let mut handler = XSDRAWGLTF::new();
        handler.register_gltf_command("read_gltf".to_string());
        handler.register_gltf_command("write_glb".to_string());
        assert_eq!(handler.gltf_commands().len(), 2);
    }

    #[test]
    fn test_set_and_get_option() {
        let mut handler = XSDRAWGLTF::new();
        handler.set_option("quality".to_string(), "high".to_string());
        assert_eq!(handler.get_option("quality"), Some("high"));
        assert_eq!(handler.get_option("missing"), None);
    }

    #[test]
    fn test_init_standard_gltf_commands() {
        let mut handler = XSDRAWGLTF::new();
        handler.init_standard_gltf_commands();
        assert_eq!(handler.gltf_commands().len(), 4);
        assert_eq!(handler.option_count(), 3);
        assert_eq!(handler.get_option("export_normals"), Some("on"));
    }

    #[test]
    fn test_option_count() {
        let mut handler = XSDRAWGLTF::new();
        assert_eq!(handler.option_count(), 0);
        handler.set_option("opt1".to_string(), "val1".to_string());
        handler.set_option("opt2".to_string(), "val2".to_string());
        assert_eq!(handler.option_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut handler = XSDRAWGLTF::new();
        handler.init_standard_gltf_commands();
        assert!(handler.gltf_commands().len() > 0);
        handler.clear();
        assert_eq!(handler.gltf_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }
}
