// FILE: xdedraw_colors.rs
// occt: XDEDRAW_Colors

//! DRAW commands for color and material properties in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_Colors.hxx
//!
//! Provides commands to set, get, and manage shape colors in XDE documents.

/// Helper structure for color-related DRAW commands in XDE.
#[derive(Clone, Debug)]
pub struct XDEDRAWColors {
    color_commands: Vec<String>,
}

impl XDEDRAWColors {
    /// Creates a new color command handler.
    pub fn new() -> Self {
        Self {
            color_commands: Vec::new(),
        }
    }

    /// Registers a color-related command.
    pub fn register_color_command(&mut self, cmd_name: String) {
        self.color_commands.push(cmd_name);
    }

    /// Returns the list of registered color commands.
    pub fn color_commands(&self) -> &[String] {
        &self.color_commands
    }

    /// Clears all registered color commands.
    pub fn clear(&mut self) {
        self.color_commands.clear();
    }

    /// Returns the number of registered color commands.
    pub fn command_count(&self) -> usize {
        self.color_commands.len()
    }

    /// Helper: initializes standard color commands (set_color, get_color, etc.)
    pub fn init_standard_commands(&mut self) {
        self.color_commands.push("xde_set_color".to_string());
        self.color_commands.push("xde_get_color".to_string());
        self.color_commands.push("xde_set_transparency".to_string());
        self.color_commands.push("xde_get_transparency".to_string());
    }
}

impl Default for XDEDRAWColors {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_handler() {
        let handler = XDEDRAWColors::new();
        assert_eq!(handler.command_count(), 0);
    }

    #[test]
    fn test_register_color_command() {
        let mut handler = XDEDRAWColors::new();
        handler.register_color_command("set_color".to_string());
        handler.register_color_command("get_color".to_string());
        assert_eq!(handler.command_count(), 2);
        assert!(handler.color_commands().contains(&"set_color".to_string()));
    }

    #[test]
    fn test_init_standard_commands() {
        let mut handler = XDEDRAWColors::new();
        handler.init_standard_commands();
        assert_eq!(handler.command_count(), 4);
        assert!(handler.color_commands().iter().any(|c| c.contains("set_color")));
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWColors::new();
        handler.register_color_command("color1".to_string());
        handler.register_color_command("color2".to_string());
        assert_eq!(handler.command_count(), 2);
        handler.clear();
        assert_eq!(handler.command_count(), 0);
    }
}
