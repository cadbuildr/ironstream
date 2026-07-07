// FILE: iges_basic_group.rs
// occt: IGESBasic_Group

/// Group, Type <402> Form <1>
/// The Group Associativity allows a collection of a set
/// of entities to be maintained as a single, logical entity.
pub struct IgesBasicGroup {
    entities: Vec<String>,
    is_ordered: bool,
    is_without_back_p: bool,
    group_type: i32,
    form: i32,
}

impl IgesBasicGroup {
    /// Create a new Group with default values.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            is_ordered: false,
            is_without_back_p: false,
            group_type: 402,
            form: 1,
        }
    }

    /// Create a Group with a predefined count of items (which all start as empty).
    pub fn with_capacity(nb: i32) -> Self {
        let mut group = Self::new();
        group.entities = vec![String::new(); nb as usize];
        group
    }

    /// Set the fields of the class Group.
    /// - all_entities: Used to store pointers to members of the Group.
    pub fn init(&mut self, all_entities: Vec<String>) {
        self.entities = all_entities;
        self.update_form();
    }

    /// Set a Group to be, or not to be Ordered (according to mode).
    pub fn set_ordered(&mut self, mode: bool) {
        self.is_ordered = mode;
        self.update_form();
    }

    /// Set a Group to be, or not to be WithoutBackP.
    pub fn set_without_back_p(&mut self, mode: bool) {
        self.is_without_back_p = mode;
        self.update_form();
    }

    /// Returns true if the group is Ordered.
    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    /// Returns true if the group is WithoutBackP.
    pub fn is_without_back_p(&self) -> bool {
        self.is_without_back_p
    }

    /// Enforce a new value for the type and form.
    pub fn set_user(&mut self, group_type: i32, form: i32) {
        self.group_type = group_type;
        self.form = form;
    }

    /// Change the count of items.
    /// If greater, new items are empty strings.
    /// If lower, old items are lost.
    pub fn set_nb(&mut self, nb: i32) {
        self.entities.resize(nb as usize, String::new());
    }

    /// Returns the number of entities in the Group.
    pub fn nb_entities(&self) -> i32 {
        self.entities.len() as i32
    }

    /// Returns the specific entity from the Group.
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

    /// Internal: Update form number based on flags.
    fn update_form(&mut self) {
        self.form = match (self.is_ordered, self.is_without_back_p) {
            (false, false) => 1,
            (false, true) => 7,
            (true, false) => 14,
            (true, true) => 15,
        };
    }
}

impl Default for IgesBasicGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let group = IgesBasicGroup::new();
        assert_eq!(group.nb_entities(), 0);
        assert!(!group.is_ordered());
        assert!(!group.is_without_back_p());
        assert_eq!(group.form, 1);
    }

    #[test]
    fn test_with_capacity() {
        let group = IgesBasicGroup::with_capacity(3);
        assert_eq!(group.nb_entities(), 3);
        assert_eq!(group.entity(1), Some(""));
    }

    #[test]
    fn test_set_ordered() {
        let mut group = IgesBasicGroup::new();
        group.set_nb(1);
        group.set_ordered(true);
        assert!(group.is_ordered());
        assert_eq!(group.form, 14);
        group.set_ordered(false);
        assert!(!group.is_ordered());
        assert_eq!(group.form, 1);
    }

    #[test]
    fn test_set_without_back_p() {
        let mut group = IgesBasicGroup::new();
        group.set_nb(1);
        group.set_without_back_p(true);
        assert!(group.is_without_back_p());
        assert_eq!(group.form, 7);
    }

    #[test]
    fn test_form_combinations() {
        let mut group = IgesBasicGroup::new();
        group.set_nb(1);

        group.set_ordered(false);
        group.set_without_back_p(false);
        assert_eq!(group.form, 1);

        group.set_ordered(false);
        group.set_without_back_p(true);
        assert_eq!(group.form, 7);

        group.set_ordered(true);
        group.set_without_back_p(false);
        assert_eq!(group.form, 14);

        group.set_ordered(true);
        group.set_without_back_p(true);
        assert_eq!(group.form, 15);
    }

    #[test]
    fn test_set_value() {
        let mut group = IgesBasicGroup::with_capacity(2);
        group.set_value(1, "entity1".to_string());
        group.set_value(2, "entity2".to_string());
        assert_eq!(group.entity(1), Some("entity1"));
        assert_eq!(group.entity(2), Some("entity2"));
    }

    #[test]
    fn test_set_nb() {
        let mut group = IgesBasicGroup::with_capacity(3);
        assert_eq!(group.nb_entities(), 3);
        group.set_nb(5);
        assert_eq!(group.nb_entities(), 5);
        group.set_nb(2);
        assert_eq!(group.nb_entities(), 2);
    }

    #[test]
    fn test_boundary_checks() {
        let group = IgesBasicGroup::with_capacity(2);
        assert_eq!(group.entity(0), None);
        assert_eq!(group.entity(3), None);
    }
}
