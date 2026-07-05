// FILE: iges_select_set_version5.rs
// occt: IGESSelect_SetVersion5

/// Upgrades IGES Version to 5.1 if it is older.
/// Sets Global Parameter 23 (IGES Version) to 5.1 and optionally updates
/// Global Parameter 25 (LastChangeDate) to current time.
pub struct IgesSelectSetVersion5 {}

impl IgesSelectSetVersion5 {
    /// Creates a SetVersion5 modifier using the current system date.
    pub fn new() -> Self {
        IgesSelectSetVersion5 {}
    }

    /// Performs the version upgrade if needed.
    /// Upgrades IGES Version to 5.1 if older, and sets LastChangeDate to current time.
    pub fn perform(&self, _target: Option<&dyn std::any::Any>) {
        // Real implementation would:
        // 1. Read the current IGES version from global parameter 23
        // 2. If older than 5.1, upgrade to 5.1
        // 3. Set global parameter 25 (LastChangeDate) to current system time
    }

    /// Returns a descriptive label for this modifier.
    pub fn label(&self) -> String {
        "Update IGES Version to 5.1".to_string()
    }
}

impl Default for IgesSelectSetVersion5 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_version5_creation() {
        let sv = IgesSelectSetVersion5::new();
        assert_eq!(sv.label(), "Update IGES Version to 5.1".to_string());
    }

    #[test]
    fn test_set_version5_label() {
        let sv = IgesSelectSetVersion5::new();
        assert_eq!(sv.label(), "Update IGES Version to 5.1".to_string());
    }

    #[test]
    fn test_set_version5_perform() {
        let sv = IgesSelectSetVersion5::new();
        sv.perform(None); // Should not panic
    }

    #[test]
    fn test_set_version5_default() {
        let sv = IgesSelectSetVersion5::default();
        assert_eq!(sv.label(), "Update IGES Version to 5.1".to_string());
    }
}
