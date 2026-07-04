// FILE: step_shape_advanced_face.rs
// occt: StepShape_AdvancedFace

/// Represents an advanced face in a STEP shape model
pub struct AdvancedFace {
    name: Option<String>,
    face_type: String,
}

impl AdvancedFace {
    /// Create a new AdvancedFace
    pub fn new() -> Self {
        AdvancedFace {
            name: None,
            face_type: "ADVANCED_FACE".to_string(),
        }
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the face type
    pub fn face_type(&self) -> &str {
        &self.face_type
    }
}

impl Default for AdvancedFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let face = AdvancedFace::new();
        assert_eq!(face.name(), None);
        assert_eq!(face.face_type(), "ADVANCED_FACE");
    }

    #[test]
    fn test_set_and_get_name() {
        let mut face = AdvancedFace::new();
        face.set_name("Face1".to_string());
        assert_eq!(face.name(), Some("Face1"));
    }

    #[test]
    fn test_face_type() {
        let face = AdvancedFace::new();
        assert!(face.face_type().contains("ADVANCED"));
        assert!(face.face_type().contains("FACE"));
    }
}
