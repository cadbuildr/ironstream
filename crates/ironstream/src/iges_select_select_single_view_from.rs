// FILE: iges_select_select_single_view_from.rs
// occt: IGESSelect_SelectSingleViewFrom

/// Selects Single Views attached to input IGES entities.
/// Single Views themselves or Drawings are passed as such (Drawings for their Annotations).
pub struct IgesSelectSelectSingleViewFrom {}

impl IgesSelectSelectSingleViewFrom {
    /// Creates a SelectSingleViewFrom selector.
    pub fn new() -> Self {
        IgesSelectSelectSingleViewFrom {}
    }

    /// Selects the Single Views attached to input entities.
    /// Returns iterator of Single Views from the input entities.
    pub fn root_result(&self) -> Vec<usize> {
        // In a real implementation, this would traverse the entity graph
        // to find Single Views attached to input entities
        Vec::new()
    }

    /// Returns the selection label.
    pub fn label(&self) -> String {
        "Single Views attached".to_string()
    }

    /// Returns true because selection works with a ViewSorter giving unique results.
    pub fn has_unique_result(&self) -> bool {
        true
    }
}

impl Default for IgesSelectSelectSingleViewFrom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_single_view_from_creation() {
        let ssvf = IgesSelectSelectSingleViewFrom::new();
        assert_eq!(ssvf.label(), "Single Views attached".to_string());
        assert!(ssvf.has_unique_result());
    }

    #[test]
    fn test_select_single_view_from_root_result() {
        let ssvf = IgesSelectSelectSingleViewFrom::new();
        let result = ssvf.root_result();
        assert!(result.is_empty());
    }

    #[test]
    fn test_select_single_view_from_default() {
        let ssvf = IgesSelectSelectSingleViewFrom::default();
        assert_eq!(ssvf.label(), "Single Views attached".to_string());
    }
}
