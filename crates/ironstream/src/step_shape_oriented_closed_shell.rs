// FILE: step_shape_oriented_closed_shell.rs
// occt: StepShape_OrientedClosedShell

//! Representation of STEP entity OrientedClosedShell

#[derive(Clone, Debug)]
pub struct OrientedClosedShell {
    name: String,
    cfs_faces: Vec<String>,
    closed_shell_element: Option<String>,
    orientation: bool,
}

impl OrientedClosedShell {
    /// Returns an OrientedClosedShell
    pub fn new() -> Self {
        OrientedClosedShell {
            name: String::new(),
            cfs_faces: Vec::new(),
            closed_shell_element: None,
            orientation: false,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, closed_shell: Option<String>, orientation: bool) {
        self.name = name;
        self.closed_shell_element = closed_shell;
        self.orientation = orientation;
        self.cfs_faces = Vec::new();
    }

    /// Set ClosedShellElement
    pub fn set_closed_shell_element(&mut self, element: Option<String>) {
        self.closed_shell_element = element;
    }

    /// Returns ClosedShellElement
    pub fn closed_shell_element(&self) -> &Option<String> {
        &self.closed_shell_element
    }

    /// Set Orientation
    pub fn set_orientation(&mut self, orientation: bool) {
        self.orientation = orientation;
    }

    /// Returns Orientation
    pub fn orientation(&self) -> bool {
        self.orientation
    }

    /// Set CfsFaces (override from parent)
    pub fn set_cfs_faces(&mut self, faces: Vec<String>) {
        self.cfs_faces = faces;
    }

    /// Returns CfsFaces (override from parent)
    pub fn cfs_faces(&self) -> &[String] {
        &self.cfs_faces
    }

    /// Returns face at index (1-based, override)
    pub fn cfs_faces_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.cfs_faces.len() {
            Some(&self.cfs_faces[num - 1])
        } else {
            None
        }
    }

    /// Returns number of faces (override)
    pub fn nb_cfs_faces(&self) -> usize {
        self.cfs_faces.len()
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for OrientedClosedShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shell = OrientedClosedShell::new();
        assert_eq!(shell.name(), "");
        assert!(!shell.orientation());
        assert!(shell.closed_shell_element().is_none());
    }

    #[test]
    fn test_init() {
        let mut shell = OrientedClosedShell::new();
        shell.init("Shell1".to_string(), Some("cs1".to_string()), true);
        assert_eq!(shell.name(), "Shell1");
        assert!(shell.orientation());
    }

    #[test]
    fn test_set_orientation() {
        let mut shell = OrientedClosedShell::new();
        shell.set_orientation(true);
        assert!(shell.orientation());
    }

    #[test]
    fn test_cfs_faces() {
        let mut shell = OrientedClosedShell::new();
        shell.set_cfs_faces(vec!["f1".to_string(), "f2".to_string()]);
        assert_eq!(shell.nb_cfs_faces(), 2);
        assert_eq!(shell.cfs_faces_value(1), Some(&"f1".to_string()));
    }
}
