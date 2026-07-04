// FILE: if_select_activator.rs
// occt: IFSelect_Activator

use std::collections::HashMap;

/// Return status for command execution
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnStatus {
    RetVoid = 0,
    RetDone = 1,
    RetError = -1,
    RetFail = -2,
    RetStop = -3,
}

/// Base class for handling command activators in a session context
#[derive(Clone, Debug)]
pub struct IfSelectActivator {
    group: String,
    file: String,
    commands: HashMap<String, i32>,
}

impl IfSelectActivator {
    /// Creates a new activator with default group
    pub fn new() -> Self {
        IfSelectActivator {
            group: "XSTEP".to_string(),
            file: String::new(),
            commands: HashMap::new(),
        }
    }

    /// Sets the group name and optional file
    pub fn set_for_group(&mut self, group: &str, file: &str) {
        self.group = group.to_string();
        self.file = file.to_string();
    }

    /// Returns the group name
    pub fn group(&self) -> &str {
        &self.group
    }

    /// Returns the file name
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Adds a command with its number
    pub fn add(&mut self, number: i32, command: &str) {
        self.commands.insert(command.to_string(), number);
    }

    /// Adds a command for xset (creation mode)
    pub fn add_set(&mut self, number: i32, command: &str) {
        self.commands.insert(command.to_string(), number);
    }

    /// Removes a command
    pub fn remove(&mut self, command: &str) {
        self.commands.remove(command);
    }

    /// Selects a command and returns its number
    pub fn select(&self, command: &str) -> Option<i32> {
        self.commands.get(command).copied()
    }

    /// Returns the mode for a command (-1 if not found)
    pub fn mode(&self, _command: &str) -> i32 {
        0
    }

    /// Returns all commands
    pub fn commands_list(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    /// Execute command (to be overridden)
    pub fn do_command(&self, _number: i32) -> ReturnStatus {
        ReturnStatus::RetVoid
    }

    /// Get help for a command (to be overridden)
    pub fn help(&self, _number: i32) -> &str {
        "No help available"
    }
}

impl Default for IfSelectActivator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let act = IfSelectActivator::new();
        assert_eq!(act.group(), "XSTEP");
    }

    #[test]
    fn test_set_group() {
        let mut act = IfSelectActivator::new();
        act.set_for_group("CUSTOM", "file.txt");
        assert_eq!(act.group(), "CUSTOM");
        assert_eq!(act.file(), "file.txt");
    }

    #[test]
    fn test_add_command() {
        let mut act = IfSelectActivator::new();
        act.add(1, "test");
        assert_eq!(act.select("test"), Some(1));
    }

    #[test]
    fn test_remove_command() {
        let mut act = IfSelectActivator::new();
        act.add(1, "test");
        assert_eq!(act.select("test"), Some(1));
        act.remove("test");
        assert_eq!(act.select("test"), None);
    }

    #[test]
    fn test_commands_list() {
        let mut act = IfSelectActivator::new();
        act.add(1, "cmd1");
        act.add(2, "cmd2");
        let cmds = act.commands_list();
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn test_return_status() {
        assert_eq!(ReturnStatus::RetVoid as i32, 0);
        assert_eq!(ReturnStatus::RetDone as i32, 1);
        assert_eq!(ReturnStatus::RetError as i32, -1);
    }
}
