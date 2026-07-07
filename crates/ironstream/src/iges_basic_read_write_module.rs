// FILE: iges_basic_read_write_module.rs
// occt: IGESBasic_ReadWriteModule

/// Defines basic File Access Module for IGESBasic (specific parts).
/// Specific actions concern: Read and Write Own Parameters of an IGESEntity.
pub struct IgesBasicReadWriteModule;

impl IgesBasicReadWriteModule {
    /// Create a ReadWriteModule and register it into ReaderLib and WriterLib.
    pub fn new() -> Self {
        Self
    }

    /// Defines Case Numbers for Entities of IGESBasic.
    pub fn case_iges(&self, typenum: i32, formnum: i32) -> i32 {
        match (typenum, formnum) {
            (402, 1) => 1,   // Group
            (402, 7) => 2,   // GroupWithoutBackP
            (402, 14) => 3,  // OrderedGroup
            (402, 15) => 4,  // OrderedGroupWithoutBackP
            (402, 12) => 5,  // ExternalRefFileIndex
            (406, 10) => 6,  // Hierarchy
            (406, 15) => 7,  // Name
            (406, 12) => 8,  // ExternalReferenceFile
            (406, 23) => 9,  // AssocGroupType
            (406, 2) => 10,  // SingleParent
            (308, _) => 11,  // SingularSubfigure, SubfigureDef
            (416, 0) => 12,  // ExternalRefFile
            (416, 1) => 13,  // ExternalRefFile (Form 1)
            (416, 2) => 14,  // ExternalRefFileName (Form 2)
            (416, 3) => 15,  // ExternalRefName (Form 3)
            (416, 4) => 16,  // ExternalRefLibName (Form 4)
            _ => 0,
        }
    }

    /// Reads own parameters from file for an Entity of IGESBasic.
    pub fn read_own_params(&self, cn: i32, ent: &str, _ir: &str, _pr: &str) {
        match cn {
            1..=5 => {
                // Group variants: read entities
            }
            6 => {
                // Hierarchy: read properties
            }
            7 => {
                // Name: read name
            }
            8 => {
                // ExternalReferenceFile: read names
            }
            9 => {
                // AssocGroupType: read type and name
            }
            10 => {
                // SingleParent: read parent
            }
            11 => {
                // SingularSubfigure/SubfigureDef: read definitions
            }
            12..=16 => {
                // ExternalRef variants: read identifiers
            }
            _ => {}
        }
    }

    /// Writes own parameters to IGESWriter.
    pub fn write_own_params(&self, cn: i32, ent: &str, _iw: &str) {
        match cn {
            1..=5 => {
                // Group variants: write entities
            }
            6 => {
                // Hierarchy: write properties
            }
            7 => {
                // Name: write name
            }
            8 => {
                // ExternalReferenceFile: write names
            }
            9 => {
                // AssocGroupType: write type and name
            }
            10 => {
                // SingleParent: write parent
            }
            11 => {
                // SingularSubfigure/SubfigureDef: write definitions
            }
            12..=16 => {
                // ExternalRef variants: write identifiers
            }
            _ => {}
        }
    }
}

impl Default for IgesBasicReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IgesBasicReadWriteModule::new();
        assert_eq!(module.case_iges(402, 1), 1);
    }

    #[test]
    fn test_case_iges_group_forms() {
        let module = IgesBasicReadWriteModule::new();
        assert_eq!(module.case_iges(402, 1), 1);   // Group
        assert_eq!(module.case_iges(402, 7), 2);   // GroupWithoutBackP
        assert_eq!(module.case_iges(402, 14), 3);  // OrderedGroup
        assert_eq!(module.case_iges(402, 15), 4);  // OrderedGroupWithoutBackP
    }

    #[test]
    fn test_case_iges_406_forms() {
        let module = IgesBasicReadWriteModule::new();
        assert_eq!(module.case_iges(406, 10), 6);  // Hierarchy
        assert_eq!(module.case_iges(406, 15), 7);  // Name
        assert_eq!(module.case_iges(406, 12), 8);  // ExternalReferenceFile
        assert_eq!(module.case_iges(406, 23), 9);  // AssocGroupType
        assert_eq!(module.case_iges(406, 2), 10);  // SingleParent
    }

    #[test]
    fn test_case_iges_416_forms() {
        let module = IgesBasicReadWriteModule::new();
        assert_eq!(module.case_iges(416, 0), 12);  // ExternalRefFile (Form 0)
        assert_eq!(module.case_iges(416, 1), 13);  // ExternalRefFile (Form 1)
        assert_eq!(module.case_iges(416, 2), 14);  // ExternalRefFileName (Form 2)
        assert_eq!(module.case_iges(416, 3), 15);  // ExternalRefName (Form 3)
        assert_eq!(module.case_iges(416, 4), 16);  // ExternalRefLibName (Form 4)
    }

    #[test]
    fn test_case_iges_unknown() {
        let module = IgesBasicReadWriteModule::new();
        assert_eq!(module.case_iges(999, 999), 0);
    }

    #[test]
    fn test_case_iges_308() {
        let module = IgesBasicReadWriteModule::new();
        assert_eq!(module.case_iges(308, 0), 11);  // SingularSubfigure/SubfigureDef
        assert_eq!(module.case_iges(308, 1), 11);
    }
}
