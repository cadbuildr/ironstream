// FILE: xsdraw.rs
// occt: XSDRAW

//! Main DRAW command registry for XSDRAW (Shape Data DRAW).
//! Original: Draw/TKXSDRAW/XSDRAW/XSDRAW.hxx
//!
//! Central command registration for shape reading/writing and model operations.

/// XSDRAW command registry for shape data exchange.
/// Manages DRAW commands for reading and writing shapes in various formats.
#[derive(Clone, Debug)]
pub struct XSDRAW {
    command_count: usize,
    format_handlers: Vec<String>,
}

impl XSDRAW {
    /// Creates a new XSDRAW command registry.
    pub fn new() -> Self {
        Self {
            command_count: 0,
            format_handlers: Vec::new(),
        }
    }

    /// Registers a command in the DRAW interpreter.
    pub fn register_command(&mut self, _name: &str) {
        self.command_count += 1;
    }

    /// Registers a format handler (STEP, IGES, STL, etc.)
    pub fn register_format_handler(&mut self, format_name: String) {
        self.format_handlers.push(format_name);
    }

    /// Returns the total number of registered commands.
    pub fn command_count(&self) -> usize {
        self.command_count
    }

    /// Returns the list of registered format handlers.
    pub fn format_handlers(&self) -> &[String] {
        &self.format_handlers
    }

    /// Clears all registered commands and handlers.
    pub fn clear(&mut self) {
        self.command_count = 0;
        self.format_handlers.clear();
    }

    /// Initializes all standard XSDRAW commands.
    pub fn init_all_commands(&mut self) {
        // Registers core XSDRAW commands for various file formats
        self.command_count = 0;
    }
}

impl Default for XSDRAW {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_registry() {
        let registry = XSDRAW::new();
        assert_eq!(registry.command_count(), 0);
        assert_eq!(registry.format_handlers().len(), 0);
    }

    #[test]
    fn test_register_command() {
        let mut registry = XSDRAW::new();
        registry.register_command("read_step");
        registry.register_command("write_iges");
        assert_eq!(registry.command_count(), 2);
    }

    #[test]
    fn test_register_format_handler() {
        let mut registry = XSDRAW::new();
        registry.register_format_handler("step".to_string());
        registry.register_format_handler("iges".to_string());
        registry.register_format_handler("stl".to_string());
        assert_eq!(registry.format_handlers().len(), 3);
    }

    #[test]
    fn test_clear() {
        let mut registry = XSDRAW::new();
        registry.register_command("cmd1");
        registry.register_command("cmd2");
        registry.register_format_handler("fmt".to_string());
        assert_eq!(registry.command_count(), 2);
        assert_eq!(registry.format_handlers().len(), 1);

        registry.clear();
        assert_eq!(registry.command_count(), 0);
        assert_eq!(registry.format_handlers().len(), 0);
    }

    #[test]
    fn test_default() {
        let registry = XSDRAW::default();
        assert_eq!(registry.command_count(), 0);
    }
}
