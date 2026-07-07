// FILE: xdedraw.rs
// occt: XDEDRAW

//! Main DRAW command registry for XDE (eXtended Data Exchange) operations.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW.hxx
//!
//! Provides the command registration interface for XDE commands in the DRAW interpreter.
//! This includes shape, color, layer, property, note, and GDT (geometric design tolerancing) commands.

/// XDEDRAW command registration registry.
/// Collects all XDE-specific DRAW commands.
#[derive(Clone, Debug)]
pub struct XDEDRAW {
    command_count: usize,
}

impl XDEDRAW {
    /// Creates a new XDEDRAW command registry.
    pub fn new() -> Self {
        Self {
            command_count: 0,
        }
    }

    /// Initializes the standard XDE command set.
    /// This would register all XDE commands (shapes, colors, layers, props, notes, GDTs).
    pub fn init_commands(&mut self) {
        // In the real implementation, this would register:
        // - Shape handling commands
        // - Color/style commands
        // - Layer management
        // - Properties
        // - Notes
        // - GDT commands
        // - View management
        self.command_count = 0;
    }

    /// Registers a new command in the DRAW interpreter.
    pub fn register_command(&mut self, _name: &str) {
        self.command_count += 1;
    }

    /// Returns the total number of registered commands.
    pub fn command_count(&self) -> usize {
        self.command_count
    }

    /// Clears all registered commands.
    pub fn clear(&mut self) {
        self.command_count = 0;
    }
}

impl Default for XDEDRAW {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_registry() {
        let registry = XDEDRAW::new();
        assert_eq!(registry.command_count(), 0);
    }

    #[test]
    fn test_register_commands() {
        let mut registry = XDEDRAW::new();
        registry.register_command("xde_shape");
        registry.register_command("xde_color");
        registry.register_command("xde_layer");
        assert_eq!(registry.command_count(), 3);
    }

    #[test]
    fn test_clear() {
        let mut registry = XDEDRAW::new();
        registry.register_command("cmd1");
        registry.register_command("cmd2");
        assert_eq!(registry.command_count(), 2);
        registry.clear();
        assert_eq!(registry.command_count(), 0);
    }

    #[test]
    fn test_default() {
        let registry = XDEDRAW::default();
        assert_eq!(registry.command_count(), 0);
    }
}
