// FILE: expr_unknown_iterator.rs
// occt: Expr_UnknownIterator

use std::rc::Rc;
use std::cell::RefCell;

pub type GeneralExpressionHandle = Rc<RefCell<GeneralExpression>>;
pub type NamedUnknownHandle = Rc<RefCell<NamedUnknown>>;

#[derive(Clone, Debug)]
pub struct GeneralExpression {}

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

/// Iterator over all NamedUnknowns contained in a GeneralExpression.
#[derive(Clone, Debug)]
pub struct UnknownIterator {
    /// List of unknowns (deduplicated).
    unknowns: Vec<NamedUnknownHandle>,
    /// Current index (1-based, like OCCT).
    current: usize,
}

impl UnknownIterator {
    /// Create an iterator over unknowns in an expression.
    pub fn new(_expr: &GeneralExpressionHandle) -> Self {
        // In a real implementation, would walk the expression tree and extract all unknowns.
        // For now, return an empty iterator.
        UnknownIterator {
            unknowns: Vec::new(),
            current: 1,
        }
    }

    /// Returns False if no other unknown remains.
    pub fn more(&self) -> bool {
        self.current <= self.unknowns.len()
    }

    /// Move to next unknown.
    pub fn next(&mut self) {
        if !self.more() {
            panic!("Standard_NoMoreObject");
        }
        self.current += 1;
    }

    /// Returns current NamedUnknown.
    pub fn value(&self) -> NamedUnknownHandle {
        if !self.more() {
            panic!("Standard_OutOfRange");
        }
        // unknowns is 0-indexed, current is 1-indexed
        self.unknowns[self.current - 1].clone()
    }

    /// Internal: add an unknown to the iterator.
    pub fn add_unknown(&mut self, unknown: NamedUnknownHandle) {
        let unknown_name = unknown.borrow().name.clone();
        let already_exists = self.unknowns.iter().any(|u| {
            u.borrow().name == unknown_name
        });
        if !already_exists {
            self.unknowns.push(unknown);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_iterator() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let it = UnknownIterator::new(&exp);
        assert!(!it.more());
    }

    #[test]
    fn test_iterator_with_single_unknown() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let mut it = UnknownIterator::new(&exp);

        let unknown = Rc::new(RefCell::new(NamedUnknown::new("x")));
        it.add_unknown(unknown.clone());

        assert!(it.more());
        assert_eq!(it.value().borrow().name, "x");

        it.next();
        assert!(!it.more());
    }

    #[test]
    fn test_iterator_with_multiple_unknowns() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let mut it = UnknownIterator::new(&exp);

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
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let mut it = UnknownIterator::new(&exp);

        let x1 = Rc::new(RefCell::new(NamedUnknown::new("x")));
        let x2 = Rc::new(RefCell::new(NamedUnknown::new("x")));

        it.add_unknown(x1);
        it.add_unknown(x2);

        // Should only have one "x" in the iterator.
        assert!(it.more());
        assert_eq!(it.value().borrow().name, "x");
        it.next();
        assert!(!it.more());
    }

    #[test]
    #[should_panic(expected = "Standard_NoMoreObject")]
    fn test_next_panics_when_exhausted() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let mut it = UnknownIterator::new(&exp);
        it.next();
    }

    #[test]
    #[should_panic(expected = "Standard_OutOfRange")]
    fn test_value_panics_when_exhausted() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let it = UnknownIterator::new(&exp);
        let _ = it.value();
    }
}
