// FILE: vrml_vertex_ordering.rs
// occt: Vrml_VertexOrdering
//
// Faithful port of OCCT Vrml_VertexOrdering (DataExchange/TKDEVRML/Vrml/
// Vrml_VertexOrdering.hxx/.cxx): enumeration for vertex ordering in VRML.

/// Port of Vrml_VertexOrdering enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlVertexOrdering {
    UnknownOrdering,
    Clockwise,
    Counterclockwise,
}

impl VrmlVertexOrdering {
    pub fn to_string(&self) -> &'static str {
        match self {
            VrmlVertexOrdering::UnknownOrdering => "UNKNOWN_ORDERING",
            VrmlVertexOrdering::Clockwise => "CLOCKWISE",
            VrmlVertexOrdering::Counterclockwise => "COUNTERCLOCKWISE",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "UNKNOWN_ORDERING" => Some(VrmlVertexOrdering::UnknownOrdering),
            "CLOCKWISE" => Some(VrmlVertexOrdering::Clockwise),
            "COUNTERCLOCKWISE" => Some(VrmlVertexOrdering::Counterclockwise),
            _ => None,
        }
    }
}

impl std::fmt::Display for VrmlVertexOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_to_string() {
        assert_eq!(VrmlVertexOrdering::UnknownOrdering.to_string(), "UNKNOWN_ORDERING");
    }

    #[test]
    fn clockwise_to_string() {
        assert_eq!(VrmlVertexOrdering::Clockwise.to_string(), "CLOCKWISE");
    }

    #[test]
    fn counterclockwise_to_string() {
        assert_eq!(VrmlVertexOrdering::Counterclockwise.to_string(), "COUNTERCLOCKWISE");
    }

    #[test]
    fn from_string_unknown() {
        let ord = VrmlVertexOrdering::from_string("UNKNOWN_ORDERING");
        assert_eq!(ord, Some(VrmlVertexOrdering::UnknownOrdering));
    }

    #[test]
    fn from_string_clockwise() {
        let ord = VrmlVertexOrdering::from_string("CLOCKWISE");
        assert_eq!(ord, Some(VrmlVertexOrdering::Clockwise));
    }

    #[test]
    fn from_string_counterclockwise() {
        let ord = VrmlVertexOrdering::from_string("COUNTERCLOCKWISE");
        assert_eq!(ord, Some(VrmlVertexOrdering::Counterclockwise));
    }

    #[test]
    fn from_string_lowercase() {
        let ord = VrmlVertexOrdering::from_string("clockwise");
        assert_eq!(ord, Some(VrmlVertexOrdering::Clockwise));
    }

    #[test]
    fn from_string_invalid() {
        let ord = VrmlVertexOrdering::from_string("INVALID");
        assert_eq!(ord, None);
    }

    #[test]
    fn display_trait() {
        let unknown = VrmlVertexOrdering::UnknownOrdering;
        let clockwise = VrmlVertexOrdering::Clockwise;
        let counterclockwise = VrmlVertexOrdering::Counterclockwise;
        assert_eq!(format!("{}", unknown), "UNKNOWN_ORDERING");
        assert_eq!(format!("{}", clockwise), "CLOCKWISE");
        assert_eq!(format!("{}", counterclockwise), "COUNTERCLOCKWISE");
    }

    #[test]
    fn equality() {
        let a = VrmlVertexOrdering::Clockwise;
        let b = VrmlVertexOrdering::Clockwise;
        let c = VrmlVertexOrdering::Counterclockwise;
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VrmlVertexOrdering::Clockwise);
        set.insert(VrmlVertexOrdering::Counterclockwise);
        assert!(set.contains(&VrmlVertexOrdering::Clockwise));
    }
}
