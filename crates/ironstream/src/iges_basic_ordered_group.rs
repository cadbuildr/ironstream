// FILE: iges_basic_ordered_group.rs
// occt: IGESBasic_OrderedGroup

/// OrderedGroup, Type <402> Form <14>
/// Defines an Ordered Group with back pointers.
/// Allows a collection of a set of entities to be maintained as a single entity,
/// but the group is ordered.
pub struct IgesBasicOrderedGroup {
    entities: Vec<String>,
    is_ordered: bool,
    is_without_back_p: bool,
    group_type: i32,
    form: i32,
}

impl IgesBasicOrderedGroup {
    /// Create a new OrderedGroup with default values.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            is_ordered: true,
            is_without_back_p: false,
            group_type: 402,
            form: 14,
        }
    }

    /// Create an OrderedGroup with a predefined count of items.
    pub fn with_capacity(nb: i32) -> Self {
        let mut group = Self::new();
        group.entities = vec![String::new(); nb as usize];
        group
    }

    /// Set the fields of the class.
    pub fn init(&mut self, all_entities: Vec<String>) {
        self.entities = all_entities;
        self.is_ordered = true;
    }

    /// Returns true if the group is ordered (always true for OrderedGroup).
    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    /// Returns true if the group is without back pointers (always false for OrderedGroup).
    pub fn is_without_back_p(&self) -> bool {
        self.is_without_back_p
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

    /// Returns the form number (14 for ordered with back pointers).
    pub fn form(&self) -> i32 {
        self.form
    }
}

impl Default for IgesBasicOrderedGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let group = IgesBasicOrderedGroup::new();
        assert_eq!(group.nb_entities(), 0);
        assert!(group.is_ordered());
        assert!(!group.is_without_back_p());
        assert_eq!(group.form(), 14);
    }

    #[test]
    fn test_with_capacity() {
        let group = IgesBasicOrderedGroup::with_capacity(3);
        assert_eq!(group.nb_entities(), 3);
        assert_eq!(group.entity(1), Some(""));
    }

    #[test]
    fn test_set_value() {
        let mut group = IgesBasicOrderedGroup::with_capacity(2);
        group.set_value(1, "entity1".to_string());
        group.set_value(2, "entity2".to_string());
        assert_eq!(group.entity(1), Some("entity1"));
        assert_eq!(group.entity(2), Some("entity2"));
    }

    #[test]
    fn test_set_nb() {
        let mut group = IgesBasicOrderedGroup::with_capacity(3);
        assert_eq!(group.nb_entities(), 3);
        group.set_nb(5);
        assert_eq!(group.nb_entities(), 5);
    }

    #[test]
    fn test_boundary_checks() {
        let group = IgesBasicOrderedGroup::with_capacity(2);
        assert_eq!(group.entity(0), None);
        assert_eq!(group.entity(3), None);
    }

    #[test]
    fn test_is_ordered_is_true() {
        let group = IgesBasicOrderedGroup::new();
        assert!(group.is_ordered());
    }

    #[test]
    fn test_is_without_back_p_is_false() {
        let group = IgesBasicOrderedGroup::new();
        assert!(!group.is_without_back_p());
    }
}
