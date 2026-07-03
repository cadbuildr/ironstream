// FILE: v3d_type_of_view.rs
// occt: V3d_TypeOfView

//! Defines the type of projection of the view.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeOfView {
    #[default]
    Orthographic,
    Perspective,
}

impl TypeOfView {
    pub fn as_u32(&self) -> u32 {
        match self {
            TypeOfView::Orthographic => 0,
            TypeOfView::Perspective => 1,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(TypeOfView::Orthographic),
            1 => Some(TypeOfView::Perspective),
            _ => None,
        }
    }

    pub fn is_orthographic(&self) -> bool {
        *self == TypeOfView::Orthographic
    }

    pub fn is_perspective(&self) -> bool {
        *self == TypeOfView::Perspective
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_of_view_conversions() {
        assert_eq!(TypeOfView::Orthographic.as_u32(), 0);
        assert_eq!(TypeOfView::Perspective.as_u32(), 1);
    }

    #[test]
    fn type_of_view_from_u32() {
        assert_eq!(TypeOfView::from_u32(0), Some(TypeOfView::Orthographic));
        assert_eq!(TypeOfView::from_u32(1), Some(TypeOfView::Perspective));
        assert_eq!(TypeOfView::from_u32(2), None);
    }

    #[test]
    fn type_of_view_predicates() {
        assert!(TypeOfView::Orthographic.is_orthographic());
        assert!(!TypeOfView::Orthographic.is_perspective());
        assert!(!TypeOfView::Perspective.is_orthographic());
        assert!(TypeOfView::Perspective.is_perspective());
    }
}
