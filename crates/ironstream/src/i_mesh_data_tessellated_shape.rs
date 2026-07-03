// FILE: i_mesh_data_tessellated_shape.rs
// occt: IMeshData_TessellatedShape

/// Interface class representing shaped model with deflection.
pub struct IMeshDataTessellatedShape {
    shape: Option<String>,
    deflection: f64,
}

impl IMeshDataTessellatedShape {
    /// Create new instance with default deflection (f64::MAX).
    pub fn new() -> Self {
        IMeshDataTessellatedShape {
            shape: None,
            deflection: f64::MAX,
        }
    }

    /// Create with shape.
    pub fn with_shape(shape_data: String) -> Self {
        IMeshDataTessellatedShape {
            shape: Some(shape_data),
            deflection: f64::MAX,
        }
    }

    /// Set shape.
    pub fn set_shape(&mut self, shape_data: String) {
        self.shape = Some(shape_data);
    }

    /// Get shape.
    pub fn get_shape(&self) -> Option<&String> {
        self.shape.as_ref()
    }

    /// Get deflection value for the discrete model.
    pub fn get_deflection(&self) -> f64 {
        self.deflection
    }

    /// Set deflection value for the discrete model.
    pub fn set_deflection(&mut self, value: f64) {
        self.deflection = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shape = IMeshDataTessellatedShape::new();
        assert_eq!(shape.get_deflection(), f64::MAX);
        assert_eq!(shape.get_shape(), None);
    }

    #[test]
    fn test_with_shape() {
        let shape = IMeshDataTessellatedShape::with_shape("test".to_string());
        assert_eq!(shape.get_shape(), Some(&"test".to_string()));
        assert_eq!(shape.get_deflection(), f64::MAX);
    }

    #[test]
    fn test_set_deflection() {
        let mut shape = IMeshDataTessellatedShape::new();
        shape.set_deflection(0.01);
        assert_eq!(shape.get_deflection(), 0.01);
    }
}
