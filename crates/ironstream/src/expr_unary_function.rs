// FILE: expr_unary_function.rs
// occt: Expr_UnaryFunction

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

/// Base class for unary functions (e.g., Sin, Cos, Sqrt, etc.).
#[derive(Clone, Debug)]
pub struct UnaryFunction {
    operand: GeneralExpressionHandle,
}

impl UnaryFunction {
    /// Create a unary function with the given operand.
    pub fn new(operand: GeneralExpressionHandle) -> Self {
        UnaryFunction { operand }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_function_creation() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let func = UnaryFunction::new(exp.clone());
        assert!(std::ptr::eq(func.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_unary_function_set_operand() {
        let exp1 = Rc::new(RefCell::new(GeneralExpression {}));
        let exp2 = Rc::new(RefCell::new(GeneralExpression {}));

        let mut func = UnaryFunction::new(exp1);
        func.set_operand(exp2.clone());
        assert!(std::ptr::eq(func.operand().as_ptr(), exp2.as_ptr()));
    }

    #[test]
    fn test_unary_function_nb_sub_expressions() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let func = UnaryFunction::new(exp);
        assert_eq!(func.nb_sub_expressions(), 1);
    }

    #[test]
    fn test_unary_function_sub_expression() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let func = UnaryFunction::new(exp.clone());

        assert!(func.sub_expression(1).is_some());
        assert!(func.sub_expression(2).is_none());
    }

    #[test]
    fn test_unary_function_contains() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let other = Rc::new(RefCell::new(GeneralExpression {}));

        let func = UnaryFunction::new(exp.clone());
        assert!(func.contains(&exp));
        assert!(!func.contains(&other));
    }

    #[test]
    fn test_unary_function_contains_unknowns() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let func = UnaryFunction::new(exp);
        assert!(!func.contains_unknowns());
    }
}
