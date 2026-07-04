// FILE: step_shape_open_shell.rs
// occt: StepShape_OpenShell

//! Representation of STEP entity OpenShell

#[derive(Clone, Debug)]
pub struct OpenShell {
    name: String,
    cfs_faces: Vec<String>, // Inherited from ConnectedFaceSet
}

impl OpenShell {
    /// Returns an OpenShell
    pub fn new() -> Self {
        OpenShell {
            name: String::new(),
            cfs_faces: Vec::new(),
        }
    }

    /// Initialize all fields (inherited)
    pub fn init(&mut self, name: String, faces: Vec<String>) {
        self.name = name;
        self.cfs_faces = faces;
    }

    /// Set CfsFaces (inherited)
    pub fn set_cfs_faces(&mut self, faces: Vec<String>) {
        self.cfs_faces = faces;
    }

    /// Returns CfsFaces (inherited)
    pub fn cfs_faces(&self) -> &[String] {
        &self.cfs_faces
    }

    /// Returns number of faces (inherited)
    pub fn nb_cfs_faces(&self) -> usize {
        self.cfs_faces.len()
    }

    /// Returns name field (inherited)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field (inherited)
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for OpenShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shell = OpenShell::new();
        assert_eq!(shell.name(), "");
        assert_eq!(shell.nb_cfs_faces(), 0);
    }

    #[test]
    fn test_init() {
        let mut shell = OpenShell::new();
        shell.init("Shell1".to_string(), vec!["face1".to_string(), "face2".to_string()]);
        assert_eq!(shell.name(), "Shell1");
        assert_eq!(shell.nb_cfs_faces(), 2);
    }

    #[test]
    fn test_set_cfs_faces() {
        let mut shell = OpenShell::new();
        shell.set_cfs_faces(vec!["f1".to_string(), "f2".to_string()]);
        assert_eq!(shell.nb_cfs_faces(), 2);
    }
}
