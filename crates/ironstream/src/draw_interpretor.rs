// FILE: draw_interpretor.rs
// occt: Draw_Interpretor

//! Encapsulates a TCL-like interpreter for defining Draw commands.

use std::collections::HashMap;

/// Command function signature for Draw commands
pub type CommandFunction = fn(&mut DrawInterpretor, &[&str]) -> i32;

/// A command callback for the interpretor
pub struct CommandCallback {
    name: String,
    func: CommandFunction,
}

/// Encapsulates an interpreter for Draw commands (TCL-like)
pub struct DrawInterpretor {
    commands: HashMap<String, CommandFunction>,
    variables: HashMap<String, String>,
}

impl DrawInterpretor {
    /// Create a new Draw interpretor
    pub fn new() -> Self {
        DrawInterpretor {
            commands: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// Add a command to the interpretor
    pub fn add_command(&mut self, name: &str, func: CommandFunction) {
        self.commands.insert(name.to_string(), func);
    }

    /// Execute a command
    pub fn execute(&mut self, name: &str, args: &[&str]) -> i32 {
        match self.commands.get(name) {
            Some(&func) => func(self, args),
            None => 1, // Command not found
        }
    }

    /// Set a variable
    pub fn set_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    /// Get a variable
    pub fn get_variable(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    /// List all commands
    pub fn commands(&self) -> Vec<&str> {
        self.commands.keys().map(|s| s.as_str()).collect()
    }

    /// Reset the interpretor
    pub fn reset(&mut self) {
        self.commands.clear();
        self.variables.clear();
    }
}

impl Default for DrawInterpretor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_command(_interp: &mut DrawInterpretor, _args: &[&str]) -> i32 {
        0
    }

    #[test]
    fn test_interpretor_creation() {
        let interp = DrawInterpretor::new();
        assert_eq!(interp.commands().len(), 0);
    }

    #[test]
    fn test_add_command() {
        let mut interp = DrawInterpretor::new();
        interp.add_command("test", test_command);
        assert_eq!(interp.commands().len(), 1);
    }

    #[test]
    fn test_execute_command() {
        let mut interp = DrawInterpretor::new();
        interp.add_command("test", test_command);
        let result = interp.execute("test", &[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_execute_nonexistent_command() {
        let mut interp = DrawInterpretor::new();
        let result = interp.execute("nonexistent", &[]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_variables() {
        let mut interp = DrawInterpretor::new();
        interp.set_variable("test_var", "test_value");
        assert_eq!(interp.get_variable("test_var"), Some("test_value"));
    }

    #[test]
    fn test_reset() {
        let mut interp = DrawInterpretor::new();
        interp.add_command("test", test_command);
        interp.set_variable("var", "value");
        interp.reset();
        assert_eq!(interp.commands().len(), 0);
        assert!(interp.get_variable("var").is_none());
    }
}
