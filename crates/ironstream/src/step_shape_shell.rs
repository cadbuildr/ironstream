// FILE: step_shape_shell.rs
// occt: StepShape_Shell

use std::sync::Arc;

/// Placeholder for StepShape_OpenShell
pub struct OpenShell {
    id: usize,
}

impl OpenShell {
    pub fn new(id: usize) -> Self {
        OpenShell { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepShape_ClosedShell
pub struct ClosedShell {
    id: usize,
}

impl ClosedShell {
    pub fn new(id: usize) -> Self {
        ClosedShell { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// A discriminated union type representing a shell (OpenShell or ClosedShell).
pub enum Shell {
    /// Case 1: OpenShell
    OpenShell(Arc<OpenShell>),
    /// Case 2: ClosedShell
    ClosedShell(Arc<ClosedShell>),
}

impl Shell {
    /// Create a Shell from an OpenShell
    pub fn from_open_shell(shell: Arc<OpenShell>) -> Self {
        Shell::OpenShell(shell)
    }

    /// Create a Shell from a ClosedShell
    pub fn from_closed_shell(shell: Arc<ClosedShell>) -> Self {
        Shell::ClosedShell(shell)
    }

    /// Get the case number (kind) of this shell
    /// 1 -> OpenShell
    /// 2 -> ClosedShell
    pub fn case_num(&self) -> usize {
        match self {
            Shell::OpenShell(_) => 1,
            Shell::ClosedShell(_) => 2,
        }
    }

    /// Try to get as an OpenShell, returns None if this is a ClosedShell
    pub fn as_open_shell(&self) -> Option<&Arc<OpenShell>> {
        match self {
            Shell::OpenShell(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as a ClosedShell, returns None if this is an OpenShell
    pub fn as_closed_shell(&self) -> Option<&Arc<ClosedShell>> {
        match self {
            Shell::ClosedShell(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_open_shell() {
        let shell = Arc::new(OpenShell::new(1));
        let s = Shell::from_open_shell(shell);
        assert_eq!(s.case_num(), 1);
    }

    #[test]
    fn test_case_num_closed_shell() {
        let shell = Arc::new(ClosedShell::new(2));
        let s = Shell::from_closed_shell(shell);
        assert_eq!(s.case_num(), 2);
    }

    #[test]
    fn test_as_open_shell() {
        let shell = Arc::new(OpenShell::new(10));
        let s = Shell::from_open_shell(shell.clone());
        assert!(s.as_open_shell().is_some());
        assert_eq!(s.as_open_shell().unwrap().id(), 10);
        assert!(s.as_closed_shell().is_none());
    }

    #[test]
    fn test_as_closed_shell() {
        let shell = Arc::new(ClosedShell::new(20));
        let s = Shell::from_closed_shell(shell.clone());
        assert!(s.as_closed_shell().is_some());
        assert_eq!(s.as_closed_shell().unwrap().id(), 20);
        assert!(s.as_open_shell().is_none());
    }

    #[test]
    fn test_multiple_shells() {
        let open = Shell::from_open_shell(Arc::new(OpenShell::new(1)));
        let closed = Shell::from_closed_shell(Arc::new(ClosedShell::new(2)));

        assert_eq!(open.case_num(), 1);
        assert_eq!(closed.case_num(), 2);
    }
}
