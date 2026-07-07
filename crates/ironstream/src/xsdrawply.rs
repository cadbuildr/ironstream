// FILE: xsdrawply.rs
// occt: XSDRAWPLY

//! DRAW commands for PLY file I/O.
//! Original: Draw/TKXSDRAWPLY/XSDRAWPLY/XSDRAWPLY.hxx
//!
//! Provides DRAW commands for reading and writing PLY (Polygon File) mesh files.

use std::collections::HashMap;

/// PLY command handler for DRAW.
/// Manages commands for PLY (Polygon File Format) file I/O and mesh conversion.
#[derive(Clone, Debug)]
pub struct XSDRAWPLY {
    ply_commands: Vec<String>,
    ply_options: HashMap<String, String>,
}

impl XSDRAWPLY {
    /// Creates a new PLY command handler.
    pub fn new() -> Self {
        Self {
            ply_commands: Vec::new(),
            ply_options: HashMap::new(),
        }
    }

    /// Registers a PLY command.
    pub fn register_ply_command(&mut self, cmd_name: String) {
        self.ply_commands.push(cmd_name);
    }

    /// Sets a PLY export option.
    pub fn set_option(&mut self, option_name: String, option_value: String) {
        self.ply_options.insert(option_name, option_value);
    }

    /// Gets a PLY option value.
    pub fn get_option(&self, option_name: &str) -> Option<&str> {
        self.ply_options.get(option_name).map(|s| s.as_str())
    }

    /// Returns the list of registered PLY commands.
    pub fn ply_commands(&self) -> &[String] {
        &self.ply_commands
    }

    /// Returns the number of configured options.
    pub fn option_count(&self) -> usize {
        self.ply_options.len()
    }

    /// Clears all commands and options.
    pub fn clear(&mut self) {
        self.ply_commands.clear();
        self.ply_options.clear();
    }

    /// Initializes standard PLY commands.
    pub fn init_standard_ply_commands(&mut self) {
        self.ply_commands.push("read_ply".to_string());
        self.ply_commands.push("write_ply".to_string());

        // Set default PLY options
        self.ply_options
            .insert("format".to_string(), "ascii".to_string());
        self.ply_options
            .insert("export_colors".to_string(), "on".to_string());
        self.ply_options
            .insert("export_normals".to_string(), "on".to_string());
    }
}

impl Default for XSDRAWPLY {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ply_handler() {
        let handler = XSDRAWPLY::new();
        assert_eq!(handler.ply_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }

    #[test]
    fn test_register_ply_command() {
        let mut handler = XSDRAWPLY::new();
        handler.register_ply_command("read_ply".to_string());
        handler.register_ply_command("write_ply".to_string());
        assert_eq!(handler.ply_commands().len(), 2);
    }

    #[test]
    fn test_set_and_get_option() {
        let mut handler = XSDRAWPLY::new();
        handler.set_option("format".to_string(), "binary".to_string());
        assert_eq!(handler.get_option("format"), Some("binary"));
        assert_eq!(handler.get_option("missing"), None);
    }

    #[test]
    fn test_init_standard_ply_commands() {
        let mut handler = XSDRAWPLY::new();
        handler.init_standard_ply_commands();
        assert_eq!(handler.ply_commands().len(), 2);
        assert_eq!(handler.option_count(), 3);
        assert_eq!(handler.get_option("format"), Some("ascii"));
    }

    #[test]
    fn test_multiple_options() {
        let mut handler = XSDRAWPLY::new();
        handler.set_option("opt1".to_string(), "val1".to_string());
        handler.set_option("opt2".to_string(), "val2".to_string());
        handler.set_option("opt3".to_string(), "val3".to_string());
        assert_eq!(handler.option_count(), 3);
    }

    #[test]
    fn test_clear() {
        let mut handler = XSDRAWPLY::new();
        handler.init_standard_ply_commands();
        assert!(handler.ply_commands().len() > 0);
        handler.clear();
        assert_eq!(handler.ply_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }
}
