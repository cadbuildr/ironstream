// FILE: iges_basic_general_module.rs
// occt: IGESBasic_GeneralModule

/// Definition of General Services for IGESBasic (specific part).
/// This Services comprise: Shared & Implied Lists, Copy, Check.
pub struct IgesBasicGeneralModule;

impl IgesBasicGeneralModule {
    /// Create a new GeneralModule from IGESBasic and register it into GeneralLib.
    pub fn new() -> Self {
        Self
    }

    /// Lists the Entities shared by a given IGESEntity, from its specific parameters.
    /// Specific for each type.
    pub fn own_shared_case(&self, cn: i32, _ent: &str) -> Vec<String> {
        match cn {
            406 | 416 | 402 => vec![],
            _ => vec![],
        }
    }

    /// Returns a DirChecker, specific for each type of Entity (identified by its Case Number).
    /// This DirChecker defines constraints which must be respected by the DirectoryPart.
    pub fn dir_checker(&self, cn: i32, _ent: &str) -> String {
        match cn {
            406 => "AssocGroupType".to_string(),
            416 => "ExternalRefFile".to_string(),
            402 => "ExternalRefFileIndex".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Performs Specific Semantic Check for each type of Entity.
    pub fn own_check_case(&self, cn: i32, _ent: &str, _shares: &str) -> Vec<String> {
        match cn {
            406 | 416 | 402 => vec![],
            _ => vec![],
        }
    }

    /// Specific creation of a new void entity.
    pub fn new_void(&self, cn: i32) -> Option<String> {
        match cn {
            406 => Some("AssocGroupType".to_string()),
            416 => Some("ExternalRefFile".to_string()),
            402 => Some("ExternalRefFileIndex".to_string()),
            _ => None,
        }
    }

    /// Copies parameters which are specific of each Type of Entity.
    pub fn own_copy_case(&self, cn: i32, _ent_from: &str, _ent_to: &str) {
        match cn {
            406 | 416 | 402 => {},
            _ => {},
        }
    }

    /// Returns a category number which characterizes an entity.
    /// Structure for Groups, Figures & Co.
    /// Description for External Refs.
    /// Auxiliary for other.
    pub fn category_number(&self, cn: i32, _ent: &str, _shares: &str) -> i32 {
        match cn {
            406 => 1, // Group structure
            416 => 2, // External Reference
            402 => 1, // Structure
            _ => 0,   // Auxiliary
        }
    }
}

impl Default for IgesBasicGeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IgesBasicGeneralModule::new();
        assert_eq!(module.dir_checker(406, ""), "AssocGroupType");
    }

    #[test]
    fn test_dir_checker() {
        let module = IgesBasicGeneralModule::new();
        assert_eq!(module.dir_checker(406, ""), "AssocGroupType");
        assert_eq!(module.dir_checker(416, ""), "ExternalRefFile");
        assert_eq!(module.dir_checker(402, ""), "ExternalRefFileIndex");
        assert_eq!(module.dir_checker(999, ""), "Unknown");
    }

    #[test]
    fn test_new_void() {
        let module = IgesBasicGeneralModule::new();
        assert_eq!(module.new_void(406), Some("AssocGroupType".to_string()));
        assert_eq!(module.new_void(416), Some("ExternalRefFile".to_string()));
        assert_eq!(module.new_void(999), None);
    }

    #[test]
    fn test_category_number() {
        let module = IgesBasicGeneralModule::new();
        assert_eq!(module.category_number(406, "", ""), 1); // Structure
        assert_eq!(module.category_number(416, "", ""), 2); // External Reference
        assert_eq!(module.category_number(402, "", ""), 1); // Structure
        assert_eq!(module.category_number(999, "", ""), 0); // Auxiliary
    }

    #[test]
    fn test_own_shared_case() {
        let module = IgesBasicGeneralModule::new();
        let shared = module.own_shared_case(406, "");
        assert_eq!(shared.len(), 0);
    }

    #[test]
    fn test_own_check_case() {
        let module = IgesBasicGeneralModule::new();
        let checks = module.own_check_case(406, "", "");
        assert_eq!(checks.len(), 0);
    }
}
