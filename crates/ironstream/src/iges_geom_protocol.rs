// FILE: iges_geom_protocol.rs
// occt: IGESGeom_Protocol

/// Description of Protocol for IGESGeom.
/// This protocol defines the case numbers for various entity types
/// and manages resources from parent protocols.
pub struct Protocol {
    /// Number of resource protocols (IGESBasic is the parent)
    nb_resources: i32,
}

impl Protocol {
    /// Creates a new Protocol for IGESGeom.
    pub fn new() -> Self {
        Protocol { nb_resources: 1 }
    }

    /// Returns the count of Resource Protocols. Here, one (IGESBasic).
    pub fn nb_resources(&self) -> i32 {
        self.nb_resources
    }

    /// Returns a Resource protocol ID, given a rank (1-indexed).
    pub fn resource(&self, num: i32) -> Option<i32> {
        if num == 1 {
            Some(1) // IGESBasic protocol resource
        } else {
            None
        }
    }

    /// Returns a Case Number specific to each recognized Type.
    /// This maps entity types to case numbers used by modules.
    pub fn type_number(&self, type_name: &str) -> Option<i32> {
        match type_name {
            "BSplineCurve" => Some(1),
            "BSplineSurface" => Some(2),
            "Boundary" => Some(3),
            "BoundedSurface" => Some(4),
            "CircularArc" => Some(5),
            "CompositeCurve" => Some(6),
            "ConicArc" => Some(7),
            "CopiousData" => Some(8),
            "CurveOnSurface" => Some(9),
            "Direction" => Some(10),
            "Flash" => Some(11),
            "Line" => Some(12),
            "OffsetCurve" => Some(13),
            "OffsetSurface" => Some(14),
            "Plane" => Some(15),
            "Point" => Some(16),
            "RuledSurface" => Some(17),
            "SplineCurve" => Some(18),
            "SplineSurface" => Some(19),
            "SurfaceOfRevolution" => Some(20),
            "TabulatedCylinder" => Some(21),
            "TransformationMatrix" => Some(22),
            "TrimmedSurface" => Some(23),
            _ => None,
        }
    }

    /// Reverse lookup: given a case number, return the type name.
    pub fn case_to_type(&self, case_num: i32) -> Option<&'static str> {
        match case_num {
            1 => Some("BSplineCurve"),
            2 => Some("BSplineSurface"),
            3 => Some("Boundary"),
            4 => Some("BoundedSurface"),
            5 => Some("CircularArc"),
            6 => Some("CompositeCurve"),
            7 => Some("ConicArc"),
            8 => Some("CopiousData"),
            9 => Some("CurveOnSurface"),
            10 => Some("Direction"),
            11 => Some("Flash"),
            12 => Some("Line"),
            13 => Some("OffsetCurve"),
            14 => Some("OffsetSurface"),
            15 => Some("Plane"),
            16 => Some("Point"),
            17 => Some("RuledSurface"),
            18 => Some("SplineCurve"),
            19 => Some("SplineSurface"),
            20 => Some("SurfaceOfRevolution"),
            21 => Some("TabulatedCylinder"),
            22 => Some("TransformationMatrix"),
            23 => Some("TrimmedSurface"),
            _ => None,
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_protocol() {
        let proto = Protocol::new();
        assert_eq!(proto.nb_resources(), 1);
    }

    #[test]
    fn test_resource() {
        let proto = Protocol::new();
        assert_eq!(proto.resource(1), Some(1));
        assert_eq!(proto.resource(2), None);
    }

    #[test]
    fn test_type_number() {
        let proto = Protocol::new();
        assert_eq!(proto.type_number("Line"), Some(12));
        assert_eq!(proto.type_number("Point"), Some(16));
        assert_eq!(proto.type_number("Plane"), Some(15));
        assert_eq!(proto.type_number("Unknown"), None);
    }

    #[test]
    fn test_case_to_type() {
        let proto = Protocol::new();
        assert_eq!(proto.case_to_type(12), Some("Line"));
        assert_eq!(proto.case_to_type(16), Some("Point"));
        assert_eq!(proto.case_to_type(99), None);
    }

    #[test]
    fn test_bidirectional_mapping() {
        let proto = Protocol::new();
        for case in 1..=23 {
            if let Some(type_name) = proto.case_to_type(case) {
                assert_eq!(proto.type_number(type_name), Some(case));
            }
        }
    }
}
