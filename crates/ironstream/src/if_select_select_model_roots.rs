// FILE: if_select_select_model_roots.rs
// occt: IFSelect_SelectModelRoots

/// Selects all root entities from an InterfaceModel.
/// A "Root Entity" is one with no sharing entity (unless there are loops).
/// Guarantees uniqueness in result.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectModelRoots;

impl IFSelectSelectModelRoots {
    /// Creates a SelectModelRoots
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium: "Model Roots"
    pub fn label(&self) -> &'static str {
        "Model Roots"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = IFSelectSelectModelRoots::new();
    }

    #[test]
    fn test_label() {
        let selector = IFSelectSelectModelRoots::new();
        assert_eq!(selector.label(), "Model Roots");
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectModelRoots::default();
        assert_eq!(selector.label(), "Model Roots");
    }
}
