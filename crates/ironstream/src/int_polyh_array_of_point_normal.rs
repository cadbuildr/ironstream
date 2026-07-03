// FILE: int_polyh_array_of_point_normal.rs
// occt: IntPolyh_ArrayOfPointNormal

/// Implementation of IntPolyh_ArrayOfPointNormal
pub struct IntPolyh_ArrayOfPointNormal;

impl IntPolyh_ArrayOfPointNormal {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPolyh_ArrayOfPointNormal
    }
}

impl Default for IntPolyh_ArrayOfPointNormal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPolyh_ArrayOfPointNormal::new();
    }
}
