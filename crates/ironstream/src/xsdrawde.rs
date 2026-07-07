// FILE: xsdrawde.rs
// occt: XSDRAWDE

//! DRAW commands for data exchange (STEP, IGES, etc.)
//! Original: Draw/TKXSDRAWDE/XSDRAWDE/XSDRAWDE.hxx
//!
//! Provides DRAW commands for reading and writing STEP and IGES files.

use std::collections::HashMap;

/// Data exchange command handler for DRAW.
/// Manages commands for STEP and IGES file I/O.
#[derive(Clone, Debug)]
pub struct XSDRAWDE {
    de_commands: Vec<String>,
    file_formats: HashMap<String, String>, // Format name -> Description
}

impl XSDRAWDE {
    /// Creates a new data exchange command handler.
    pub fn new() -> Self {
        Self {
            de_commands: Vec::new(),
            file_formats: HashMap::new(),
        }
    }

    /// Registers a data exchange command.
    pub fn register_de_command(&mut self, cmd_name: String) {
        self.de_commands.push(cmd_name);
    }

    /// Registers a file format. Returns true if registered.
    pub fn register_format(&mut self, format_name: String, description: String) -> bool {
        self.file_formats.insert(format_name, description).is_none()
    }

    /// Returns the list of registered DE commands.
    pub fn de_commands(&self) -> &[String] {
        &self.de_commands
    }

    /// Returns the description of a registered format.
    pub fn format_description(&self, format_name: &str) -> Option<&str> {
        self.file_formats.get(format_name).map(|s| s.as_str())
    }

    /// Returns the number of registered formats.
    pub fn format_count(&self) -> usize {
        self.file_formats.len()
    }

    /// Clears all commands and formats.
    pub fn clear(&mut self) {
        self.de_commands.clear();
        self.file_formats.clear();
    }

    /// Initializes standard data exchange commands (STEP, IGES).
    pub fn init_standard_de_commands(&mut self) {
        self.de_commands.push("read_step".to_string());
        self.de_commands.push("write_step".to_string());
        self.de_commands.push("read_iges".to_string());
        self.de_commands.push("write_iges".to_string());

        self.file_formats.insert(
            "step".to_string(),
            "STandard for the Exchange of Product data".to_string(),
        );
        self.file_formats.insert(
            "iges".to_string(),
            "Initial Graphics Exchange Specification".to_string(),
        );
    }
}

impl Default for XSDRAWDE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_de_handler() {
        let handler = XSDRAWDE::new();
        assert_eq!(handler.de_commands().len(), 0);
        assert_eq!(handler.format_count(), 0);
    }

    #[test]
    fn test_register_de_command() {
        let mut handler = XSDRAWDE::new();
        handler.register_de_command("read_step".to_string());
        handler.register_de_command("write_iges".to_string());
        assert_eq!(handler.de_commands().len(), 2);
    }

    #[test]
    fn test_register_format() {
        let mut handler = XSDRAWDE::new();
        assert!(handler.register_format("step".to_string(), "STEP format".to_string()));
        assert!(!handler.register_format("step".to_string(), "Another".to_string())); // Already exists
        assert_eq!(handler.format_count(), 1);
    }

    #[test]
    fn test_format_description() {
        let mut handler = XSDRAWDE::new();
        handler.register_format("iges".to_string(), "IGES format".to_string());
        assert_eq!(handler.format_description("iges"), Some("IGES format"));
        assert_eq!(handler.format_description("missing"), None);
    }

    #[test]
    fn test_init_standard_de_commands() {
        let mut handler = XSDRAWDE::new();
        handler.init_standard_de_commands();
        assert_eq!(handler.de_commands().len(), 4);
        assert_eq!(handler.format_count(), 2);
        assert!(handler.de_commands().iter().any(|c| c.contains("step")));
        assert!(handler.de_commands().iter().any(|c| c.contains("iges")));
    }

    #[test]
    fn test_clear() {
        let mut handler = XSDRAWDE::new();
        handler.init_standard_de_commands();
        assert!(handler.de_commands().len() > 0);
        handler.clear();
        assert_eq!(handler.de_commands().len(), 0);
        assert_eq!(handler.format_count(), 0);
    }
}
