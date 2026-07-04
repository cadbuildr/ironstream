// FILE: iges_basic_single_parent.rs
// occt: IGESBasic_SingleParent

/// SingleParent, Type <402> Form <9>
/// Defines a logical structure of one independent (parent) entity
/// and one or more subordinate (children) entities.
pub struct IgesBasicSingleParent {
    nb_parent_entities: i32,
    parent_entity: String,
    children: Vec<String>,
}

impl IgesBasicSingleParent {
    /// Create a new SingleParent with default values.
    pub fn new() -> Self {
        Self {
            nb_parent_entities: 1,
            parent_entity: String::new(),
            children: Vec::new(),
        }
    }

    /// Set the fields of the class SingleParent.
    /// - nb_parent_entities: Indicates number of Parents, always = 1
    /// - parent_entity: Used to hold the Parent Entity
    /// - all_children: Used to hold the children
    pub fn init(&mut self, nb_parent_entities: i32, parent_entity: String, all_children: Vec<String>) {
        self.nb_parent_entities = nb_parent_entities;
        self.parent_entity = parent_entity;
        self.children = all_children;
    }

    /// Returns the number of Parent Entities, which should be 1.
    pub fn nb_parent_entities(&self) -> i32 {
        self.nb_parent_entities
    }

    /// Returns the Parent Entity.
    pub fn single_parent(&self) -> &str {
        &self.parent_entity
    }

    /// Returns the number of children of the Parent.
    pub fn nb_children(&self) -> i32 {
        self.children.len() as i32
    }

    /// Returns the specific child as indicated by Index.
    /// Raises exception if Index <= 0 or Index > NbChildren().
    pub fn child(&self, index: i32) -> Option<&str> {
        if index <= 0 || index > self.nb_children() {
            return None;
        }
        Some(&self.children[(index - 1) as usize])
    }
}

impl Default for IgesBasicSingleParent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sp = IgesBasicSingleParent::new();
        assert_eq!(sp.nb_parent_entities(), 1);
        assert_eq!(sp.single_parent(), "");
        assert_eq!(sp.nb_children(), 0);
    }

    #[test]
    fn test_init() {
        let mut sp = IgesBasicSingleParent::new();
        let children = vec!["child1".to_string(), "child2".to_string()];
        sp.init(1, "parent".to_string(), children);
        assert_eq!(sp.nb_parent_entities(), 1);
        assert_eq!(sp.single_parent(), "parent");
        assert_eq!(sp.nb_children(), 2);
        assert_eq!(sp.child(1), Some("child1"));
        assert_eq!(sp.child(2), Some("child2"));
    }

    #[test]
    fn test_boundary_checks() {
        let mut sp = IgesBasicSingleParent::new();
        let children = vec!["child1".to_string()];
        sp.init(1, "parent".to_string(), children);
        assert_eq!(sp.child(0), None);
        assert_eq!(sp.child(2), None);
    }

    #[test]
    fn test_multiple_children() {
        let mut sp = IgesBasicSingleParent::new();
        let children = vec![
            "child1".to_string(),
            "child2".to_string(),
            "child3".to_string(),
        ];
        sp.init(1, "parent".to_string(), children);
        assert_eq!(sp.nb_children(), 3);
        assert_eq!(sp.child(1), Some("child1"));
        assert_eq!(sp.child(2), Some("child2"));
        assert_eq!(sp.child(3), Some("child3"));
    }

    #[test]
    fn test_default() {
        let sp = IgesBasicSingleParent::default();
        assert_eq!(sp.nb_parent_entities(), 1);
        assert_eq!(sp.single_parent(), "");
        assert_eq!(sp.nb_children(), 0);
    }
}
