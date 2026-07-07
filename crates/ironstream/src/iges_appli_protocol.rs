// FILE: iges_appli_protocol.rs
// occt: IGESAppli_Protocol

/// Protocol handler for IGESAppli package.
///
/// Manages type identification and resource protocols for application entities.
#[derive(Clone, Debug)]
pub struct IgesAppliProtocol {
    nb_resources: i32,
}

impl IgesAppliProtocol {
    /// Creates a new protocol handler.
    pub fn new() -> Self {
        Self { nb_resources: 2 }
    }

    /// Returns the count of direct resource protocols (IGESDefs and IGESDraw).
    pub fn nb_resources(&self) -> i32 {
        self.nb_resources
    }

    /// Returns a resource protocol by rank.
    pub fn resource(&self, num: i32) -> Option<String> {
        match num {
            1 => Some("IGESDefs_Protocol".to_string()),
            2 => Some("IGESDraw_Protocol".to_string()),
            _ => None,
        }
    }

    /// Returns the type number for a given type string.
    pub fn type_number(&self, type_name: &str) -> i32 {
        match type_name {
            "IGESAppli_DrilledHole" => 406,
            "IGESAppli_Node" => 134,
            "IGESAppli_ElementResults" => 146,
            "IGESAppli_FiniteElement" => 136,
            "IGESAppli_NodalDisplAndRot" => 139,
            "IGESAppli_PartNumber" => 406,
            "IGESAppli_PinNumber" => 406,
            _ => 0,
        }
    }
}

impl Default for IgesAppliProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let protocol = IgesAppliProtocol::new();
        assert_eq!(protocol.nb_resources(), 2);
    }

    #[test]
    fn test_resource() {
        let protocol = IgesAppliProtocol::new();
        assert_eq!(protocol.resource(1), Some("IGESDefs_Protocol".to_string()));
        assert_eq!(protocol.resource(2), Some("IGESDraw_Protocol".to_string()));
        assert_eq!(protocol.resource(3), None);
    }

    #[test]
    fn test_type_number() {
        let protocol = IgesAppliProtocol::new();
        assert_eq!(protocol.type_number("IGESAppli_DrilledHole"), 406);
        assert_eq!(protocol.type_number("IGESAppli_Node"), 134);
    }
}
