// FILE: std_object_location.rs
// occt: StdObject_Location

/// Persistent representation of location (transformation)
#[derive(Clone, Debug)]
pub struct StdObjectLocation {
    transform_type: i32,
    values: Vec<f64>,
}

impl StdObjectLocation {
    /// Create a new location
    pub fn new() -> Self {
        StdObjectLocation {
            transform_type: 0,
            values: Vec::new(),
        }
    }

    /// Create an identity location
    pub fn identity() -> Self {
        StdObjectLocation {
            transform_type: 0,
            values: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        }
    }

    /// Get the transformation type
    pub fn transform_type(&self) -> i32 {
        self.transform_type
    }

    /// Set the transformation type
    pub fn set_transform_type(&mut self, typ: i32) {
        self.transform_type = typ;
    }

    /// Get transformation values
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Set transformation values
    pub fn set_values(&mut self, vals: Vec<f64>) {
        self.values = vals;
    }
}

impl Default for StdObjectLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let loc = StdObjectLocation::new();
        assert_eq!(loc.transform_type(), 0);
        assert!(loc.values().is_empty());
    }

    #[test]
    fn test_identity() {
        let loc = StdObjectLocation::identity();
        assert_eq!(loc.values().len(), 12);
    }

    #[test]
    fn test_set_transform_type() {
        let mut loc = StdObjectLocation::new();
        loc.set_transform_type(1);
        assert_eq!(loc.transform_type(), 1);
    }

    #[test]
    fn test_set_values() {
        let mut loc = StdObjectLocation::new();
        let vals = vec![1.0, 0.0, 0.0, 0.0];
        loc.set_values(vals.clone());
        assert_eq!(loc.values(), &vals[..]);
    }
}
