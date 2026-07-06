// FILE: xdedraw_gd_ts.rs
// occt: XDEDRAW_GDTs

//! DRAW commands for Geometric Design Tolerancing (GDT) in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_GDTs.hxx
//!
//! Provides commands to create and manipulate GDT objects (tolerances, datums, etc.).

/// GDT command handler for XDE documents.
#[derive(Clone, Debug)]
pub struct XDEDRAWGDTs {
    gdt_commands: Vec<String>,
    gdt_count: usize,
}

impl XDEDRAWGDTs {
    /// Creates a new GDT command handler.
    pub fn new() -> Self {
        Self {
            gdt_commands: Vec::new(),
            gdt_count: 0,
        }
    }

    /// Registers a GDT command.
    pub fn register_gdt_command(&mut self, cmd_name: String) {
        self.gdt_commands.push(cmd_name);
    }

    /// Creates a new GDT feature. Returns an identifier for the created GDT.
    pub fn create_gdt(&mut self, _gdt_type: &str) -> usize {
        self.gdt_count += 1;
        self.gdt_count
    }

    /// Returns the list of registered GDT commands.
    pub fn gdt_commands(&self) -> &[String] {
        &self.gdt_commands
    }

    /// Returns the total number of created GDTs.
    pub fn gdt_count(&self) -> usize {
        self.gdt_count
    }

    /// Clears all GDT commands and resets the count.
    pub fn clear(&mut self) {
        self.gdt_commands.clear();
        self.gdt_count = 0;
    }

    /// Initializes standard GDT commands.
    pub fn init_standard_gdt_commands(&mut self) {
        self.gdt_commands.push("xde_create_tolerance".to_string());
        self.gdt_commands.push("xde_create_datum".to_string());
        self.gdt_commands.push("xde_create_datum_target".to_string());
        self.gdt_commands.push("xde_get_tolerance".to_string());
    }
}

impl Default for XDEDRAWGDTs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_gdt_handler() {
        let handler = XDEDRAWGDTs::new();
        assert_eq!(handler.gdt_count(), 0);
        assert_eq!(handler.gdt_commands().len(), 0);
    }

    #[test]
    fn test_register_gdt_command() {
        let mut handler = XDEDRAWGDTs::new();
        handler.register_gdt_command("tolerance_cmd".to_string());
        handler.register_gdt_command("datum_cmd".to_string());
        assert_eq!(handler.gdt_commands().len(), 2);
    }

    #[test]
    fn test_create_gdt() {
        let mut handler = XDEDRAWGDTs::new();
        let id1 = handler.create_gdt("tolerance");
        let id2 = handler.create_gdt("datum");
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(handler.gdt_count(), 2);
    }

    #[test]
    fn test_init_standard_gdt_commands() {
        let mut handler = XDEDRAWGDTs::new();
        handler.init_standard_gdt_commands();
        assert_eq!(handler.gdt_commands().len(), 4);
        assert!(handler.gdt_commands().iter().any(|c| c.contains("tolerance")));
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWGDTs::new();
        handler.register_gdt_command("cmd".to_string());
        handler.create_gdt("type");
        assert_eq!(handler.gdt_count(), 1);
        assert_eq!(handler.gdt_commands().len(), 1);

        handler.clear();
        assert_eq!(handler.gdt_count(), 0);
        assert_eq!(handler.gdt_commands().len(), 0);
    }
}
