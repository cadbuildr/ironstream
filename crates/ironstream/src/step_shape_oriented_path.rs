// FILE: step_shape_oriented_path.rs
// occt: StepShape_OrientedPath

use std::sync::Arc;

/// Placeholder for StepShape_EdgeLoop
pub struct EdgeLoop {
    name: Arc<str>,
}

/// Represents an oriented path in STEP format.
/// Inherits from StepShape_Path with orientation metadata.
pub struct OrientedPath {
    name: Arc<str>,
    path_element: Option<Arc<EdgeLoop>>,
    orientation: bool,
}

impl OrientedPath {
    /// Create a new OrientedPath
    pub fn new() -> Self {
        OrientedPath {
            name: Arc::from(""),
            path_element: None,
            orientation: false,
        }
    }

    /// Initialize with name, path element, and orientation
    pub fn init(&mut self, name: Arc<str>, path_element: Arc<EdgeLoop>, orientation: bool) {
        self.name = name;
        self.path_element = Some(path_element);
        self.orientation = orientation;
    }

    /// Set the path element (EdgeLoop)
    pub fn set_path_element(&mut self, path_element: Arc<EdgeLoop>) {
        self.path_element = Some(path_element);
    }

    /// Get the path element
    pub fn path_element(&self) -> Option<&Arc<EdgeLoop>> {
        self.path_element.as_ref()
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

impl Default for OrientedPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oriented_path_creation() {
        let op = OrientedPath::new();
        assert_eq!(op.orientation(), false);
        assert_eq!(op.name(), "");
    }

    #[test]
    fn test_set_orientation() {
        let mut op = OrientedPath::new();
        op.set_orientation(true);
        assert_eq!(op.orientation(), true);
    }

    #[test]
    fn test_init_method() {
        let mut op = OrientedPath::new();
        let edge_loop = Arc::new(EdgeLoop {
            name: Arc::from("test_loop"),
        });
        let name: Arc<str> = Arc::from("oriented_path_1");

        op.init(name.clone(), edge_loop.clone(), true);

        assert_eq!(op.name(), "oriented_path_1");
        assert_eq!(op.orientation(), true);
        assert!(op.path_element().is_some());
    }

    #[test]
    fn test_set_path_element() {
        let mut op = OrientedPath::new();
        let edge_loop = Arc::new(EdgeLoop {
            name: Arc::from("test_loop"),
        });

        op.set_path_element(edge_loop.clone());
        assert!(op.path_element().is_some());
    }
}
