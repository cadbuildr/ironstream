// FILE: step_shape_connected_face_sub_set.rs
// occt: StepShape_ConnectedFaceSubSet

//! Representation of STEP entity ConnectedFaceSubSet

#[derive(Clone, Debug)]
pub struct ConnectedFaceSubSet {
    name: String,
    cfs_faces: Vec<String>, // Placeholder for Face handles
    parent_face_set: Option<Box<ConnectedFaceSubSet>>, // Self-reference for parent
}

impl ConnectedFaceSubSet {
    /// Empty constructor
    pub fn new() -> Self {
        ConnectedFaceSubSet {
            name: String::new(),
            cfs_faces: Vec::new(),
            parent_face_set: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, faces: Vec<String>, parent: Option<Box<ConnectedFaceSubSet>>) {
        self.name = name;
        self.cfs_faces = faces;
        self.parent_face_set = parent;
    }

    /// Returns field ParentFaceSet
    pub fn parent_face_set(&self) -> &Option<Box<ConnectedFaceSubSet>> {
        &self.parent_face_set
    }

    /// Set field ParentFaceSet
    pub fn set_parent_face_set(&mut self, parent: Option<Box<ConnectedFaceSubSet>>) {
        self.parent_face_set = parent;
    }

    /// Set field CfsFaces (inherited)
    pub fn set_cfs_faces(&mut self, faces: Vec<String>) {
        self.cfs_faces = faces;
    }

    /// Returns field CfsFaces (inherited)
    pub fn cfs_faces(&self) -> &[String] {
        &self.cfs_faces
    }

    /// Returns a face value by index (1-based, inherited)
    pub fn cfs_faces_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.cfs_faces.len() {
            Some(&self.cfs_faces[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of faces (inherited)
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

impl Default for ConnectedFaceSubSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let subset = ConnectedFaceSubSet::new();
        assert_eq!(subset.name(), "");
        assert_eq!(subset.nb_cfs_faces(), 0);
        assert!(subset.parent_face_set().is_none());
    }

    #[test]
    fn test_init() {
        let mut subset = ConnectedFaceSubSet::new();
        subset.init("TestSubSet".to_string(), vec!["face1".to_string(), "face2".to_string()], None);
        assert_eq!(subset.name(), "TestSubSet");
        assert_eq!(subset.nb_cfs_faces(), 2);
    }

    #[test]
    fn test_set_parent_face_set() {
        let mut subset = ConnectedFaceSubSet::new();
        let parent = Box::new(ConnectedFaceSubSet::new());
        subset.set_parent_face_set(Some(parent));
        assert!(subset.parent_face_set().is_some());
    }

    #[test]
    fn test_cfs_faces_inherited() {
        let mut subset = ConnectedFaceSubSet::new();
        subset.set_cfs_faces(vec!["f1".to_string(), "f2".to_string()]);
        assert_eq!(subset.cfs_faces_value(1), Some(&"f1".to_string()));
    }
}
