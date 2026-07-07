// FILE: step_select_activator.rs
// occt: StepSelect_Activator

/// Return status for command execution
#[derive(Clone, Debug, PartialEq)]
pub enum ReturnStatus {
    Ok,
    Error,
}

/// Performs actions specific to StepSelect
pub struct Activator {
    // No public fields
}

impl Activator {
    /// Create a new Activator
    pub fn new() -> Self {
        Activator {}
    }

    /// Execute a command by number
    pub fn do_command(&self, number: i32) -> ReturnStatus {
        match number {
            1 => ReturnStatus::Ok,
            _ => ReturnStatus::Error,
        }
    }

    /// Get help text for a command
    pub fn help(&self, number: i32) -> &'static str {
        match number {
            1 => "StepSelect command 1",
            _ => "Unknown command",
        }
    }
}

impl Default for Activator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let act = Activator::new();
        assert!(matches!(act, Activator { .. }));
    }

    #[test]
    fn test_do_command_valid() {
        let act = Activator::new();
        let result = act.do_command(1);
        assert_eq!(result, ReturnStatus::Ok);
    }

    #[test]
    fn test_do_command_invalid() {
        let act = Activator::new();
        let result = act.do_command(999);
        assert_eq!(result, ReturnStatus::Error);
    }

    #[test]
    fn test_help() {
        let act = Activator::new();
        assert!(!act.help(1).is_empty());
        assert!(!act.help(999).is_empty());
    }
}
