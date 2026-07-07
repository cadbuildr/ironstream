// FILE: xsdrawiges.rs
// occt: XSDRAWIGES

//! DRAW commands for IGES file I/O.
//! Original: Draw/TKXSDRAWIGES/XSDRAWIGES/XSDRAWIGES.hxx
//!
//! Provides DRAW commands for reading, writing, and manipulating IGES models.

use std::collections::HashMap;

/// IGES command handler for DRAW.
/// Manages commands for IGES (Initial Graphics Exchange Specification) file operations.
#[derive(Clone, Debug)]
pub struct XSDRAWIGES {
    iges_commands: Vec<String>,
    iges_options: HashMap<String, String>,
}

impl XSDRAWIGES {
    /// Creates a new IGES command handler.
    pub fn new() -> Self {
        Self {
            iges_commands: Vec::new(),
            iges_options: HashMap::new(),
        }
    }

    /// Registers an IGES command.
    pub fn register_iges_command(&mut self, cmd_name: String) {
        self.iges_commands.push(cmd_name);
    }

    /// Sets an IGES export option.
    pub fn set_option(&mut self, option_name: String, option_value: String) {
        self.iges_options.insert(option_name, option_value);
    }

    /// Gets an IGES option value.
    pub fn get_option(&self, option_name: &str) -> Option<&str> {
        self.iges_options.get(option_name).map(|s| s.as_str())
    }

    /// Returns the list of registered IGES commands.
    pub fn iges_commands(&self) -> &[String] {
        &self.iges_commands
    }

    /// Returns the number of configured options.
    pub fn option_count(&self) -> usize {
        self.iges_options.len()
    }

    /// Clears all commands and options.
    pub fn clear(&mut self) {
        self.iges_commands.clear();
        self.iges_options.clear();
    }

    /// Initializes standard IGES commands.
    pub fn init_standard_iges_commands(&mut self) {
        self.iges_commands.push("read_iges".to_string());
        self.iges_commands.push("write_iges".to_string());
        self.iges_commands.push("iges_status".to_string());

        // Set default IGES options
        self.iges_options
            .insert("write_mode".to_string(), "3d".to_string());
        self.iges_options
            .insert("precision".to_string(), "1e-7".to_string());
    }
}

impl Default for XSDRAWIGES {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_iges_handler() {
        let handler = XSDRAWIGES::new();
        assert_eq!(handler.iges_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }

    #[test]
    fn test_register_iges_command() {
        let mut handler = XSDRAWIGES::new();
        handler.register_iges_command("read_iges".to_string());
        handler.register_iges_command("write_iges".to_string());
        assert_eq!(handler.iges_commands().len(), 2);
    }

    #[test]
    fn test_set_and_get_option() {
        let mut handler = XSDRAWIGES::new();
        handler.set_option("precision".to_string(), "1e-9".to_string());
        assert_eq!(handler.get_option("precision"), Some("1e-9"));
        assert_eq!(handler.get_option("missing"), None);
    }

    #[test]
    fn test_init_standard_iges_commands() {
        let mut handler = XSDRAWIGES::new();
        handler.init_standard_iges_commands();
        assert_eq!(handler.iges_commands().len(), 3);
        assert_eq!(handler.option_count(), 2);
        assert_eq!(handler.get_option("write_mode"), Some("3d"));
    }

    #[test]
    fn test_clear() {
        let mut handler = XSDRAWIGES::new();
        handler.init_standard_iges_commands();
        assert!(handler.iges_commands().len() > 0);
        handler.clear();
        assert_eq!(handler.iges_commands().len(), 0);
        assert_eq!(handler.option_count(), 0);
    }
}
