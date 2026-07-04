// FILE: step_shape_face_based_surface_model.rs
// occt: StepShape_FaceBasedSurfaceModel

//! Representation of STEP entity FaceBasedSurfaceModel

#[derive(Clone, Debug)]
pub struct FaceBasedSurfaceModel {
    name: String,
    fbsm_faces: Vec<String>, // Placeholder for ConnectedFaceSet handles
}

impl FaceBasedSurfaceModel {
    /// Empty constructor
    pub fn new() -> Self {
        FaceBasedSurfaceModel {
            name: String::new(),
            fbsm_faces: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, faces: Vec<String>) {
        self.name = name;
        self.fbsm_faces = faces;
    }

    /// Returns field FbsmFaces
    pub fn fbsm_faces(&self) -> &[String] {
        &self.fbsm_faces
    }

    /// Set field FbsmFaces
    pub fn set_fbsm_faces(&mut self, faces: Vec<String>) {
        self.fbsm_faces = faces;
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

impl Default for FaceBasedSurfaceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let model = FaceBasedSurfaceModel::new();
        assert_eq!(model.name(), "");
        assert_eq!(model.fbsm_faces().len(), 0);
    }

    #[test]
    fn test_init() {
        let mut model = FaceBasedSurfaceModel::new();
        model.init(
            "Model1".to_string(),
            vec!["face_set1".to_string(), "face_set2".to_string()],
        );
        assert_eq!(model.name(), "Model1");
        assert_eq!(model.fbsm_faces().len(), 2);
    }

    #[test]
    fn test_set_fbsm_faces() {
        let mut model = FaceBasedSurfaceModel::new();
        model.set_fbsm_faces(vec!["fs1".to_string(), "fs2".to_string()]);
        assert_eq!(model.fbsm_faces().len(), 2);
    }
}
