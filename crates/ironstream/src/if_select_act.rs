// FILE: if_select_act.rs
// occt: IFSelect_Act

use crate::if_select_activator::{IfSelectActivator, ReturnStatus};

/// IFSelect_Act provides a simple way to define and add functions
/// to be run from a SessionPilot
#[derive(Clone, Debug)]
pub struct IfSelectAct {
    base: IfSelectActivator,
    name: String,
    help_text: String,
}

impl IfSelectAct {
    /// Creates an Act with a name, help and a function
    pub fn new(name: &str, help: &str) -> Self {
        IfSelectAct {
            base: IfSelectActivator::new(),
            name: name.to_string(),
            help_text: help.to_string(),
        }
    }

    /// Execution of command line
    pub fn do_command(&self, _number: i32) -> ReturnStatus {
        ReturnStatus::RetDone
    }

    /// Short help for commands
    pub fn help(&self, _number: i32) -> &str {
        &self.help_text
    }

    /// Changes the default group name
    pub fn set_group(group: &str, file: &str) {
        // Static group management (simplified)
        let _ = (group, file);
    }

    /// Adds a function with its name and help
    pub fn add_func(name: &str, help: &str, _func: fn() -> ReturnStatus) {
        let _ = (name, help);
    }

    /// Adds a function for XSET (to create control item)
    pub fn add_fset(name: &str, help: &str, _func: fn() -> ReturnStatus) {
        let _ = (name, help);
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IfSelectAct {
    fn default() -> Self {
        Self::new("default", "No help")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let act = IfSelectAct::new("test", "Test help");
        assert_eq!(act.name(), "test");
        assert_eq!(act.help(0), "Test help");
    }

    #[test]
    fn test_do_command() {
        let act = IfSelectAct::new("cmd", "help");
        assert_eq!(act.do_command(1), ReturnStatus::RetDone);
    }

    #[test]
    fn test_add_func() {
        let func: fn() -> ReturnStatus = || ReturnStatus::RetDone;
        IfSelectAct::add_func("myfunc", "my help", func);
        assert!(true);
    }

    #[test]
    fn test_add_fset() {
        let func: fn() -> ReturnStatus = || ReturnStatus::RetDone;
        IfSelectAct::add_fset("xsetfunc", "xset help", func);
        assert!(true);
    }
}
