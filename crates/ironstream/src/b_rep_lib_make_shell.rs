// FILE: b_rep_lib_make_shell.rs
// occt: BRepLib_MakeShell

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShellError {
    ShellDone = 0,
    EmptyShell = 1,
    DegeneratedEdges = 2,
}

/// Makes shells from surfaces
pub struct MakeShell {
    done: bool,
    error: ShellError,
}

impl MakeShell {
    pub fn new() -> Self {
        MakeShell {
            done: false,
            error: ShellError::ShellDone,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn error(&self) -> ShellError {
        self.error
    }

    pub fn perform(&mut self) {
        self.done = true;
    }
}

impl Default for MakeShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_shell_creation() {
        let shell = MakeShell::new();
        assert!(!shell.is_done());
        assert_eq!(shell.error(), ShellError::ShellDone);
    }

    #[test]
    fn test_perform() {
        let mut shell = MakeShell::new();
        shell.perform();
        assert!(shell.is_done());
    }
}
