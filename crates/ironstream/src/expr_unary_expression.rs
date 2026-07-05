// FILE: expr_unary_expression.rs
// occt: Expr_UnaryExpression

use std::rc::Rc;
use std::cell::RefCell;

pub type GeneralExpressionHandle = Rc<RefCell<GeneralExpression>>;
pub type NamedUnknownHandle = Rc<RefCell<NamedUnknown>>;

#[derive(Clone, Debug)]
pub struct GeneralExpression {}
impl GeneralExpression {
    pub fn sub_expression(&self, _i: usize) -> Option<GeneralExpressionHandle> { None }
}

#[derive(Clone, Debug)]
pub struct NamedUnknown { pub name: String }

/// Base class for unary expressions (expressions with one operand).
/// Examples: Sin(x), Sqrt(x), -x, etc.
#[derive(Clone, Debug)]
pub struct UnaryExpression {
    operand: GeneralExpressionHandle,
}

impl UnaryExpression {
    /// Create a unary expression with the given operand.
    pub fn new(operand: GeneralExpressionHandle) -> Self {
        UnaryExpression { operand }
    }

    /// Get the operand.
    pub fn operand(&self) -> &GeneralExpressionHandle {
        &self.operand
    }

    /// Set the operand.
    pub fn set_operand(&mut self, exp: GeneralExpressionHandle) {
        self.operand = exp;
    }

    /// Get the number of sub-expressions (always 1 for unary).
    pub fn nb_sub_expressions(&self) -> usize {
        1
    }

    /// Get the i-th sub-expression (1-indexed).
    pub fn sub_expression(&self, i: usize) -> Option<GeneralExpressionHandle> {
        if i == 1 {
            Some(self.operand.clone())
        } else {
            None
        }
    }

    /// Does this expression contain unknowns?
    pub fn contains_unknowns(&self) -> bool {
        false
    }

    /// Does this expression contain the given expression?
    pub fn contains(&self, exp: &GeneralExpressionHandle) -> bool {
        std::ptr::eq(self.operand.as_ptr(), exp.as_ptr())
    }

    /// Replace all occurrences of <var> with <with>.
    pub fn replace(&mut self, _var: &NamedUnknownHandle, _with: &GeneralExpressionHandle) {
        // Placeholder: in real implementation, would replace in operand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_expression_creation() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let unary = UnaryExpression::new(exp.clone());
        assert!(std::ptr::eq(unary.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_unary_expression_set_operand() {
        let exp1 = Rc::new(RefCell::new(GeneralExpression {}));
        let exp2 = Rc::new(RefCell::new(GeneralExpression {}));

        let mut unary = UnaryExpression::new(exp1);
        unary.set_operand(exp2.clone());
        assert!(std::ptr::eq(unary.operand().as_ptr(), exp2.as_ptr()));
    }

    #[test]
    fn test_unary_expression_nb_sub_expressions() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let unary = UnaryExpression::new(exp);
        assert_eq!(unary.nb_sub_expressions(), 1);
    }

    #[test]
    fn test_unary_expression_sub_expression() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let unary = UnaryExpression::new(exp.clone());

        assert!(unary.sub_expression(1).is_some());
        assert!(unary.sub_expression(2).is_none());
    }

    #[test]
    fn test_unary_expression_contains() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let other = Rc::new(RefCell::new(GeneralExpression {}));

        let unary = UnaryExpression::new(exp.clone());
        assert!(unary.contains(&exp));
        assert!(!unary.contains(&other));
    }

    #[test]
    fn test_unary_expression_contains_unknowns() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let unary = UnaryExpression::new(exp);
        assert!(!unary.contains_unknowns());
    }
}
