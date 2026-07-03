// FILE: bi_tgte_contact_type.rs
// occt: BiTgte_ContactType

/// Enumeration representing the type of contact between two elements.
/// Mirrors OpenCascade BiTgte_ContactType enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BiTgteContactType {
    /// Face-Face contact
    FaceFace = 0,
    /// Face-Edge contact
    FaceEdge = 1,
    /// Face-Vertex contact
    FaceVertex = 2,
    /// Edge-Edge contact
    EdgeEdge = 3,
    /// Edge-Vertex contact
    EdgeVertex = 4,
    /// Vertex-Vertex contact
    VertexVertex = 5,
}

impl BiTgteContactType {
    /// Converts an integer to a contact type.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(BiTgteContactType::FaceFace),
            1 => Some(BiTgteContactType::FaceEdge),
            2 => Some(BiTgteContactType::FaceVertex),
            3 => Some(BiTgteContactType::EdgeEdge),
            4 => Some(BiTgteContactType::EdgeVertex),
            5 => Some(BiTgteContactType::VertexVertex),
            _ => None,
        }
    }

    /// Converts a contact type to an integer.
    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_type_enum() {
        assert_eq!(BiTgteContactType::FaceFace.to_int(), 0);
        assert_eq!(BiTgteContactType::FaceEdge.to_int(), 1);
        assert_eq!(BiTgteContactType::FaceVertex.to_int(), 2);
        assert_eq!(BiTgteContactType::EdgeEdge.to_int(), 3);
        assert_eq!(BiTgteContactType::EdgeVertex.to_int(), 4);
        assert_eq!(BiTgteContactType::VertexVertex.to_int(), 5);
    }

    #[test]
    fn test_contact_type_from_int() {
        assert_eq!(
            BiTgteContactType::from_int(0),
            Some(BiTgteContactType::FaceFace)
        );
        assert_eq!(
            BiTgteContactType::from_int(1),
            Some(BiTgteContactType::FaceEdge)
        );
        assert_eq!(BiTgteContactType::from_int(6), None);
    }

    #[test]
    fn test_contact_type_equality() {
        let ct1 = BiTgteContactType::FaceFace;
        let ct2 = BiTgteContactType::FaceFace;
        assert_eq!(ct1, ct2);
    }
}
