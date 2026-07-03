// FILE: std_select_type_of_edge.rs
// occt: StdSelect_TypeOfEdge

//! Provides values for different types of edges.
//! These values are used to filter edges in frameworks inheriting StdSelect_EdgeFilter.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeOfEdge {
    #[default]
    AnyEdge,
    Line,
    Circle,
}

impl TypeOfEdge {
    pub fn as_u32(&self) -> u32 {
        match self {
            TypeOfEdge::AnyEdge => 0,
            TypeOfEdge::Line => 1,
            TypeOfEdge::Circle => 2,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(TypeOfEdge::AnyEdge),
            1 => Some(TypeOfEdge::Line),
            2 => Some(TypeOfEdge::Circle),
            _ => None,
        }
    }

    pub fn is_any(&self) -> bool {
        *self == TypeOfEdge::AnyEdge
    }

    pub fn is_linear(&self) -> bool {
        *self == TypeOfEdge::Line
    }

    pub fn is_circular(&self) -> bool {
        *self == TypeOfEdge::Circle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_of_edge_conversions() {
        assert_eq!(TypeOfEdge::AnyEdge.as_u32(), 0);
        assert_eq!(TypeOfEdge::Line.as_u32(), 1);
        assert_eq!(TypeOfEdge::Circle.as_u32(), 2);
    }

    #[test]
    fn type_of_edge_from_u32() {
        assert_eq!(TypeOfEdge::from_u32(0), Some(TypeOfEdge::AnyEdge));
        assert_eq!(TypeOfEdge::from_u32(1), Some(TypeOfEdge::Line));
        assert_eq!(TypeOfEdge::from_u32(2), Some(TypeOfEdge::Circle));
        assert_eq!(TypeOfEdge::from_u32(3), None);
    }

    #[test]
    fn type_of_edge_predicates() {
        assert!(TypeOfEdge::AnyEdge.is_any());
        assert!(TypeOfEdge::Line.is_linear());
        assert!(TypeOfEdge::Circle.is_circular());
    }
}
