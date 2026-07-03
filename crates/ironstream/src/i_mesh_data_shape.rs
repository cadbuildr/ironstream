// FILE: i_mesh_data_shape.rs
// occt: IMeshData_Shape

/// Interface class representing model with associated TopoDS_Shape.
/// Intended for inheritance by structures and algorithms keeping reference TopoDS_Shape.
pub struct IMeshDataShape {
    shape: Option<String>, // Placeholder for TopoDS_Shape (not fully portable)
}

impl IMeshDataShape {
    /// Create new instance.
    pub fn new() -> Self {
        IMeshDataShape { shape: None }
    }

    /// Set shape.
    pub fn set_shape(&mut self, shape_data: String) {
        self.shape = Some(shape_data);
    }

    /// Get shape.
    pub fn get_shape(&self) -> Option<&String> {
        self.shape.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shape = IMeshDataShape::new();
        assert_eq!(shape.get_shape(), None);
    }

    #[test]
    fn test_set_get_shape() {
        let mut shape = IMeshDataShape::new();
        shape.set_shape("test_shape".to_string());
        assert_eq!(shape.get_shape(), Some(&"test_shape".to_string()));
    }
}
