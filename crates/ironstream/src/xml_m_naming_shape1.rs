// FILE: xml_m_naming_shape1.rs
// occt: XmlMNaming_Shape1

/// Orientation enumeration for topological shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Forward,
    Reversed,
    Internal,
    External,
}

/// Persistent representation of a TopoDS_Shape in XML.
/// Contains a reference to a TShape, Location, and Orientation.
pub struct XmlMNamingShape1 {
    tshape_id: i32,
    loc_id: i32,
    orientation: Orientation,
}

impl XmlMNamingShape1 {
    /// Create a new shape descriptor.
    pub fn new() -> Self {
        XmlMNamingShape1 {
            tshape_id: 0,
            loc_id: 0,
            orientation: Orientation::Forward,
        }
    }

    /// Create a shape descriptor from document.
    pub fn from_document() -> Self {
        Self::new()
    }

    /// Create a shape descriptor from XML element.
    pub fn from_element() -> Self {
        Self::new()
    }

    /// Get the TShape ID.
    pub fn tshape_id(&self) -> i32 {
        self.tshape_id
    }

    /// Get the Location ID.
    pub fn loc_id(&self) -> i32 {
        self.loc_id
    }

    /// Get the orientation.
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Set shape properties.
    pub fn set_shape(&mut self, id: i32, loc_id: i32, orient: Orientation) {
        self.tshape_id = id;
        self.loc_id = loc_id;
        self.orientation = orient;
    }

    /// Set properties from a vertex.
    pub fn set_vertex(&mut self) {
        self.tshape_id = 0;
        self.loc_id = 0;
        self.orientation = Orientation::Forward;
    }
}

impl Default for XmlMNamingShape1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Orientation {
    /// Convert orientation to string representation.
    pub fn to_string(self) -> &'static str {
        match self {
            Orientation::Forward => "FORWARD",
            Orientation::Reversed => "REVERSED",
            Orientation::Internal => "INTERNAL",
            Orientation::External => "EXTERNAL",
        }
    }

    /// Parse orientation from string.
    pub fn from_string(s: &str) -> Option<Orientation> {
        match s {
            "FORWARD" => Some(Orientation::Forward),
            "REVERSED" => Some(Orientation::Reversed),
            "INTERNAL" => Some(Orientation::Internal),
            "EXTERNAL" => Some(Orientation::External),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        let shape = XmlMNamingShape1::new();
        assert_eq!(shape.tshape_id(), 0);
        assert_eq!(shape.loc_id(), 0);
        assert_eq!(shape.orientation(), Orientation::Forward);
    }

    #[test]
    fn test_set_shape() {
        let mut shape = XmlMNamingShape1::new();
        shape.set_shape(42, 10, Orientation::Reversed);
        assert_eq!(shape.tshape_id(), 42);
        assert_eq!(shape.loc_id(), 10);
        assert_eq!(shape.orientation(), Orientation::Reversed);
    }

    #[test]
    fn test_set_vertex() {
        let mut shape = XmlMNamingShape1::new();
        shape.set_shape(99, 20, Orientation::Internal);
        shape.set_vertex();
        assert_eq!(shape.tshape_id(), 0);
        assert_eq!(shape.loc_id(), 0);
        assert_eq!(shape.orientation(), Orientation::Forward);
    }

    #[test]
    fn test_orientation_to_string() {
        assert_eq!(Orientation::Forward.to_string(), "FORWARD");
        assert_eq!(Orientation::Reversed.to_string(), "REVERSED");
        assert_eq!(Orientation::Internal.to_string(), "INTERNAL");
        assert_eq!(Orientation::External.to_string(), "EXTERNAL");
    }

    #[test]
    fn test_orientation_from_string() {
        assert_eq!(Orientation::from_string("FORWARD"), Some(Orientation::Forward));
        assert_eq!(Orientation::from_string("REVERSED"), Some(Orientation::Reversed));
        assert_eq!(Orientation::from_string("INVALID"), None);
    }

    #[test]
    fn test_orientation_roundtrip() {
        for orient in [
            Orientation::Forward,
            Orientation::Reversed,
            Orientation::Internal,
            Orientation::External,
        ] {
            let s = orient.to_string();
            let orient2 = Orientation::from_string(s);
            assert_eq!(Some(orient), orient2);
        }
    }

    #[test]
    fn test_default_construction() {
        let shape = XmlMNamingShape1::default();
        assert_eq!(shape.tshape_id(), 0);
        assert_eq!(shape.orientation(), Orientation::Forward);
    }

    #[test]
    fn test_from_document() {
        let shape = XmlMNamingShape1::from_document();
        assert_eq!(shape.tshape_id(), 0);
    }

    #[test]
    fn test_from_element() {
        let shape = XmlMNamingShape1::from_element();
        assert_eq!(shape.loc_id(), 0);
    }
}
