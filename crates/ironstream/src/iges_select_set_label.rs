// FILE: iges_select_set_label.rs
// occt: IGESSelect_SetLabel

/// Sets or clears the short label of IGES entities.
/// Mode: 0 = clear (always enforced), 1 = set label to DE number (changes if already set).
pub struct IgesSelectSetLabel {
    mode: i32,
    enforce: bool,
}

impl IgesSelectSetLabel {
    /// Creates a SetLabel modifier.
    ///
    /// # Arguments
    /// - `mode`: 0 = clear label, 1 = set label to DE number
    /// - `enforce`: If true, always apply the change; if false, only set if not already set
    pub fn new(mode: i32, enforce: bool) -> Self {
        IgesSelectSetLabel { mode, enforce }
    }

    /// Returns the mode (0 = clear, 1 = set to DE number).
    pub fn mode(&self) -> i32 {
        self.mode
    }

    /// Returns whether the change is enforced.
    pub fn is_enforced(&self) -> bool {
        self.enforce
    }

    /// Applies the label modification to entities.
    pub fn perform(&self, _target: Option<&dyn std::any::Any>) {
        // Real implementation would:
        // 1. Iterate through selected entities
        // 2. Either clear the label or set it to the DE number
        // 3. If not enforced, skip entities that already have a label
    }

    /// Returns a descriptive label for this modifier.
    pub fn label(&self) -> String {
        let base = match self.mode {
            0 => "Clear Short Label".to_string(),
            _ => "Set Label to DE".to_string(),
        };

        if self.enforce {
            format!("{} (enforced)", base)
        } else {
            base
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_label_creation_clear() {
        let sl = IgesSelectSetLabel::new(0, false);
        assert_eq!(sl.mode(), 0);
        assert!(!sl.is_enforced());
    }

    #[test]
    fn test_set_label_creation_set() {
        let sl = IgesSelectSetLabel::new(1, true);
        assert_eq!(sl.mode(), 1);
        assert!(sl.is_enforced());
    }

    #[test]
    fn test_set_label_label_clear_not_enforced() {
        let sl = IgesSelectSetLabel::new(0, false);
        assert_eq!(sl.label(), "Clear Short Label".to_string());
    }

    #[test]
    fn test_set_label_label_clear_enforced() {
        let sl = IgesSelectSetLabel::new(0, true);
        assert_eq!(sl.label(), "Clear Short Label (enforced)".to_string());
    }

    #[test]
    fn test_set_label_label_set_not_enforced() {
        let sl = IgesSelectSetLabel::new(1, false);
        assert_eq!(sl.label(), "Set Label to DE".to_string());
    }

    #[test]
    fn test_set_label_label_set_enforced() {
        let sl = IgesSelectSetLabel::new(1, true);
        assert_eq!(sl.label(), "Set Label to DE (enforced)".to_string());
    }

    #[test]
    fn test_set_label_perform() {
        let sl = IgesSelectSetLabel::new(0, false);
        sl.perform(None); // Should not panic
    }
}
