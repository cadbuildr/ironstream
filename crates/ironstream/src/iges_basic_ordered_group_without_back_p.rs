// FILE: iges_basic_ordered_group_without_back_p.rs
// occt: IGESBasic_OrderedGroupWithoutBackP

/// OrderedGroupWithoutBackP, Type <402> Form <15>
/// Allows a collection of a set of entities to be maintained as a single entity,
/// but the group is ordered and there are no back pointers.
pub struct IgesBasicOrderedGroupWithoutBackP {
    entities: Vec<String>,
}

impl IgesBasicOrderedGroupWithoutBackP {
    /// Create a new OrderedGroupWithoutBackP with default values.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Create an OrderedGroupWithoutBackP with a predefined count of items.
    pub fn with_capacity(nb: i32) -> Self {
        let mut group = Self::new();
        group.entities = vec![String::new(); nb as usize];
        group
    }

    /// Set the fields of the class.
    pub fn init(&mut self, all_entities: Vec<String>) {
        self.entities = all_entities;
    }

    /// Returns true if the group is ordered (always true).
    pub fn is_ordered(&self) -> bool {
        true
    }

    /// Returns true if the group is without back pointers (always true).
    pub fn is_without_back_p(&self) -> bool {
        true
    }

    /// Change the count of items.
    pub fn set_nb(&mut self, nb: i32) {
        self.entities.resize(nb as usize, String::new());
    }

    /// Returns the number of entities in the group.
    pub fn nb_entities(&self) -> i32 {
        self.entities.len() as i32
    }

    /// Returns the specific entity from the group.
    pub fn entity(&self, index: i32) -> Option<&str> {
        if index <= 0 || index > self.nb_entities() {
            return None;
        }
        Some(&self.entities[(index - 1) as usize])
    }

    /// Set a new value for item at Index.
    pub fn set_value(&mut self, index: i32, ent: String) {
        if index > 0 && index <= self.nb_entities() {
            self.entities[(index - 1) as usize] = ent;
        }
    }

    /// Returns the form number (15 for ordered without back pointers).
    pub fn form(&self) -> i32 {
        15
    }
}

impl Default for IgesBasicOrderedGroupWithoutBackP {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let group = IgesBasicOrderedGroupWithoutBackP::new();
        assert_eq!(group.nb_entities(), 0);
        assert!(group.is_ordered());
        assert!(group.is_without_back_p());
        assert_eq!(group.form(), 15);
    }

    #[test]
    fn test_with_capacity() {
        let group = IgesBasicOrderedGroupWithoutBackP::with_capacity(3);
        assert_eq!(group.nb_entities(), 3);
        assert_eq!(group.entity(1), Some(""));
    }

    #[test]
    fn test_set_value() {
        let mut group = IgesBasicOrderedGroupWithoutBackP::with_capacity(2);
        group.set_value(1, "entity1".to_string());
        group.set_value(2, "entity2".to_string());
        assert_eq!(group.entity(1), Some("entity1"));
        assert_eq!(group.entity(2), Some("entity2"));
    }

    #[test]
    fn test_set_nb() {
        let mut group = IgesBasicOrderedGroupWithoutBackP::with_capacity(3);
        assert_eq!(group.nb_entities(), 3);
        group.set_nb(5);
        assert_eq!(group.nb_entities(), 5);
    }

    #[test]
    fn test_boundary_checks() {
        let group = IgesBasicOrderedGroupWithoutBackP::with_capacity(2);
        assert_eq!(group.entity(0), None);
        assert_eq!(group.entity(3), None);
    }

    #[test]
    fn test_is_ordered_is_true() {
        let group = IgesBasicOrderedGroupWithoutBackP::new();
        assert!(group.is_ordered());
    }

    #[test]
    fn test_is_without_back_p_is_true() {
        let group = IgesBasicOrderedGroupWithoutBackP::new();
        assert!(group.is_without_back_p());
    }
}
