// FILE: step_shape_closed_shell.rs
// occt: StepShape_ClosedShell

/// Placeholder for Face
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    id: String,
}

/// Represents a closed shell (a complete boundary representation) in STEP
pub struct ClosedShell {
    name: Option<String>,
    faces: Vec<Face>,
}

impl ClosedShell {
    /// Create a new ClosedShell
    pub fn new() -> Self {
        ClosedShell {
            name: None,
            faces: Vec::new(),
        }
    }

    /// Initialize with name and faces
    pub fn init(&mut self, name: String, faces: Vec<Face>) {
        self.name = Some(name);
        self.faces = faces;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the faces
    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    /// Set the faces
    pub fn set_faces(&mut self, faces: Vec<Face>) {
        self.faces = faces;
    }

    /// Get a face by index (1-based)
    pub fn faces_value(&self, num: usize) -> Option<&Face> {
        if num > 0 && num <= self.faces.len() {
            Some(&self.faces[num - 1])
        } else {
            None
        }
    }

    /// Get the number of faces
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }
}

impl Default for ClosedShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let shell = ClosedShell::new();
        assert_eq!(shell.name(), None);
        assert_eq!(shell.nb_faces(), 0);
    }

    #[test]
    fn test_init() {
        let mut shell = ClosedShell::new();
        let face1 = Face { id: "f1".to_string() };
        let face2 = Face { id: "f2".to_string() };
        shell.init(
            "Shell1".to_string(),
            vec![face1.clone(), face2.clone()],
        );
        assert_eq!(shell.name(), Some("Shell1"));
        assert_eq!(shell.nb_faces(), 2);
        assert_eq!(shell.faces_value(1), Some(&face1));
        assert_eq!(shell.faces_value(2), Some(&face2));
    }

    #[test]
    fn test_set_faces() {
        let mut shell = ClosedShell::new();
        let face = Face { id: "f3".to_string() };
        shell.set_faces(vec![face.clone()]);
        assert_eq!(shell.nb_faces(), 1);
        assert_eq!(shell.faces_value(1), Some(&face));
    }

    #[test]
    fn test_faces_value_out_of_bounds() {
        let shell = ClosedShell::new();
        assert_eq!(shell.faces_value(1), None);
        assert_eq!(shell.faces_value(999), None);
    }
}
