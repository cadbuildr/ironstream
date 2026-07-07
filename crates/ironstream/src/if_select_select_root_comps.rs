// FILE: if_select_select_root_comps.rs
// occt: IFSelect_SelectRootComps

/// Selects local root strong components.
/// Handles both single components and cycles.
/// More secure than SelectRoots but slower.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectRootComps;

impl IFSelectSelectRootComps {
    /// Creates a SelectRootComps
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium: "Local Root Components"
    pub fn extract_label(&self) -> &'static str {
        "Local Root Components"
    }

    /// Sort always returns true since RootResult has done the work
    pub fn sort(&self) -> bool {
        true
    }

    /// RootResult assures uniqueness
    pub fn has_unique_result(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = IFSelectSelectRootComps::new();
    }

    #[test]
    fn test_extract_label() {
        let selector = IFSelectSelectRootComps::new();
        assert_eq!(selector.extract_label(), "Local Root Components");
    }

    #[test]
    fn test_sort() {
        let selector = IFSelectSelectRootComps::new();
        assert!(selector.sort());
    }

    #[test]
    fn test_has_unique_result() {
        let selector = IFSelectSelectRootComps::new();
        assert!(selector.has_unique_result());
    }
}
