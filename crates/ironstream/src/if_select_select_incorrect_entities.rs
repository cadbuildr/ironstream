// FILE: if_select_select_incorrect_entities.rs
// occt: IFSelect_SelectIncorrectEntities

/// Selects entities marked as incorrect in the graph.
/// This is a SelectFlag that queries the "Incorrect" flag set by ComputeCheck.
#[derive(Clone, Debug)]
pub struct IFSelectSelectIncorrectEntities {
    // Inherits from IFSelect_SelectFlag which stores the flag name
    flag_name: String,
}

impl IFSelectSelectIncorrectEntities {
    /// Creates a SelectIncorrectEntities, i.e., a SelectFlag("Incorrect")
    pub fn new() -> Self {
        Self {
            flag_name: "Incorrect".to_string(),
        }
    }

    /// Returns the name of the flag
    pub fn flag_name(&self) -> &str {
        &self.flag_name
    }
}

impl Default for IFSelectSelectIncorrectEntities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let selector = IFSelectSelectIncorrectEntities::new();
        assert_eq!(selector.flag_name(), "Incorrect");
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectIncorrectEntities::default();
        assert_eq!(selector.flag_name(), "Incorrect");
    }
}
