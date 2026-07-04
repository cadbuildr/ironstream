// FILE: std_object_shape.rs
// occt: StdObject_Shape

/// Persistent representation of a topological shape
pub struct StdObjectShape {
    shape_type: i32,
    orientation: i32,
    location_ref: Option<i32>,
}

impl StdObjectShape {
    /// Create a new shape
    pub fn new() -> Self {
        StdObjectShape {
            shape_type: 0,
            orientation: 0,
            location_ref: None,
        }
    }

    /// Get the shape type
    pub fn shape_type(&self) -> i32 {
        self.shape_type
    }

    /// Set the shape type
    pub fn set_shape_type(&mut self, typ: i32) {
        self.shape_type = typ;
    }

    /// Get the orientation
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Set the orientation
    pub fn set_orientation(&mut self, orient: i32) {
        self.orientation = orient;
    }

    /// Get the location reference
    pub fn location_ref(&self) -> Option<i32> {
        self.location_ref
    }

    /// Set the location reference
    pub fn set_location_ref(&mut self, ref_num: Option<i32>) {
        self.location_ref = ref_num;
    }
}

impl Default for StdObjectShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let shape = StdObjectShape::new();
        assert_eq!(shape.shape_type(), 0);
        assert_eq!(shape.orientation(), 0);
        assert_eq!(shape.location_ref(), None);
    }

    #[test]
    fn test_set_shape_type() {
        let mut shape = StdObjectShape::new();
        shape.set_shape_type(2);
        assert_eq!(shape.shape_type(), 2);
    }

    #[test]
    fn test_set_orientation() {
        let mut shape = StdObjectShape::new();
        shape.set_orientation(1);
        assert_eq!(shape.orientation(), 1);
    }

    #[test]
    fn test_set_location_ref() {
        let mut shape = StdObjectShape::new();
        shape.set_location_ref(Some(42));
        assert_eq!(shape.location_ref(), Some(42));
    }
}
