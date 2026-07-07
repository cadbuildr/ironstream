// FILE: iges_select_select_visible_status.rs
// occt: IGESSelect_SelectVisibleStatus

/// Selects IGES entities based on their visible status (Blank status).
/// Direct selection keeps Visible Entities (Blank = 0).
/// Reverse selection keeps Blanked Entities (Blank = 1).
pub struct IgesSelectSelectVisibleStatus {}

impl IgesSelectSelectVisibleStatus {
    /// Creates a SelectVisibleStatus selector.
    pub fn new() -> Self {
        IgesSelectSelectVisibleStatus {}
    }

    /// Determines if an entity is visible (Blank Status = 0).
    ///
    /// # Arguments
    /// - `_rank`: The rank/index of the entity
    /// - `blank_status`: The blank status of the entity (0 = visible, 1 = blanked)
    ///
    /// Returns true if the entity is visible
    pub fn sort(&self, _rank: i32, blank_status: i32) -> bool {
        blank_status == 0
    }

    /// Returns the selection criterium description.
    pub fn extract_label(&self) -> String {
        "IGES Entity, Status Visible".to_string()
    }
}

impl Default for IgesSelectSelectVisibleStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_visible_status_creation() {
        let svs = IgesSelectSelectVisibleStatus::new();
        assert_eq!(svs.extract_label(), "IGES Entity, Status Visible".to_string());
    }

    #[test]
    fn test_select_visible_status_visible() {
        let svs = IgesSelectSelectVisibleStatus::new();
        assert!(svs.sort(0, 0)); // Blank = 0 (visible)
    }

    #[test]
    fn test_select_visible_status_blanked() {
        let svs = IgesSelectSelectVisibleStatus::new();
        assert!(!svs.sort(0, 1)); // Blank = 1 (blanked)
    }

    #[test]
    fn test_select_visible_status_default() {
        let svs = IgesSelectSelectVisibleStatus::default();
        assert_eq!(svs.extract_label(), "IGES Entity, Status Visible".to_string());
    }
}
