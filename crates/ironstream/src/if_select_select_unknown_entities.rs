// FILE: if_select_select_unknown_entities.rs
// occt: IFSelect_SelectUnknownEntities

/// Selects entities that are qualified as "Unknown".
/// An unknown entity is one whose type has not been recognized.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectUnknownEntities;

impl IFSelectSelectUnknownEntities {
    /// Creates a SelectUnknownEntities
    pub fn new() -> Self {
        Self
    }

    /// Returns a text defining the criterium
    pub fn extract_label(&self) -> &'static str {
        "Unknown Entities"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = IFSelectSelectUnknownEntities::new();
    }

    #[test]
    fn test_extract_label() {
        let selector = IFSelectSelectUnknownEntities::new();
        assert_eq!(selector.extract_label(), "Unknown Entities");
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectUnknownEntities::default();
        assert_eq!(selector.extract_label(), "Unknown Entities");
    }
}
