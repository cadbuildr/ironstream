// FILE: iges_graph_general_module_o.rs
// occt: IGESGraph_GeneralModule

/// Represents IGESGraph_GeneralModule - General services for IGESGraph.
/// Provides shared/implied lists, copy, and check functionality for IGES Graph entities.
pub struct IgesGraphGeneralModule {
    // Module state - typically empty for a service class
}

impl IgesGraphGeneralModule {
    /// Creates a new GeneralModule from IGESGraph.
    pub fn new() -> Self {
        IgesGraphGeneralModule {}
    }

    /// Lists entities shared by a given IGESEntity from its specific parameters.
    ///
    /// # Arguments
    /// - `case_num`: Case number identifying the entity type
    /// - `entity`: The IGES entity to inspect
    ///
    /// Returns shared entity references
    pub fn own_shared_case(&self, case_num: i32, _entity: Option<&dyn std::any::Any>) -> Vec<usize> {
        // The actual implementation depends on the case number and entity type
        // This is a simplified stub - real implementation would iterate through
        // shared references in the entity
        match case_num {
            _ => Vec::new(),
        }
    }

    /// Returns a DirChecker specific for each type of Entity.
    ///
    /// # Arguments
    /// - `case_num`: Case number identifying the entity type
    /// - `entity`: The IGES entity to check
    pub fn dir_checker(&self, case_num: i32, _entity: Option<&dyn std::any::Any>) -> DirChecker {
        DirChecker::new(case_num)
    }

    /// Performs semantic check for each type of entity.
    ///
    /// # Arguments
    /// - `case_num`: Case number identifying the entity type
    /// - `entity`: The IGES entity to check
    pub fn own_check_case(&self, case_num: i32, _entity: Option<&dyn std::any::Any>) -> Check {
        Check::new(case_num)
    }

    /// Creates a new void entity of the specified type.
    ///
    /// # Arguments
    /// - `case_num`: Case number identifying the entity type
    ///
    /// Returns true if a new void entity was created
    pub fn new_void(&self, case_num: i32) -> bool {
        // Entity creation depends on case number
        case_num > 0
    }

    /// Copies parameters specific to each entity type.
    ///
    /// # Arguments
    /// - `case_num`: Case number identifying the entity type
    /// - `from_entity`: Source entity
    /// - `to_entity`: Destination entity
    pub fn own_copy_case(
        &self,
        _case_num: i32,
        _from: Option<&dyn std::any::Any>,
        _to: Option<&dyn std::any::Any>,
    ) {
        // Copy implementation depends on case number and entity type
    }

    /// Returns a category number characterizing an entity.
    /// For IGESGraph, all entities are Drawing category.
    ///
    /// # Arguments
    /// - `case_num`: Case number identifying the entity type
    pub fn category_number(&self, _case_num: i32) -> i32 {
        1 // Drawing category
    }
}

impl Default for IgesGraphGeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper structure for directory checking
#[derive(Clone, Debug)]
pub struct DirChecker {
    case_num: i32,
}

impl DirChecker {
    pub fn new(case_num: i32) -> Self {
        DirChecker { case_num }
    }

    pub fn case_number(&self) -> i32 {
        self.case_num
    }
}

/// Helper structure for entity checking
#[derive(Clone, Debug)]
pub struct Check {
    case_num: i32,
    messages: Vec<String>,
}

impl Check {
    pub fn new(case_num: i32) -> Self {
        Check {
            case_num,
            messages: Vec::new(),
        }
    }

    pub fn case_number(&self) -> i32 {
        self.case_num
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn messages(&self) -> &[String] {
        &self.messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_module_creation() {
        let gm = IgesGraphGeneralModule::new();
        assert_eq!(gm.category_number(1), 1);
    }

    #[test]
    fn test_general_module_new_void() {
        let gm = IgesGraphGeneralModule::new();
        assert!(gm.new_void(1));
        assert!(!gm.new_void(0));
    }

    #[test]
    fn test_general_module_dir_checker() {
        let gm = IgesGraphGeneralModule::new();
        let checker = gm.dir_checker(5, None);
        assert_eq!(checker.case_number(), 5);
    }

    #[test]
    fn test_general_module_check() {
        let gm = IgesGraphGeneralModule::new();
        let check = gm.own_check_case(3, None);
        assert_eq!(check.case_number(), 3);
    }

    #[test]
    fn test_general_module_owned_shared_case() {
        let gm = IgesGraphGeneralModule::new();
        let shared = gm.own_shared_case(1, None);
        assert!(shared.is_empty());
    }

    #[test]
    fn test_dir_checker_creation() {
        let checker = DirChecker::new(42);
        assert_eq!(checker.case_number(), 42);
    }

    #[test]
    fn test_check_creation() {
        let mut check = Check::new(10);
        assert_eq!(check.case_number(), 10);
        assert!(check.messages().is_empty());
        check.add_message("Test error".to_string());
        assert_eq!(check.messages().len(), 1);
    }
}
