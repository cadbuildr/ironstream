// FILE: if_select_select_shared.rs
// occt: IFSelect_SelectShared

/// Selects entities that are directly shared by input entities.
/// Works one level deep.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectShared;

impl IFSelectSelectShared {
    /// Creates a SelectShared
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium: "Shared (one level)"
    pub fn label(&self) -> &'static str {
        "Shared (one level)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = IFSelectSelectShared::new();
    }

    #[test]
    fn test_label() {
        let selector = IFSelectSelectShared::new();
        assert_eq!(selector.label(), "Shared (one level)");
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectShared::default();
        assert_eq!(selector.label(), "Shared (one level)");
    }
}
