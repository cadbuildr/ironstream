// FILE: top_ope_b_rep_ds.rs
// occt: TopOpeBRepDS

/// State enumeration for topological analysis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsState {
    In = 0,
    Out = 1,
    On = 2,
    Unknown = 3,
}

/// Kind enumeration for element types in TopOpeBRepDS
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopOpeBRepDSKind {
    Point = 1,
    Curve = 2,
    Surface = 3,
    Solid = 4,
    Shell = 5,
    Face = 6,
    Wire = 7,
    Edge = 8,
    Vertex = 9,
    ShapeVertex = 10,
    ShapeEdge = 11,
    ShapeFace = 12,
}

/// Configuration of topologies
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopOpeBRepDSConfig {
    Unshaded = 0,
    Shaded = 1,
}

/// Shape enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsShapeEnum {
    Compound = 0,
    CompSolid = 1,
    Solid = 2,
    Shell = 3,
    Face = 4,
    Wire = 5,
    Edge = 6,
    Vertex = 7,
    Shape = 8,
}

/// Orientation enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsOrientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

/// Package providing services for topological operations on BRep data structure.
/// Static utility methods for TopOpeBRepDS.
pub struct TopOpeBRepDS;

impl TopOpeBRepDS {
    /// Print state as string
    pub fn sprint_state(state: TopAbsState) -> String {
        match state {
            TopAbsState::In => "IN".to_string(),
            TopAbsState::Out => "OU".to_string(),
            TopAbsState::On => "ON".to_string(),
            TopAbsState::Unknown => "UN".to_string(),
        }
    }

    /// Print kind as string
    pub fn sprint_kind(kind: TopOpeBRepDSKind) -> String {
        match kind {
            TopOpeBRepDSKind::Point => "POINT".to_string(),
            TopOpeBRepDSKind::Curve => "CURVE".to_string(),
            TopOpeBRepDSKind::Surface => "SURFACE".to_string(),
            TopOpeBRepDSKind::Solid => "SOLID".to_string(),
            TopOpeBRepDSKind::Shell => "SHELL".to_string(),
            TopOpeBRepDSKind::Face => "FACE".to_string(),
            TopOpeBRepDSKind::Wire => "WIRE".to_string(),
            TopOpeBRepDSKind::Edge => "EDGE".to_string(),
            TopOpeBRepDSKind::Vertex => "VERTEX".to_string(),
            TopOpeBRepDSKind::ShapeVertex => "SHAPEVERTEX".to_string(),
            TopOpeBRepDSKind::ShapeEdge => "SHAPEEDGE".to_string(),
            TopOpeBRepDSKind::ShapeFace => "SHAPEFACE".to_string(),
        }
    }

    /// Print kind with index as string
    pub fn sprint_kind_with_index(kind: TopOpeBRepDSKind, index: i32) -> String {
        format!("{}({})", Self::sprint_kind(kind), index)
    }

    /// Print shape type as string
    pub fn sprint_shape(shape: TopAbsShapeEnum) -> String {
        match shape {
            TopAbsShapeEnum::Compound => "COMPOUND".to_string(),
            TopAbsShapeEnum::CompSolid => "COMPSOLID".to_string(),
            TopAbsShapeEnum::Solid => "SOLID".to_string(),
            TopAbsShapeEnum::Shell => "SHELL".to_string(),
            TopAbsShapeEnum::Face => "FACE".to_string(),
            TopAbsShapeEnum::Wire => "WIRE".to_string(),
            TopAbsShapeEnum::Edge => "EDGE".to_string(),
            TopAbsShapeEnum::Vertex => "VERTEX".to_string(),
            TopAbsShapeEnum::Shape => "SHAPE".to_string(),
        }
    }

    /// Print shape with index as string
    pub fn sprint_shape_with_index(shape: TopAbsShapeEnum, index: i32) -> String {
        format!("{}({})", Self::sprint_shape(shape), index)
    }

    /// Print orientation as string
    pub fn sprint_orientation(orient: TopAbsOrientation) -> String {
        match orient {
            TopAbsOrientation::Forward => "FORWARD".to_string(),
            TopAbsOrientation::Reversed => "REVERSED".to_string(),
            TopAbsOrientation::Internal => "INTERNAL".to_string(),
            TopAbsOrientation::External => "EXTERNAL".to_string(),
        }
    }

    /// Print configuration as string
    pub fn sprint_config(config: TopOpeBRepDSConfig) -> String {
        match config {
            TopOpeBRepDSConfig::Unshaded => "UNSHADED".to_string(),
            TopOpeBRepDSConfig::Shaded => "SHADED".to_string(),
        }
    }

    /// Check if kind is a geometry type
    pub fn is_geometry(kind: TopOpeBRepDSKind) -> bool {
        matches!(
            kind,
            TopOpeBRepDSKind::Point
                | TopOpeBRepDSKind::Curve
                | TopOpeBRepDSKind::Surface
        )
    }

    /// Check if kind is a topology type
    pub fn is_topology(kind: TopOpeBRepDSKind) -> bool {
        matches!(
            kind,
            TopOpeBRepDSKind::Solid
                | TopOpeBRepDSKind::Shell
                | TopOpeBRepDSKind::Face
                | TopOpeBRepDSKind::Wire
                | TopOpeBRepDSKind::Edge
                | TopOpeBRepDSKind::Vertex
                | TopOpeBRepDSKind::ShapeVertex
                | TopOpeBRepDSKind::ShapeEdge
                | TopOpeBRepDSKind::ShapeFace
        )
    }

    /// Convert kind to shape enumeration
    pub fn kind_to_shape(kind: TopOpeBRepDSKind) -> Option<TopAbsShapeEnum> {
        match kind {
            TopOpeBRepDSKind::Solid => Some(TopAbsShapeEnum::Solid),
            TopOpeBRepDSKind::Shell => Some(TopAbsShapeEnum::Shell),
            TopOpeBRepDSKind::Face => Some(TopAbsShapeEnum::Face),
            TopOpeBRepDSKind::Wire => Some(TopAbsShapeEnum::Wire),
            TopOpeBRepDSKind::Edge => Some(TopAbsShapeEnum::Edge),
            TopOpeBRepDSKind::Vertex => Some(TopAbsShapeEnum::Vertex),
            _ => None,
        }
    }

    /// Convert shape enumeration to kind
    pub fn shape_to_kind(shape: TopAbsShapeEnum) -> Option<TopOpeBRepDSKind> {
        match shape {
            TopAbsShapeEnum::Solid => Some(TopOpeBRepDSKind::Solid),
            TopAbsShapeEnum::Shell => Some(TopOpeBRepDSKind::Shell),
            TopAbsShapeEnum::Face => Some(TopOpeBRepDSKind::Face),
            TopAbsShapeEnum::Wire => Some(TopOpeBRepDSKind::Wire),
            TopAbsShapeEnum::Edge => Some(TopOpeBRepDSKind::Edge),
            TopAbsShapeEnum::Vertex => Some(TopOpeBRepDSKind::Vertex),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprint_state() {
        assert_eq!(TopOpeBRepDS::sprint_state(TopAbsState::In), "IN");
        assert_eq!(TopOpeBRepDS::sprint_state(TopAbsState::Out), "OU");
        assert_eq!(TopOpeBRepDS::sprint_state(TopAbsState::On), "ON");
        assert_eq!(TopOpeBRepDS::sprint_state(TopAbsState::Unknown), "UN");
    }

    #[test]
    fn test_sprint_kind() {
        assert_eq!(TopOpeBRepDS::sprint_kind(TopOpeBRepDSKind::Point), "POINT");
        assert_eq!(TopOpeBRepDS::sprint_kind(TopOpeBRepDSKind::Curve), "CURVE");
        assert_eq!(TopOpeBRepDS::sprint_kind(TopOpeBRepDSKind::Vertex), "VERTEX");
    }

    #[test]
    fn test_sprint_kind_with_index() {
        assert_eq!(
            TopOpeBRepDS::sprint_kind_with_index(TopOpeBRepDSKind::Point, 5),
            "POINT(5)"
        );
    }

    #[test]
    fn test_sprint_shape() {
        assert_eq!(TopOpeBRepDS::sprint_shape(TopAbsShapeEnum::Face), "FACE");
        assert_eq!(TopOpeBRepDS::sprint_shape(TopAbsShapeEnum::Vertex), "VERTEX");
    }

    #[test]
    fn test_is_geometry() {
        assert!(TopOpeBRepDS::is_geometry(TopOpeBRepDSKind::Point));
        assert!(TopOpeBRepDS::is_geometry(TopOpeBRepDSKind::Curve));
        assert!(TopOpeBRepDS::is_geometry(TopOpeBRepDSKind::Surface));
        assert!(!TopOpeBRepDS::is_geometry(TopOpeBRepDSKind::Vertex));
    }

    #[test]
    fn test_is_topology() {
        assert!(TopOpeBRepDS::is_topology(TopOpeBRepDSKind::Vertex));
        assert!(TopOpeBRepDS::is_topology(TopOpeBRepDSKind::Face));
        assert!(!TopOpeBRepDS::is_topology(TopOpeBRepDSKind::Point));
    }

    #[test]
    fn test_kind_to_shape() {
        assert_eq!(
            TopOpeBRepDS::kind_to_shape(TopOpeBRepDSKind::Face),
            Some(TopAbsShapeEnum::Face)
        );
        assert_eq!(TopOpeBRepDS::kind_to_shape(TopOpeBRepDSKind::Point), None);
    }

    #[test]
    fn test_shape_to_kind() {
        assert_eq!(
            TopOpeBRepDS::shape_to_kind(TopAbsShapeEnum::Vertex),
            Some(TopOpeBRepDSKind::Vertex)
        );
        assert_eq!(
            TopOpeBRepDS::shape_to_kind(TopAbsShapeEnum::Compound),
            None
        );
    }
}
