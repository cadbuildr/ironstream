// FILE: b_rep_lib_command.rs
// occt: BRepLib_Command

/// Root class for all commands in BRepLib
pub struct Command {
    done: bool,
}

impl Command {
    pub fn new() -> Self {
        Command { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn check(&self) -> Result<(), String> {
        if self.done {
            Ok(())
        } else {
            Err("Command not done".to_string())
        }
    }

    pub fn set_done(&mut self) {
        self.done = true;
    }

    pub fn set_not_done(&mut self) {
        self.done = false;
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new();
        assert!(!cmd.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut cmd = Command::new();
        cmd.set_done();
        assert!(cmd.is_done());
    }

    #[test]
    fn test_check() {
        let mut cmd = Command::new();
        assert!(cmd.check().is_err());
        cmd.set_done();
        assert!(cmd.check().is_ok());
    }
}
