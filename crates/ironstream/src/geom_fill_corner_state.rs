// FILE: geom_fill_corner_state.rs
// occt: GeomFill_CornerState

/// State of corner continuity for surface filling
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CornerState {
    Unset = 0,
    Tangent = 1,
    PointAndTangent = 2,
}

impl CornerState {
    pub fn is_tangent(&self) -> bool {
        matches!(self, CornerState::Tangent | CornerState::PointAndTangent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(CornerState::Unset as i32, 0);
        assert_eq!(CornerState::Tangent as i32, 1);
        assert_eq!(CornerState::PointAndTangent as i32, 2);
    }

    #[test]
    fn test_is_tangent() {
        assert!(!CornerState::Unset.is_tangent());
        assert!(CornerState::Tangent.is_tangent());
        assert!(CornerState::PointAndTangent.is_tangent());
    }
}
