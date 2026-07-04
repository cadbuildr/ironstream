// FILE: iges_solid_protocol.rs
// occt: IGESSolid_Protocol

/// IGESSolid Protocol provides protocol support for IGESSolid entities.
/// This is the protocol definition that manages registered types and resources.
pub struct IGESSolidProtocol {
    // Base protocol functionality (simplified for core Rust port)
}

impl IGESSolidProtocol {
    /// Creates a new IGESSolid Protocol
    pub fn new() -> Self {
        Self {}
    }

    /// Gives the count of Resource Protocols.
    /// Returns the number of resource protocols (IGESGeom has one).
    pub fn nb_resources(&self) -> usize {
        1
    }

    /// Returns a resource protocol given a rank (1-indexed).
    /// For IGESSolid, rank 1 returns the IGESGeom protocol.
    pub fn resource(&self, num: usize) -> Option<String> {
        match num {
            1 => Some("IGESGeom_Protocol".to_string()),
            _ => None,
        }
    }

    /// Returns a Type Number for a given type name.
    /// Each recognized type maps to a unique case number used by modules.
    pub fn type_number(&self, type_name: &str) -> Option<i32> {
        match type_name {
            "IGESSolid_Block" => Some(150),
            "IGESSolid_ConeFrustum" => Some(151),
            "IGESSolid_RightAngularWedge" => Some(152),
            "IGESSolid_Cylinder" => Some(153),
            "IGESSolid_Cone" => Some(154),
            "IGESSolid_Sphere" => Some(155),
            "IGESSolid_Torus" => Some(156),
            "IGESSolid_SolidOfRevolution" => Some(157),
            "IGESSolid_SolidOfLinearExtrusion" => Some(158),
            "IGESSolid_EdgeList" => Some(504),
            "IGESSolid_VertexList" => Some(505),
            "IGESSolid_Loop" => Some(508),
            "IGESSolid_Face" => Some(510),
            "IGESSolid_Shell" => Some(514),
            "IGESSolid_ManifoldSolid" => Some(186),
            _ => None,
        }
    }
}

impl Default for IGESSolidProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let proto = IGESSolidProtocol::new();
        assert_eq!(proto.nb_resources(), 1);
    }

    #[test]
    fn test_resource_lookup() {
        let proto = IGESSolidProtocol::new();
        assert_eq!(proto.resource(1), Some("IGESGeom_Protocol".to_string()));
        assert_eq!(proto.resource(2), None);
        assert_eq!(proto.resource(0), None);
    }

    #[test]
    fn test_type_number_lookup() {
        let proto = IGESSolidProtocol::new();
        assert_eq!(proto.type_number("IGESSolid_Block"), Some(150));
        assert_eq!(proto.type_number("IGESSolid_Sphere"), Some(155));
        assert_eq!(proto.type_number("IGESSolid_ManifoldSolid"), Some(186));
        assert_eq!(proto.type_number("UnknownType"), None);
    }
}
