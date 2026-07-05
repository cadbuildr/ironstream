// FILE: expr_system_relation.rs
// occt: Expr_SystemRelation

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a handle to a GeneralRelation (simulated via Rc).
pub type GeneralRelationHandle = Rc<RefCell<GeneralRelation>>;

/// Represents a handle to a NamedUnknown (simulated via Rc).
pub type NamedUnknownHandle = Rc<RefCell<NamedUnknown>>;

/// A general relation.
#[derive(Clone, Debug)]
pub struct GeneralRelation {
    // Placeholder for relation content
}

impl GeneralRelation {
    /// Returns the number of sub-relations.
    pub fn nb_of_sub_relations(&self) -> usize {
        0
    }

    /// Returns the number of single relations.
    pub fn nb_of_single_relations(&self) -> usize {
        0
    }

    /// Check if linear.
    pub fn is_linear(&self) -> bool {
        false
    }
}

/// A named unknown variable.
#[derive(Clone, Debug)]
pub struct NamedUnknown {
    pub name: String,
}

/// A system of relations (multiple relations combined).
/// occt: Expr_SystemRelation
#[derive(Clone, Debug)]
pub struct SystemRelation {
    /// List of relations in this system.
    relations: Vec<GeneralRelationHandle>,
}

impl SystemRelation {
    /// Creates a system with one relation.
    pub fn new(relation: GeneralRelationHandle) -> Self {
        SystemRelation {
            relations: vec![relation],
        }
    }

    /// Appends <relation> in the list of components.
    pub fn add(&mut self, relation: GeneralRelationHandle) {
        self.relations.push(relation);
    }

    /// Removes <relation> from the list of components.
    pub fn remove(&mut self, relation: &GeneralRelationHandle) {
        self.relations.retain(|r| !std::ptr::eq(r.as_ptr(), relation.as_ptr()));
    }

    /// Tests if all relations are linear.
    pub fn is_linear(&self) -> bool {
        self.relations.iter().all(|r| r.borrow().is_linear())
    }

    /// Returns the number of sub-relations.
    pub fn nb_of_sub_relations(&self) -> usize {
        self.relations.len()
    }

    /// Returns the number of single relations in all sub-relations.
    pub fn nb_of_single_relations(&self) -> usize {
        self.relations
            .iter()
            .map(|r| r.borrow().nb_of_single_relations())
            .sum()
    }

    /// Get the i-th relation (1-indexed).
    pub fn sub_relation(&self, i: usize) -> Option<GeneralRelationHandle> {
        if i > 0 && i <= self.relations.len() {
            Some(self.relations[i - 1].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_relation_creation() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let sys = SystemRelation::new(rel.clone());
        assert_eq!(sys.nb_of_sub_relations(), 1);
    }

    #[test]
    fn test_system_relation_add() {
        let rel1 = Rc::new(RefCell::new(GeneralRelation {}));
        let rel2 = Rc::new(RefCell::new(GeneralRelation {}));

        let mut sys = SystemRelation::new(rel1);
        assert_eq!(sys.nb_of_sub_relations(), 1);

        sys.add(rel2);
        assert_eq!(sys.nb_of_sub_relations(), 2);
    }

    #[test]
    fn test_system_relation_remove() {
        let rel1 = Rc::new(RefCell::new(GeneralRelation {}));
        let rel2 = Rc::new(RefCell::new(GeneralRelation {}));

        let mut sys = SystemRelation::new(rel1);
        sys.add(rel2.clone());
        assert_eq!(sys.nb_of_sub_relations(), 2);

        sys.remove(&rel2);
        assert_eq!(sys.nb_of_sub_relations(), 1);
    }

    #[test]
    fn test_system_relation_is_linear() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let sys = SystemRelation::new(rel);
        // Empty relations default to false in placeholder implementation
        assert!(!sys.is_linear());
    }

    #[test]
    fn test_system_relation_nb_of_single_relations() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let sys = SystemRelation::new(rel);
        let nb_single = sys.nb_of_single_relations();
        assert_eq!(nb_single, 0); // Placeholder returns 0
    }

    #[test]
    fn test_system_relation_sub_relation() {
        let rel1 = Rc::new(RefCell::new(GeneralRelation {}));
        let rel2 = Rc::new(RefCell::new(GeneralRelation {}));

        let mut sys = SystemRelation::new(rel1.clone());
        sys.add(rel2.clone());

        assert!(sys.sub_relation(1).is_some());
        assert!(sys.sub_relation(2).is_some());
        assert!(sys.sub_relation(3).is_none());
    }
}
