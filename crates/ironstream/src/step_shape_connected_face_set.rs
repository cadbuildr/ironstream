// FILE: step_shape_connected_face_set.rs
// occt: StepShape_ConnectedFaceSet

//! Representation of STEP entity ConnectedFaceSet

#[derive(Clone, Debug)]
pub struct ConnectedFaceSet {
    name: String,
    cfs_faces: Vec<String>, // Placeholder for Face handles
}

impl ConnectedFaceSet {
    /// Returns a ConnectedFaceSet
    pub fn new() -> Self {
        ConnectedFaceSet {
            name: String::new(),
            cfs_faces: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, faces: Vec<String>) {
        self.name = name;
        self.cfs_faces = faces;
    }

    /// Set field CfsFaces
    pub fn set_cfs_faces(&mut self, faces: Vec<String>) {
        self.cfs_faces = faces;
    }

    /// Returns field CfsFaces
    pub fn cfs_faces(&self) -> &[String] {
        &self.cfs_faces
    }

    /// Returns a face value by index (1-based)
    pub fn cfs_faces_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.cfs_faces.len() {
            Some(&self.cfs_faces[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of faces
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

impl Default for ConnectedFaceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cfs = ConnectedFaceSet::new();
        assert_eq!(cfs.name(), "");
        assert_eq!(cfs.nb_cfs_faces(), 0);
    }

    #[test]
    fn test_init() {
        let mut cfs = ConnectedFaceSet::new();
        cfs.init("TestSet".to_string(), vec!["face1".to_string(), "face2".to_string()]);
        assert_eq!(cfs.name(), "TestSet");
        assert_eq!(cfs.nb_cfs_faces(), 2);
    }

    #[test]
    fn test_set_cfs_faces() {
        let mut cfs = ConnectedFaceSet::new();
        cfs.set_cfs_faces(vec!["f1".to_string(), "f2".to_string(), "f3".to_string()]);
        assert_eq!(cfs.nb_cfs_faces(), 3);
    }

    #[test]
    fn test_cfs_faces_value() {
        let mut cfs = ConnectedFaceSet::new();
        cfs.set_cfs_faces(vec!["f1".to_string(), "f2".to_string()]);
        assert_eq!(cfs.cfs_faces_value(1), Some(&"f1".to_string()));
        assert_eq!(cfs.cfs_faces_value(2), Some(&"f2".to_string()));
        assert_eq!(cfs.cfs_faces_value(3), None);
    }

    #[test]
    fn test_set_name() {
        let mut cfs = ConnectedFaceSet::new();
        cfs.set_name("MyFaces".to_string());
        assert_eq!(cfs.name(), "MyFaces");
    }
}
