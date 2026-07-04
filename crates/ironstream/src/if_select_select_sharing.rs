// FILE: if_select_select_sharing.rs
// occt: IFSelect_SelectSharing

/// Selects entities that directly share (one level) input entities.
/// If an input entity shares another, both appear in result.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectSharing;

impl IFSelectSelectSharing {
    /// Creates a SelectSharing
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium: "Sharing (one level)"
    pub fn label(&self) -> &'static str {
        "Sharing (one level)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = IFSelectSelectSharing::new();
    }

    #[test]
    fn test_label() {
        let selector = IFSelectSelectSharing::new();
        assert_eq!(selector.label(), "Sharing (one level)");
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectSharing::default();
        assert_eq!(selector.label(), "Sharing (one level)");
    }
}
