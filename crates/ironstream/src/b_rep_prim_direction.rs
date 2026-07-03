// FILE: b_rep_prim_direction.rs
// occt: BRepPrim_Direction

/// Direction enumeration for primitive faces (XMin/XMax, YMin/YMax, ZMin/ZMax).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BRepPrimDirection {
    /// Minimum X face
    XMin = 0,
    /// Maximum X face
    XMax = 1,
    /// Minimum Y face
    YMin = 2,
    /// Maximum Y face
    YMax = 3,
    /// Minimum Z face
    ZMin = 4,
    /// Maximum Z face
    ZMax = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_values() {
        assert_eq!(BRepPrimDirection::XMin as i32, 0);
        assert_eq!(BRepPrimDirection::XMax as i32, 1);
        assert_eq!(BRepPrimDirection::YMin as i32, 2);
        assert_eq!(BRepPrimDirection::YMax as i32, 3);
        assert_eq!(BRepPrimDirection::ZMin as i32, 4);
        assert_eq!(BRepPrimDirection::ZMax as i32, 5);
    }

    #[test]
    fn test_direction_ordering() {
        assert!(BRepPrimDirection::XMin < BRepPrimDirection::XMax);
        assert!(BRepPrimDirection::ZMax > BRepPrimDirection::XMin);
    }
}
