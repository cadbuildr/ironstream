// FILE: iges_basic_specific_module.rs
// occt: IGESBasic_SpecificModule

/// Defines Services attached to IGES Entities:
/// Dump & OwnCorrect, for IGESBasic.
pub struct IgesBasicSpecificModule;

impl IgesBasicSpecificModule {
    /// Create a SpecificModule from IGESBasic and register it into SpecificLib.
    pub fn new() -> Self {
        Self
    }

    /// Specific Dump (own parameters) for IGESBasic.
    pub fn own_dump(&self, cn: i32, ent: &str, _dumper: &str, _own: i32) -> String {
        match cn {
            1 => format!("AssocGroupType: {}", ent),
            2 => format!("ExternalRefFile: {}", ent),
            3 => format!("ExternalRefFileIndex: {}", ent),
            4 => format!("ExternalRefFileName: {}", ent),
            5 => format!("ExternalRefLibName: {}", ent),
            6 => format!("ExternalRefName: {}", ent),
            7 => format!("ExternalReferenceFile: {}", ent),
            8 => format!("Group: {}", ent),
            9 => format!("GroupWithoutBackP: {}", ent),
            10 => format!("Hierarchy: {}", ent),
            11 => format!("Name: {}", ent),
            12 => format!("OrderedGroup: {}", ent),
            13 => format!("OrderedGroupWithoutBackP: {}", ent),
            14 => format!("SingleParent: {}", ent),
            15 => format!("SingularSubfigure: {}", ent),
            16 => format!("SubfigureDef: {}", ent),
            _ => format!("Unknown: {}", ent),
        }
    }

    /// Performs non-ambiguous Corrections on Entities which support them
    /// (AssocGroupType, Hierarchy, Name, SingleParent).
    pub fn own_correct(&self, cn: i32, ent: &str) -> bool {
        match cn {
            1 => true,  // AssocGroupType
            10 => true, // Hierarchy
            11 => true, // Name
            14 => true, // SingleParent
            _ => false,
        }
    }
}

impl Default for IgesBasicSpecificModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IgesBasicSpecificModule::new();
        assert_eq!(module.own_dump(1, "test", "dumper", 0), "AssocGroupType: test");
    }

    #[test]
    fn test_own_dump() {
        let module = IgesBasicSpecificModule::new();
        assert_eq!(module.own_dump(1, "test", "", 0), "AssocGroupType: test");
        assert_eq!(module.own_dump(8, "test", "", 0), "Group: test");
        assert_eq!(module.own_dump(10, "test", "", 0), "Hierarchy: test");
        assert_eq!(module.own_dump(11, "test", "", 0), "Name: test");
    }

    #[test]
    fn test_own_correct_true() {
        let module = IgesBasicSpecificModule::new();
        assert!(module.own_correct(1, "test"));  // AssocGroupType
        assert!(module.own_correct(10, "test")); // Hierarchy
        assert!(module.own_correct(11, "test")); // Name
        assert!(module.own_correct(14, "test")); // SingleParent
    }

    #[test]
    fn test_own_correct_false() {
        let module = IgesBasicSpecificModule::new();
        assert!(!module.own_correct(2, "test"));
        assert!(!module.own_correct(8, "test"));
        assert!(!module.own_correct(99, "test"));
    }
}
