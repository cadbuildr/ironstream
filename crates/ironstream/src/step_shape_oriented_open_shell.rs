// FILE: step_shape_oriented_open_shell.rs
// occt: StepShape_OrientedOpenShell

use std::sync::Arc;

/// Placeholder for StepShape_OpenShell base class
pub struct OpenShell {
    name: Arc<str>,
}

/// Represents an oriented open shell in STEP format.
/// Inherits from StepShape_OpenShell with additional orientation metadata.
pub struct OrientedOpenShell {
    name: Arc<str>,
    open_shell_element: Option<Arc<OpenShell>>,
    orientation: bool,
}

impl OrientedOpenShell {
    /// Create a new OrientedOpenShell
    pub fn new() -> Self {
        OrientedOpenShell {
            name: Arc::from(""),
            open_shell_element: None,
            orientation: false,
        }
    }

    /// Initialize with name, open shell element, and orientation
    pub fn init(&mut self, name: Arc<str>, open_shell_element: Arc<OpenShell>, orientation: bool) {
        self.name = name;
        self.open_shell_element = Some(open_shell_element);
        self.orientation = orientation;
    }

    /// Set the open shell element
    pub fn set_open_shell_element(&mut self, open_shell_element: Arc<OpenShell>) {
        self.open_shell_element = Some(open_shell_element);
    }

    /// Get the open shell element
    pub fn open_shell_element(&self) -> Option<&Arc<OpenShell>> {
        self.open_shell_element.as_ref()
    }

    /// Set the orientation
    pub fn set_orientation(&mut self, orientation: bool) {
        self.orientation = orientation;
    }

    /// Get the orientation
    pub fn orientation(&self) -> bool {
        self.orientation
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for OrientedOpenShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oriented_open_shell_creation() {
        let oos = OrientedOpenShell::new();
        assert_eq!(oos.orientation(), false);
        assert_eq!(oos.name(), "");
    }

    #[test]
    fn test_set_orientation() {
        let mut oos = OrientedOpenShell::new();
        oos.set_orientation(true);
        assert_eq!(oos.orientation(), true);
    }

    #[test]
    fn test_init_method() {
        let mut oos = OrientedOpenShell::new();
        let shell = Arc::new(OpenShell {
            name: Arc::from("test_shell"),
        });
        let name: Arc<str> = Arc::from("oriented_shell_1");

        oos.init(name.clone(), shell.clone(), true);

        assert_eq!(oos.name(), "oriented_shell_1");
        assert_eq!(oos.orientation(), true);
        assert!(oos.open_shell_element().is_some());
    }

    #[test]
    fn test_set_open_shell_element() {
        let mut oos = OrientedOpenShell::new();
        let shell = Arc::new(OpenShell {
            name: Arc::from("test_shell"),
        });

        oos.set_open_shell_element(shell.clone());
        assert!(oos.open_shell_element().is_some());
    }
}
