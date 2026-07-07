// FILE: if_select_select_model_entities.rs
// occt: IFSelect_SelectModelEntities

/// Selects all entities from an InterfaceModel.
/// This guarantees uniqueness in its result.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectModelEntities;

impl IFSelectSelectModelEntities {
    /// Creates a SelectModelEntities
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium: "Model Entities"
    pub fn label(&self) -> &'static str {
        "Model Entities"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = IFSelectSelectModelEntities::new();
    }

    #[test]
    fn test_label() {
        let selector = IFSelectSelectModelEntities::new();
        assert_eq!(selector.label(), "Model Entities");
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectModelEntities::default();
        assert_eq!(selector.label(), "Model Entities");
    }
}
