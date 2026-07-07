// FILE: expr_single_relation.rs
// occt: Expr_SingleRelation

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a handle to a GeneralExpression (simulated via Rc).
pub type GeneralExpressionHandle = Rc<RefCell<GeneralExpression>>;

/// Represents a handle to a GeneralRelation (simulated via Rc).
pub type GeneralRelationHandle = Rc<RefCell<GeneralRelation>>;

/// Represents a handle to a NamedUnknown (simulated via Rc).
pub type NamedUnknownHandle = Rc<RefCell<NamedUnknown>>;

/// A general expression.
#[derive(Clone, Debug)]
pub struct GeneralExpression {
    // Placeholder for expression content
}

impl GeneralExpression {
    /// Check if this expression is linear.
    /// This placeholder expression holds no unknowns, and in OCCT an
    /// expression without unknowns is linear (e.g. Expr_NumericValue::IsLinear
    /// returns true).
    pub fn is_linear(&self) -> bool {
        true
    }

    /// Check if this expression contains the given expression.
    pub fn contains(&self, _exp: &GeneralExpressionHandle) -> bool {
        false
    }

    /// Replace all occurrences of <var> with <with>.
    pub fn replace(&mut self, _var: &NamedUnknownHandle, _with: &GeneralExpressionHandle) {
        // Placeholder
    }
}

/// A named unknown variable.
#[derive(Clone, Debug)]
pub struct NamedUnknown {
    pub name: String,
}

/// A general relation (base class).
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

    /// Returns the sub-relation at the given index (1-indexed).
    pub fn sub_relation(&self, _index: usize) -> Option<GeneralRelationHandle> {
        None
    }
}

/// A relation between two expressions (e.g., "a = b", "a < b", etc.).
/// occt: Expr_SingleRelation
#[derive(Clone, Debug)]
pub struct SingleRelation {
    /// First member (LHS)
    first_member: Option<GeneralExpressionHandle>,
    /// Second member (RHS)
    second_member: Option<GeneralExpressionHandle>,
}

impl SingleRelation {
    /// Create a new empty single relation.
    pub fn new() -> Self {
        SingleRelation {
            first_member: None,
            second_member: None,
        }
    }

    /// Set the first member (LHS).
    pub fn set_first_member(&mut self, exp: GeneralExpressionHandle) {
        self.first_member = Some(exp);
    }

    /// Set the second member (RHS).
    pub fn set_second_member(&mut self, exp: GeneralExpressionHandle) {
        self.second_member = Some(exp);
    }

    /// Get the first member (LHS).
    pub fn first_member(&self) -> Option<GeneralExpressionHandle> {
        self.first_member.clone()
    }

    /// Get the second member (RHS).
    pub fn second_member(&self) -> Option<GeneralExpressionHandle> {
        self.second_member.clone()
    }

    /// Check if this relation is linear.
    pub fn is_linear(&self) -> bool {
        if let Some(ref fm) = self.first_member {
            if !fm.borrow().is_linear() {
                return false;
            }
        }
        if let Some(ref sm) = self.second_member {
            if !sm.borrow().is_linear() {
                return false;
            }
        }
        true
    }

    /// Get the number of sub-relations (always 0 for single relation).
    pub fn nb_of_sub_relations(&self) -> usize {
        0
    }

    /// Get the number of single relations (always 1).
    pub fn nb_of_single_relations(&self) -> usize {
        1
    }

    /// Get the sub-relation at the given index (always panics, as there are no sub-relations).
    pub fn sub_relation(&self, _index: usize) -> GeneralRelationHandle {
        panic!("Standard_OutOfRange");
    }

    /// Check if this relation contains the given expression.
    pub fn contains(&self, exp: &GeneralExpressionHandle) -> bool {
        if let Some(ref fm) = self.first_member {
            if std::ptr::eq(fm.as_ptr(), exp.as_ptr()) {
                return true;
            }
            if fm.borrow().contains(exp) {
                return true;
            }
        }
        if let Some(ref sm) = self.second_member {
            if std::ptr::eq(sm.as_ptr(), exp.as_ptr()) {
                return true;
            }
            if sm.borrow().contains(exp) {
                return true;
            }
        }
        false
    }

    /// Replace all occurrences of <var> with <with>.
    pub fn replace(&mut self, var: &NamedUnknownHandle, with_expr: &GeneralExpressionHandle) {
        if let Some(ref fm) = self.first_member.clone() {
            fm.borrow_mut().replace(var, with_expr);
        }
        if let Some(ref sm) = self.second_member.clone() {
            sm.borrow_mut().replace(var, with_expr);
        }
    }
}

impl Default for SingleRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_relation_creation() {
        let rel = SingleRelation::new();
        assert!(rel.first_member().is_none());
        assert!(rel.second_member().is_none());
    }

    #[test]
    fn test_set_first_member() {
        let mut rel = SingleRelation::new();
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        rel.set_first_member(exp.clone());
        assert!(rel.first_member().is_some());
    }

    #[test]
    fn test_set_second_member() {
        let mut rel = SingleRelation::new();
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        rel.set_second_member(exp.clone());
        assert!(rel.second_member().is_some());
    }

    #[test]
    fn test_is_linear_both_linear() {
        let mut rel = SingleRelation::new();
        let e1 = Rc::new(RefCell::new(GeneralExpression {}));
        let e2 = Rc::new(RefCell::new(GeneralExpression {}));
        rel.set_first_member(e1);
        rel.set_second_member(e2);
        assert!(rel.is_linear());
    }

    #[test]
    fn test_nb_of_sub_relations() {
        let rel = SingleRelation::new();
        assert_eq!(rel.nb_of_sub_relations(), 0);
    }

    #[test]
    fn test_nb_of_single_relations() {
        let rel = SingleRelation::new();
        assert_eq!(rel.nb_of_single_relations(), 1);
    }

    #[test]
    #[should_panic(expected = "Standard_OutOfRange")]
    fn test_sub_relation_panics() {
        let rel = SingleRelation::new();
        let _ = rel.sub_relation(1);
    }

    #[test]
    fn test_contains_first_member() {
        let mut rel = SingleRelation::new();
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        rel.set_first_member(exp.clone());
        assert!(rel.contains(&exp));
    }

    #[test]
    fn test_contains_second_member() {
        let mut rel = SingleRelation::new();
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        rel.set_second_member(exp.clone());
        assert!(rel.contains(&exp));
    }

    #[test]
    fn test_does_not_contain() {
        let rel = SingleRelation::new();
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        assert!(!rel.contains(&exp));
    }

    #[test]
    fn test_replace_does_not_panic() {
        let mut rel = SingleRelation::new();
        let e1 = Rc::new(RefCell::new(GeneralExpression {}));
        let e2 = Rc::new(RefCell::new(GeneralExpression {}));
        rel.set_first_member(e1);
        rel.set_second_member(e2);

        let var = Rc::new(RefCell::new(NamedUnknown {
            name: "x".into(),
        }));
        let with_expr = Rc::new(RefCell::new(GeneralExpression {}));

        rel.replace(&var, &with_expr);
        // Just verify no panic occurred
        assert!(true);
    }
}
