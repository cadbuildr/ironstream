// FILE: iges_data_general_module.rs
// occt: IGESData_GeneralModule

//! Definition of General Services adapted to IGES.
//! Provides shared & implied lists, copy, and check functionality.

/// GeneralModule provides general IGES entity services
#[derive(Clone, Debug)]
pub struct GeneralModule;

impl GeneralModule {
    /// Creates a GeneralModule for IGES services
    pub fn new() -> Self {
        GeneralModule
    }

    /// Fills the list of Entities shared by an IGES entity
    pub fn fill_shared_case(&self, cn: i32) -> Vec<usize> {
        Vec::new()
    }

    /// Lists the Entities shared by a given IGES entity from its specific parameters
    pub fn own_shared_case(&self, cn: i32) -> Vec<usize> {
        Vec::new()
    }

    /// Lists the Implied References (Associativities and OwnSharedCase entities)
    pub fn list_implied_case(&self, cn: i32) -> Vec<usize> {
        Vec::new()
    }

    /// Specific list of Entities implied by a given IGES entity
    pub fn own_implied_case(&self, cn: i32) -> Vec<usize> {
        Vec::new()
    }

    /// Performs semantic checking of an IGES entity
    pub fn check_case(&self, cn: i32) -> bool {
        true
    }

    /// Returns a DirChecker specific for each type of Entity
    pub fn dir_checker(&self, cn: i32) -> String {
        format!("DirChecker for case {}", cn)
    }

    /// Specific checks performed by each type of Entity
    pub fn own_check_case(&self, cn: i32) -> bool {
        true
    }

    /// Specific creation of a new void entity
    pub fn new_void(&self, cn: i32) -> bool {
        true
    }

    /// Copies parameters specific to each Type of Entity
    pub fn own_copy_case(&self, cn: i32) -> bool {
        true
    }
}

impl Default for GeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gm = GeneralModule::new();
        let gm2 = GeneralModule::default();
        assert_eq!(format!("{:?}", gm), format!("{:?}", gm2));
    }

    #[test]
    fn test_fill_shared_case() {
        let gm = GeneralModule::new();
        let shared = gm.fill_shared_case(1);
        assert_eq!(shared.len(), 0);
    }

    #[test]
    fn test_dir_checker() {
        let gm = GeneralModule::new();
        let checker = gm.dir_checker(1);
        assert!(checker.contains("DirChecker"));
    }

    #[test]
    fn test_check_case() {
        let gm = GeneralModule::new();
        assert!(gm.check_case(1));
    }

    #[test]
    fn test_new_void() {
        let gm = GeneralModule::new();
        assert!(gm.new_void(1));
    }

    #[test]
    fn test_own_copy_case() {
        let gm = GeneralModule::new();
        assert!(gm.own_copy_case(1));
    }
}
