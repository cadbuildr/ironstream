// FILE: iges_basic_protocol.rs
// occt: IGESBasic_Protocol

/// Description of Protocol for IGESBasic.
pub struct IgesBasicProtocol {
    nb_resources: i32,
}

impl IgesBasicProtocol {
    /// Create a new Protocol for IGESBasic.
    pub fn new() -> Self {
        Self { nb_resources: 1 }
    }

    /// Gives the count of Resource Protocol.
    /// Here, one (Protocol from IGESData).
    pub fn nb_resources(&self) -> i32 {
        self.nb_resources
    }

    /// Returns a Resource, given a rank.
    pub fn resource(&self, num: i32) -> Option<String> {
        if num == 1 {
            Some("IGESData_Protocol".to_string())
        } else {
            None
        }
    }

    /// Returns a Case Number, specific of each recognized Type.
    /// This Case Number is then used in Libraries: the various Modules
    /// attached to this class of Protocol must use them in accordance
    /// (for a given value of TypeNumber, they must consider the same Type
    /// as the Protocol defines).
    pub fn type_number(&self, atype: &str) -> i32 {
        match atype {
            "IGESBasic_AssocGroupType" => 406,
            "IGESBasic_ExternalRefFile" => 416,
            "IGESBasic_ExternalRefFileName" => 416,
            "IGESBasic_ExternalRefLibName" => 416,
            "IGESBasic_ExternalRefName" => 416,
            "IGESBasic_ExternalReferenceFile" => 406,
            "IGESBasic_ExternalRefFileIndex" => 402,
            "IGESBasic_Group" => 402,
            "IGESBasic_GroupWithoutBackP" => 402,
            "IGESBasic_OrderedGroup" => 402,
            "IGESBasic_OrderedGroupWithoutBackP" => 402,
            "IGESBasic_Hierarchy" => 406,
            "IGESBasic_Name" => 406,
            "IGESBasic_SingleParent" => 402,
            "IGESBasic_SingularSubfigure" => 308,
            "IGESBasic_SubfigureDef" => 308,
            _ => 0,
        }
    }
}

impl Default for IgesBasicProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let protocol = IgesBasicProtocol::new();
        assert_eq!(protocol.nb_resources(), 1);
    }

    #[test]
    fn test_resource() {
        let protocol = IgesBasicProtocol::new();
        assert_eq!(protocol.resource(1), Some("IGESData_Protocol".to_string()));
        assert_eq!(protocol.resource(2), None);
    }

    #[test]
    fn test_type_number() {
        let protocol = IgesBasicProtocol::new();
        assert_eq!(protocol.type_number("IGESBasic_AssocGroupType"), 406);
        assert_eq!(protocol.type_number("IGESBasic_Group"), 402);
        assert_eq!(protocol.type_number("IGESBasic_ExternalRefFile"), 416);
        assert_eq!(protocol.type_number("Unknown"), 0);
    }

    #[test]
    fn test_known_types() {
        let protocol = IgesBasicProtocol::new();
        assert_ne!(protocol.type_number("IGESBasic_Name"), 0);
        assert_ne!(protocol.type_number("IGESBasic_Hierarchy"), 0);
        assert_ne!(protocol.type_number("IGESBasic_SingleParent"), 0);
    }
}
