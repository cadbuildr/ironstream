// FILE: int_polyh_array_of_points.rs
// occt: IntPolyh_ArrayOfPoints

/// Implementation of IntPolyh_ArrayOfPoints
pub struct IntPolyh_ArrayOfPoints;

impl IntPolyh_ArrayOfPoints {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPolyh_ArrayOfPoints
    }
}

impl Default for IntPolyh_ArrayOfPoints {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPolyh_ArrayOfPoints::new();
    }
}
