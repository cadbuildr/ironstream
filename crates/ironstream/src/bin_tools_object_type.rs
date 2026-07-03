// FILE: bin_tools_object_type.rs
// occt: BinTools_ObjectType

/// Enumeration of object types for binary serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinToolsObjectType {
    Empty = 0,
    Vertex = 1,
    Edge = 2,
    Wire = 3,
    Face = 4,
    Shell = 5,
    Solid = 6,
    Compound = 7,
    CompoundSolid = 8,
}

impl BinToolsObjectType {
    /// Get the type as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            BinToolsObjectType::Empty => "Empty",
            BinToolsObjectType::Vertex => "Vertex",
            BinToolsObjectType::Edge => "Edge",
            BinToolsObjectType::Wire => "Wire",
            BinToolsObjectType::Face => "Face",
            BinToolsObjectType::Shell => "Shell",
            BinToolsObjectType::Solid => "Solid",
            BinToolsObjectType::Compound => "Compound",
            BinToolsObjectType::CompoundSolid => "CompoundSolid",
        }
    }

    /// Create from a numeric value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(BinToolsObjectType::Empty),
            1 => Some(BinToolsObjectType::Vertex),
            2 => Some(BinToolsObjectType::Edge),
            3 => Some(BinToolsObjectType::Wire),
            4 => Some(BinToolsObjectType::Face),
            5 => Some(BinToolsObjectType::Shell),
            6 => Some(BinToolsObjectType::Solid),
            7 => Some(BinToolsObjectType::Compound),
            8 => Some(BinToolsObjectType::CompoundSolid),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(BinToolsObjectType::Vertex.as_str(), "Vertex");
        assert_eq!(BinToolsObjectType::Edge.as_str(), "Edge");
        assert_eq!(BinToolsObjectType::Face.as_str(), "Face");
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(BinToolsObjectType::from_u8(0), Some(BinToolsObjectType::Empty));
        assert_eq!(BinToolsObjectType::from_u8(1), Some(BinToolsObjectType::Vertex));
        assert_eq!(BinToolsObjectType::from_u8(99), None);
    }

    #[test]
    fn test_ordering() {
        assert!(BinToolsObjectType::Vertex < BinToolsObjectType::Edge);
        assert!(BinToolsObjectType::Face > BinToolsObjectType::Edge);
    }
}
