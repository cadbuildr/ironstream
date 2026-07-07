// FILE: iges_select_select_subordinate.rs
// occt: IGESSelect_SelectSubordinate

/// Selects IGES entities based on their subordinate status.
/// Status codes:
/// - 0: Independent
/// - 1: Physically Dependent
/// - 2: Logically Dependent
/// - 3: Both (recorded)
/// - 4: 1 or 3 (at least Physically Dependent)
/// - 5: 2 or 3 (at least Logically Dependent)
/// - 6: 1 or 2 or 3 (any kind of dependence)
pub struct IgesSelectSelectSubordinate {
    status: i32,
}

impl IgesSelectSelectSubordinate {
    /// Creates a SelectSubordinate with a status to be sorted.
    ///
    /// # Arguments
    /// - `status`: The subordinate status code (0-6)
    pub fn new(status: i32) -> Self {
        IgesSelectSelectSubordinate { status }
    }

    /// Returns the status used for sorting.
    pub fn status(&self) -> i32 {
        self.status
    }

    /// Determines if an entity matches the subordinate status criterium.
    ///
    /// # Arguments
    /// - `_rank`: The rank/index of the entity
    /// - `entity_status`: The subordinate status of the entity to check
    ///
    /// Returns true if the entity's status matches the criterium
    pub fn sort(&self, _rank: i32, entity_status: i32) -> bool {
        match self.status {
            0 => entity_status == 0, // Independent
            1 => entity_status == 1, // Physically Dependent
            2 => entity_status == 2, // Logically Dependent
            3 => entity_status == 3, // Both
            4 => entity_status == 1 || entity_status == 3, // At least Physically
            5 => entity_status == 2 || entity_status == 3, // At least Logically
            6 => entity_status == 1 || entity_status == 2 || entity_status == 3, // Any dependence
            _ => false,
        }
    }

    /// Returns the selection criterium description.
    pub fn extract_label(&self) -> String {
        match self.status {
            0 => "IGES Entity, Independent".to_string(),
            1 => "IGES Entity, Physically Dependent".to_string(),
            2 => "IGES Entity, Logically Dependent".to_string(),
            3 => "IGES Entity, Both (Physical and Logical)".to_string(),
            4 => "IGES Entity, At least Physically Dependent".to_string(),
            5 => "IGES Entity, At least Logically Dependent".to_string(),
            6 => "IGES Entity, Any kind of Dependence".to_string(),
            _ => "IGES Entity".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_subordinate_creation() {
        let ss = IgesSelectSelectSubordinate::new(1);
        assert_eq!(ss.status(), 1);
    }

    #[test]
    fn test_select_subordinate_independent() {
        let ss = IgesSelectSelectSubordinate::new(0);
        assert!(ss.sort(0, 0));
        assert!(!ss.sort(0, 1));
        assert!(!ss.sort(0, 2));
        assert!(!ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_physically_dependent() {
        let ss = IgesSelectSelectSubordinate::new(1);
        assert!(!ss.sort(0, 0));
        assert!(ss.sort(0, 1));
        assert!(!ss.sort(0, 2));
        assert!(!ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_logically_dependent() {
        let ss = IgesSelectSelectSubordinate::new(2);
        assert!(!ss.sort(0, 0));
        assert!(!ss.sort(0, 1));
        assert!(ss.sort(0, 2));
        assert!(!ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_both() {
        let ss = IgesSelectSelectSubordinate::new(3);
        assert!(!ss.sort(0, 0));
        assert!(!ss.sort(0, 1));
        assert!(!ss.sort(0, 2));
        assert!(ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_at_least_physically() {
        let ss = IgesSelectSelectSubordinate::new(4);
        assert!(!ss.sort(0, 0));
        assert!(ss.sort(0, 1));
        assert!(!ss.sort(0, 2));
        assert!(ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_at_least_logically() {
        let ss = IgesSelectSelectSubordinate::new(5);
        assert!(!ss.sort(0, 0));
        assert!(!ss.sort(0, 1));
        assert!(ss.sort(0, 2));
        assert!(ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_any_dependence() {
        let ss = IgesSelectSelectSubordinate::new(6);
        assert!(!ss.sort(0, 0));
        assert!(ss.sort(0, 1));
        assert!(ss.sort(0, 2));
        assert!(ss.sort(0, 3));
    }

    #[test]
    fn test_select_subordinate_extract_label() {
        let ss0 = IgesSelectSelectSubordinate::new(0);
        assert_eq!(ss0.extract_label(), "IGES Entity, Independent".to_string());

        let ss1 = IgesSelectSelectSubordinate::new(1);
        assert_eq!(
            ss1.extract_label(),
            "IGES Entity, Physically Dependent".to_string()
        );

        let ss6 = IgesSelectSelectSubordinate::new(6);
        assert_eq!(
            ss6.extract_label(),
            "IGES Entity, Any kind of Dependence".to_string()
        );
    }
}
