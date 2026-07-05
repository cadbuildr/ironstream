// FILE: iges_select_sign_level_number.rs
// occt: IGESSelect_SignLevelNumber

/// Signature providing level number information for IGES entities.
/// Two display modes:
/// - Count mode: "LEVEL nnnnnnn", "NO LEVEL", "LEVEL LIST"
/// - Selection mode: "/nnn/", "/0/", "/1/2/nnn/" (for selection/matching)
pub struct IgesSelectSignLevelNumber {
    count_mode: bool,
}

impl IgesSelectSignLevelNumber {
    /// Creates a SignLevelNumber.
    ///
    /// # Arguments
    /// - `count_mode`: true = natural display, false = slash-separated for selection
    pub fn new(count_mode: bool) -> Self {
        IgesSelectSignLevelNumber { count_mode }
    }

    /// Returns whether count mode is enabled.
    pub fn is_count_mode(&self) -> bool {
        self.count_mode
    }

    /// Returns the level number value as a string.
    ///
    /// # Arguments
    /// - `_entity`: The IGES entity
    /// - `level_value`: The level number value (-1 = no level, -2 = level list, >= 0 = specific level)
    ///
    /// Returns formatted level number string
    pub fn value(&self, _entity: Option<&dyn std::any::Any>, level_value: i32) -> String {
        if self.count_mode {
            match level_value {
                -2 => "LEVEL LIST".to_string(),
                -1 => "NO LEVEL".to_string(),
                n if n >= 0 => format!("LEVEL {}", n),
                _ => "UNKNOWN".to_string(),
            }
        } else {
            // Selection mode with slashes
            match level_value {
                -2 => "/LEVELLIST/".to_string(),
                -1 => "/0/".to_string(),
                n if n >= 0 => format!("/{}/", n),
                _ => "/UNKNOWN/".to_string(),
            }
        }
    }

    /// Returns the signature name.
    pub fn name(&self) -> String {
        "Level Number".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_level_number_creation_count_mode() {
        let sln = IgesSelectSignLevelNumber::new(true);
        assert!(sln.is_count_mode());
    }

    #[test]
    fn test_sign_level_number_creation_selection_mode() {
        let sln = IgesSelectSignLevelNumber::new(false);
        assert!(!sln.is_count_mode());
    }

    #[test]
    fn test_sign_level_number_count_mode_no_level() {
        let sln = IgesSelectSignLevelNumber::new(true);
        assert_eq!(sln.value(None, -1), "NO LEVEL".to_string());
    }

    #[test]
    fn test_sign_level_number_count_mode_level_list() {
        let sln = IgesSelectSignLevelNumber::new(true);
        assert_eq!(sln.value(None, -2), "LEVEL LIST".to_string());
    }

    #[test]
    fn test_sign_level_number_count_mode_specific_level() {
        let sln = IgesSelectSignLevelNumber::new(true);
        assert_eq!(sln.value(None, 5), "LEVEL 5".to_string());
        assert_eq!(sln.value(None, 0), "LEVEL 0".to_string());
    }

    #[test]
    fn test_sign_level_number_selection_mode_no_level() {
        let sln = IgesSelectSignLevelNumber::new(false);
        assert_eq!(sln.value(None, -1), "/0/".to_string());
    }

    #[test]
    fn test_sign_level_number_selection_mode_level_list() {
        let sln = IgesSelectSignLevelNumber::new(false);
        assert_eq!(sln.value(None, -2), "/LEVELLIST/".to_string());
    }

    #[test]
    fn test_sign_level_number_selection_mode_specific_level() {
        let sln = IgesSelectSignLevelNumber::new(false);
        assert_eq!(sln.value(None, 3), "/3/".to_string());
        assert_eq!(sln.value(None, 10), "/10/".to_string());
    }

    #[test]
    fn test_sign_level_number_name() {
        let sln = IgesSelectSignLevelNumber::new(true);
        assert_eq!(sln.name(), "Level Number".to_string());
    }
}
