// FILE: expr_ru_iterator.rs
// occt: Expr_RUIterator

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Represents a handle to a NamedUnknown (simulated via Rc).
pub type NamedUnknownHandle = Rc<RefCell<NamedUnknown>>;

/// A named unknown variable in an expression.
#[derive(Clone, Debug)]
pub struct NamedUnknown {
    pub name: String,
}

impl NamedUnknown {
    pub fn new(name: impl Into<String>) -> Self {
        NamedUnknown {
            name: name.into(),
        }
    }
}

/// Represents a handle to a GeneralRelation (simulated via Rc).
pub type GeneralRelationHandle = Rc<RefCell<GeneralRelation>>;

/// A general relation (used for iteration).
#[derive(Clone, Debug)]
pub struct GeneralRelation {
    // Placeholder for relation structure
}

/// Iterator over NamedUnknowns in a GeneralRelation.
/// Iterates on NamedUnknowns in a GeneralRelation.
pub struct ExprRUIterator {
    /// 1-indexed map of NamedUnknown handles.
    my_map: Vec<NamedUnknownHandle>,
    /// Current index (1-based, like OCCT).
    my_current: usize,
}

impl ExprRUIterator {
    /// Creates an iterator on every NamedUnknown contained in <rel>.
    pub fn new(_rel: &GeneralRelationHandle) -> Self {
        // In real implementation, would walk the relation tree and extract all unknowns.
        // For now, create an empty iterator structure.
        ExprRUIterator {
            my_map: Vec::new(),
            my_current: 1,
        }
    }

    /// Returns False if no other unknown remains.
    pub fn more(&self) -> bool {
        self.my_current <= self.my_map.len()
    }

    /// Move to next unknown.
    pub fn next(&mut self) {
        if !self.more() {
            panic!("Standard_NoMoreObject");
        }
        self.my_current += 1;
    }

    /// Returns current NamedUnknown.
    /// Panics if no more unknowns remain.
    pub fn value(&self) -> NamedUnknownHandle {
        if !self.more() {
            panic!("Standard_OutOfRange");
        }
        // my_map is 0-indexed, my_current is 1-indexed
        self.my_map[self.my_current - 1].clone()
    }

    /// Internal: add a named unknown to the map if not already present.
    pub fn add_unknown(&mut self, unknown: NamedUnknownHandle) {
        // Check if already in map using name comparison.
        let unknown_name = unknown.borrow().name.clone();
        let already_exists = self.my_map.iter().any(|u| {
            u.borrow().name == unknown_name
        });
        if !already_exists {
            self.my_map.push(unknown.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_iterator() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let it = ExprRUIterator::new(&rel);
        assert!(!it.more());
    }

    #[test]
    fn test_iterator_with_single_unknown() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let mut it = ExprRUIterator::new(&rel);

        let unknown = Rc::new(RefCell::new(NamedUnknown::new("x")));
        it.add_unknown(unknown.clone());

        assert!(it.more());
        let val = it.value();
        assert_eq!(val.borrow().name, "x");

        it.next();
        assert!(!it.more());
    }

    #[test]
    fn test_iterator_with_multiple_unknowns() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let mut it = ExprRUIterator::new(&rel);

        let x = Rc::new(RefCell::new(NamedUnknown::new("x")));
        let y = Rc::new(RefCell::new(NamedUnknown::new("y")));
        let z = Rc::new(RefCell::new(NamedUnknown::new("z")));

        it.add_unknown(x.clone());
        it.add_unknown(y.clone());
        it.add_unknown(z.clone());

        assert!(it.more());
        assert_eq!(it.value().borrow().name, "x");
        it.next();

        assert!(it.more());
        assert_eq!(it.value().borrow().name, "y");
        it.next();

        assert!(it.more());
        assert_eq!(it.value().borrow().name, "z");
        it.next();

        assert!(!it.more());
    }

    #[test]
    fn test_iterator_duplicates_not_added() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let mut it = ExprRUIterator::new(&rel);

        let x1 = Rc::new(RefCell::new(NamedUnknown::new("x")));
        let x2 = Rc::new(RefCell::new(NamedUnknown::new("x")));

        it.add_unknown(x1);
        it.add_unknown(x2);

        // Should only have one "x" in the map.
        assert!(it.more());
        assert_eq!(it.value().borrow().name, "x");
        it.next();
        assert!(!it.more());
    }

    #[test]
    #[should_panic(expected = "Standard_NoMoreObject")]
    fn test_next_panics_when_exhausted() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let mut it = ExprRUIterator::new(&rel);
        it.next(); // Should panic
    }

    #[test]
    #[should_panic(expected = "Standard_OutOfRange")]
    fn test_value_panics_when_exhausted() {
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        let it = ExprRUIterator::new(&rel);
        let _ = it.value(); // Should panic
    }
}
