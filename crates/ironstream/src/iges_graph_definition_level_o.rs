// FILE: iges_graph_definition_level_o.rs
// occt: IGESGraph_DefinitionLevel

/// Represents an IGES Definition Level entity (Type 406, Form 1).
/// Indicates the number of levels on which an entity is defined.
pub struct IgesGraphDefinitionLevel {
    level_numbers: Vec<i32>,
}

impl IgesGraphDefinitionLevel {
    /// Creates a new empty DefinitionLevel entity.
    pub fn new() -> Self {
        IgesGraphDefinitionLevel {
            level_numbers: Vec::new(),
        }
    }

    /// Sets the level numbers.
    ///
    /// # Arguments
    /// - `all_level_numbers`: Vector of level numbers
    pub fn init(&mut self, all_level_numbers: Vec<i32>) {
        self.level_numbers = all_level_numbers;
    }

    /// Returns the number of property values (equals NbLevelNumbers).
    pub fn nb_property_values(&self) -> i32 {
        self.level_numbers.len() as i32
    }

    /// Returns the count of levels (same as NbPropertyValues).
    pub fn nb_level_numbers(&self) -> i32 {
        self.level_numbers.len() as i32
    }

    /// Returns the level number at the given index (1-based indexing).
    ///
    /// # Arguments
    /// - `level_index`: 1-based index into the level numbers
    ///
    /// # Returns
    /// The level number at the given index, or None if index is out of bounds
    pub fn level_number(&self, level_index: i32) -> Option<i32> {
        if level_index > 0 && (level_index as usize) <= self.level_numbers.len() {
            Some(self.level_numbers[(level_index - 1) as usize])
        } else {
            None
        }
    }

    /// Returns all level numbers.
    pub fn level_numbers(&self) -> &[i32] {
        &self.level_numbers
    }
}

impl Default for IgesGraphDefinitionLevel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition_level_creation() {
        let dl = IgesGraphDefinitionLevel::new();
        assert_eq!(dl.nb_property_values(), 0);
        assert_eq!(dl.nb_level_numbers(), 0);
    }

    #[test]
    fn test_definition_level_init() {
        let mut dl = IgesGraphDefinitionLevel::new();
        dl.init(vec![1, 2, 3, 4]);
        assert_eq!(dl.nb_property_values(), 4);
        assert_eq!(dl.nb_level_numbers(), 4);
        assert_eq!(dl.level_numbers(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_definition_level_index_access() {
        let mut dl = IgesGraphDefinitionLevel::new();
        dl.init(vec![10, 20, 30]);
        assert_eq!(dl.level_number(1), Some(10));
        assert_eq!(dl.level_number(2), Some(20));
        assert_eq!(dl.level_number(3), Some(30));
        assert_eq!(dl.level_number(0), None); // Out of bounds (0 not valid)
        assert_eq!(dl.level_number(4), None); // Out of bounds
    }

    #[test]
    fn test_definition_level_empty_levels() {
        let dl = IgesGraphDefinitionLevel::new();
        assert_eq!(dl.level_number(1), None);
    }
}
