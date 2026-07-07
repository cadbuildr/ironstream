// FILE: xcaf_dim_tol_objects_dimension_object.rs
// occt: XCAFDimTolObjects_DimensionObject

/// Represents a dimension object in XCAF.
#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DimensionObject {
    name: String,
    value: f64,
    unit: String,
    is_bilateral: bool,
    is_qualified: bool,
}

impl XCAFDimTolObjects_DimensionObject {
    /// Create a new dimension object.
    pub fn new(name: String, value: f64, unit: String) -> Self {
        Self {
            name,
            value,
            unit,
            is_bilateral: false,
            is_qualified: false,
        }
    }

    /// Get the dimension name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the dimension value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set the dimension value.
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Get the unit.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Check if bilateral.
    pub fn is_bilateral(&self) -> bool {
        self.is_bilateral
    }

    /// Set bilateral flag.
    pub fn set_bilateral(&mut self, bilateral: bool) {
        self.is_bilateral = bilateral;
    }

    /// Check if qualified.
    pub fn is_qualified(&self) -> bool {
        self.is_qualified
    }

    /// Set qualified flag.
    pub fn set_qualified(&mut self, qualified: bool) {
        self.is_qualified = qualified;
    }
}

impl Default for XCAFDimTolObjects_DimensionObject {
    fn default() -> Self {
        Self::new("Dimension".to_string(), 0.0, "mm".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dimension() {
        let dim = XCAFDimTolObjects_DimensionObject::new(
            "Width".to_string(),
            10.5,
            "mm".to_string(),
        );
        assert_eq!(dim.name(), "Width");
        assert_eq!(dim.value(), 10.5);
        assert_eq!(dim.unit(), "mm");
    }

    #[test]
    fn test_set_value() {
        let mut dim = XCAFDimTolObjects_DimensionObject::new(
            "Length".to_string(),
            5.0,
            "mm".to_string(),
        );
        dim.set_value(15.0);
        assert_eq!(dim.value(), 15.0);
    }

    #[test]
    fn test_bilateral_flag() {
        let mut dim = XCAFDimTolObjects_DimensionObject::new(
            "Height".to_string(),
            20.0,
            "cm".to_string(),
        );
        assert!(!dim.is_bilateral());
        dim.set_bilateral(true);
        assert!(dim.is_bilateral());
    }

    #[test]
    fn test_qualified_flag() {
        let mut dim = XCAFDimTolObjects_DimensionObject::new(
            "Depth".to_string(),
            30.0,
            "mm".to_string(),
        );
        assert!(!dim.is_qualified());
        dim.set_qualified(true);
        assert!(dim.is_qualified());
    }

    #[test]
    fn test_default() {
        let dim = XCAFDimTolObjects_DimensionObject::default();
        assert_eq!(dim.name(), "Dimension");
        assert_eq!(dim.value(), 0.0);
    }
}
