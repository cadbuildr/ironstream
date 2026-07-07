// FILE: iges_select_select_level_number.rs
// occt: IGESSelect_SelectLevelNumber

/// Represents an IGES level number selector.
/// Selects IGES entities based on their level number attachment.
/// Level = 0 means entities not attached to any level.
pub struct IgesSelectSelectLevelNumber {
    level_number: Option<i32>,
}

impl IgesSelectSelectLevelNumber {
    /// Creates a new SelectLevelNumber with no level criterium.
    pub fn new() -> Self {
        IgesSelectSelectLevelNumber { level_number: None }
    }

    /// Sets the level number criterium.
    ///
    /// # Arguments
    /// - `level`: The level number to filter by
    pub fn set_level_number(&mut self, level: i32) {
        self.level_number = Some(level);
    }

    /// Returns the level number criterium, or None if not set.
    pub fn level_number(&self) -> Option<i32> {
        self.level_number
    }

    /// Determines if an entity matches the level number criterium.
    ///
    /// # Arguments
    /// - `rank`: The rank/index of the entity
    /// - `level_value`: The level value of the entity to check
    ///
    /// Returns true if the entity's level matches the criterium
    pub fn sort(&self, _rank: i32, level_value: i32) -> bool {
        match self.level_number {
            None => level_value == 0,
            Some(target) => level_value == target,
        }
    }

    /// Returns the selection criterium description.
    pub fn extract_label(&self) -> String {
        match self.level_number {
            None => "IGES Entity attached to no Level".to_string(),
            Some(nn) if nn == 0 => "IGES Entity attached to no Level".to_string(),
            Some(nn) => format!("IGES Entity, Level Number admits {}", nn),
        }
    }
}

impl Default for IgesSelectSelectLevelNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_level_number_creation() {
        let sln = IgesSelectSelectLevelNumber::new();
        assert_eq!(sln.level_number(), None);
    }

    #[test]
    fn test_select_level_number_set() {
        let mut sln = IgesSelectSelectLevelNumber::new();
        sln.set_level_number(5);
        assert_eq!(sln.level_number(), Some(5));
    }

    #[test]
    fn test_select_level_number_sort_no_criterium() {
        let sln = IgesSelectSelectLevelNumber::new();
        assert!(sln.sort(0, 0)); // Should match entities with level 0
        assert!(!sln.sort(0, 1));
    }

    #[test]
    fn test_select_level_number_sort_with_criterium() {
        let mut sln = IgesSelectSelectLevelNumber::new();
        sln.set_level_number(3);
        assert!(!sln.sort(0, 0));
        assert!(sln.sort(0, 3));
        assert!(!sln.sort(0, 5));
    }

    #[test]
    fn test_select_level_number_extract_label_no_criterium() {
        let sln = IgesSelectSelectLevelNumber::new();
        assert_eq!(
            sln.extract_label(),
            "IGES Entity attached to no Level".to_string()
        );
    }

    #[test]
    fn test_select_level_number_extract_label_level_zero() {
        let mut sln = IgesSelectSelectLevelNumber::new();
        sln.set_level_number(0);
        assert_eq!(
            sln.extract_label(),
            "IGES Entity attached to no Level".to_string()
        );
    }

    #[test]
    fn test_select_level_number_extract_label_level_nonzero() {
        let mut sln = IgesSelectSelectLevelNumber::new();
        sln.set_level_number(5);
        assert_eq!(
            sln.extract_label(),
            "IGES Entity, Level Number admits 5".to_string()
        );
    }
}
