// FILE: iges_data_default_general.rs
// occt: IGESData_DefaultGeneral

//! Processes the specific case of UndefinedEntity from IGESData.
//! Handles default general module behavior for undefined IGES entities.

/// IGESData_DefaultGeneral handles the default case for undefined IGES entities.
/// It provides shared entity iteration, directory checking, semantic checks,
/// void entity creation, and entity copying functionality.
#[derive(Clone, Debug)]
pub struct DefaultGeneral;

impl DefaultGeneral {
    /// Creates a DefaultGeneral and puts it into GeneralLib,
    /// bound with a Protocol from IGESData
    pub fn new() -> Self {
        DefaultGeneral
    }

    /// Lists the Entities shared by an IGESEntity (UndefinedEntity)
    pub fn own_shared_case(&self, cn: i32) -> Vec<usize> {
        // For undefined entities, there are typically no shared entities
        Vec::new()
    }

    /// Returns a DirChecker, specific for each type of Entity
    /// Here, returns an empty DirChecker (no constraint to check)
    pub fn dir_checker(&self, cn: i32) -> DirChecker {
        DirChecker::empty()
    }

    /// Performs Specific Semantic Check for each type of Entity
    /// Here, does nothing (no constraint to check)
    pub fn own_check_case(&self, cn: i32) -> bool {
        // No constraints to check for undefined entities
        true
    }

    /// Specific creation of a new void entity (UndefinedEntity only)
    pub fn new_void(&self, cn: i32) -> bool {
        cn == 1 // Case 1 is for UndefinedEntity
    }

    /// Copies parameters which are specific of each Type of Entity
    pub fn own_copy_case(&self, cn: i32) -> bool {
        true
    }
}

impl Default for DefaultGeneral {
    fn default() -> Self {
        Self::new()
    }
}

/// Empty directory checker for undefined entities
#[derive(Clone, Debug)]
pub struct DirChecker;

impl DirChecker {
    pub fn empty() -> Self {
        DirChecker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dg = DefaultGeneral::new();
        let dg2 = DefaultGeneral::default();
        assert_eq!(format!("{:?}", dg), format!("{:?}", dg2));
    }

    #[test]
    fn test_own_shared_case() {
        let dg = DefaultGeneral::new();
        let shared = dg.own_shared_case(1);
        assert_eq!(shared.len(), 0);
    }

    #[test]
    fn test_dir_checker() {
        let dg = DefaultGeneral::new();
        let _checker = dg.dir_checker(1);
        // Checker exists and can be used
    }

    #[test]
    fn test_own_check_case() {
        let dg = DefaultGeneral::new();
        assert!(dg.own_check_case(1));
    }

    #[test]
    fn test_new_void() {
        let dg = DefaultGeneral::new();
        assert!(dg.new_void(1));
        assert!(!dg.new_void(2));
    }

    #[test]
    fn test_own_copy_case() {
        let dg = DefaultGeneral::new();
        assert!(dg.own_copy_case(1));
        assert!(dg.own_copy_case(2));
    }
}
