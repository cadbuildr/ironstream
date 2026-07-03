// FILE: b_rep_lib_shell_error.rs
// occt: BRepLib_ShellError

/// Errors that can occur at shell construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BRepLibShellError {
    /// Shell constructed successfully
    ShellDone = 0,
    /// Shell is empty
    EmptyShell = 1,
    /// Shell has disconnected parts
    DisconnectedShell = 2,
    /// Shell parameters are out of valid range
    ShellParametersOutOfRange = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_error_values() {
        assert_eq!(BRepLibShellError::ShellDone as i32, 0);
        assert_eq!(BRepLibShellError::EmptyShell as i32, 1);
        assert_eq!(BRepLibShellError::DisconnectedShell as i32, 2);
        assert_eq!(BRepLibShellError::ShellParametersOutOfRange as i32, 3);
    }

    #[test]
    fn test_shell_error_clone() {
        let err = BRepLibShellError::EmptyShell;
        let err2 = err;
        assert_eq!(err, err2);
    }
}
