// FILE: b_rep_lib_shape_modification.rs
// occt: BRepLib_ShapeModification

/// Modification type after a topologic operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BRepLibShapeModification {
    /// Preserved without change
    Preserved = 0,
    /// Deleted during operation
    Deleted = 1,
    /// Trimmed (reduced)
    Trimmed = 2,
    /// Merged with another shape
    Merged = 3,
    /// Boundary was modified
    BoundaryModified = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(BRepLibShapeModification::Preserved as i32, 0);
        assert_eq!(BRepLibShapeModification::Deleted as i32, 1);
        assert_eq!(BRepLibShapeModification::Trimmed as i32, 2);
        assert_eq!(BRepLibShapeModification::Merged as i32, 3);
        assert_eq!(BRepLibShapeModification::BoundaryModified as i32, 4);
    }

    #[test]
    fn test_enum_equality() {
        let preserved = BRepLibShapeModification::Preserved;
        let preserved2 = BRepLibShapeModification::Preserved;
        assert_eq!(preserved, preserved2);
    }
}
