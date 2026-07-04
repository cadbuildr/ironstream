// FILE: if_select_select_roots.rs
// occt: IFSelect_SelectRoots

/// Selects local root entities from a set.
/// A root entity is not shared by others in the same set.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectRoots;

impl IFSelectSelectRoots {
    /// Creates a SelectRoots
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium: "Local Root Entities"
    pub fn extract_label(&self) -> &'static str {
        "Local Root Entities"
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
        let _selector = IFSelectSelectRoots::new();
    }

    #[test]
    fn test_extract_label() {
        let selector = IFSelectSelectRoots::new();
        assert_eq!(selector.extract_label(), "Local Root Entities");
    }

    #[test]
    fn test_sort() {
        let selector = IFSelectSelectRoots::new();
        assert!(selector.sort());
    }

    #[test]
    fn test_has_unique_result() {
        let selector = IFSelectSelectRoots::new();
        assert!(selector.has_unique_result());
    }
}
