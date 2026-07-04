// FILE: shape_persistent_triangle_mode.rs
// occt: ShapePersistent_TriangleMode

/// Triangulation mode enumeration for shape persistence
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriangleMode {
    /// No triangulation
    Undefined = 0,
    /// Delaunay triangulation
    Delaunay = 1,
    /// Constrained triangulation
    Constrained = 2,
}

impl TriangleMode {
    /// Create from integer value
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(TriangleMode::Undefined),
            1 => Some(TriangleMode::Delaunay),
            2 => Some(TriangleMode::Constrained),
            _ => None,
        }
    }

    /// Convert to integer value
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_mode_from_i32() {
        assert_eq!(TriangleMode::from_i32(0), Some(TriangleMode::Undefined));
        assert_eq!(TriangleMode::from_i32(1), Some(TriangleMode::Delaunay));
        assert_eq!(TriangleMode::from_i32(2), Some(TriangleMode::Constrained));
        assert_eq!(TriangleMode::from_i32(3), None);
    }

    #[test]
    fn test_triangle_mode_as_i32() {
        assert_eq!(TriangleMode::Undefined.as_i32(), 0);
        assert_eq!(TriangleMode::Delaunay.as_i32(), 1);
        assert_eq!(TriangleMode::Constrained.as_i32(), 2);
    }
}
