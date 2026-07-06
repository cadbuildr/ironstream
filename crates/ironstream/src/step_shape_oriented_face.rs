// FILE: step_shape_oriented_face.rs
// occt: StepShape_OrientedFace

use std::sync::Arc;

/// Represents an oriented face in STEP format.
/// Inherits from StepShape_Face with additional orientation metadata.
pub struct OrientedFace {
    name: Arc<str>,
    face_element: Option<Arc<Face>>,
    orientation: bool,
}

/// Placeholder for StepShape_Face base class
pub struct Face {
    name: Arc<str>,
}

impl OrientedFace {
    /// Create a new OrientedFace
    pub fn new() -> Self {
        OrientedFace {
            name: Arc::from(""),
            face_element: None,
            orientation: false,
        }
    }

    /// Initialize with name, face element, and orientation
    pub fn init(&mut self, name: Arc<str>, face_element: Arc<Face>, orientation: bool) {
        self.name = name;
        self.face_element = Some(face_element);
        self.orientation = orientation;
    }

    /// Set the face element
    pub fn set_face_element(&mut self, face_element: Arc<Face>) {
        self.face_element = Some(face_element);
    }

    /// Get the face element
    pub fn face_element(&self) -> Option<&Arc<Face>> {
        self.face_element.as_ref()
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

impl Default for OrientedFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oriented_face_creation() {
        let of = OrientedFace::new();
        assert_eq!(of.orientation(), false);
        assert_eq!(of.name(), "");
    }

    #[test]
    fn test_set_orientation() {
        let mut of = OrientedFace::new();
        of.set_orientation(true);
        assert_eq!(of.orientation(), true);
    }

    #[test]
    fn test_init_method() {
        let mut of = OrientedFace::new();
        let face = Arc::new(Face {
            name: Arc::from("test_face"),
        });
        let name: Arc<str> = Arc::from("oriented_face_1");

        of.init(name.clone(), face.clone(), true);

        assert_eq!(of.name(), "oriented_face_1");
        assert_eq!(of.orientation(), true);
        assert!(of.face_element().is_some());
    }

    #[test]
    fn test_set_face_element() {
        let mut of = OrientedFace::new();
        let face = Arc::new(Face {
            name: Arc::from("test_face"),
        });

        of.set_face_element(face.clone());
        assert!(of.face_element().is_some());
    }
}
