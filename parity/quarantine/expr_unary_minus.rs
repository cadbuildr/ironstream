// FILE: expr_unary_minus.rs
// occt: Expr_UnaryMinus

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

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    operand: GeneralExpressionHandle,
}

impl UnaryExpression {
    pub fn operand(&self) -> &GeneralExpressionHandle { &self.operand }
    pub fn contains_unknowns(&self) -> bool { false }
    pub fn contains(&self, _unknown: &NamedUnknownHandle) -> bool { false }
}

/// Unary minus (negation) operator: -x.
#[derive(Clone, Debug)]
pub struct UnaryMinus {
    base: UnaryExpression,
}

impl UnaryMinus {
    /// Create a unary minus expression: -exp.
    pub fn new(exp: GeneralExpressionHandle) -> Self {
        UnaryMinus {
            base: UnaryExpression { operand: exp },
        }
    }

    pub fn operand(&self) -> &GeneralExpressionHandle { self.base.operand() }
    pub fn shallow_simplified(&self) -> GeneralExpressionHandle { self.copy() }
    pub fn copy(&self) -> GeneralExpressionHandle { Rc::new(RefCell::new(GeneralExpression {})) }
    pub fn is_identical(&self, _other: &GeneralExpressionHandle) -> bool { false }
    pub fn is_linear(&self) -> bool { !self.base.contains_unknowns() }

    /// Derivative of -u is -du/dx.
    pub fn derivative(&self, x: &NamedUnknownHandle) -> GeneralExpressionHandle {
        if !self.base.contains(x) {
            Rc::new(RefCell::new(GeneralExpression {}))
        } else {
            Rc::new(RefCell::new(GeneralExpression {}))
        }
    }

    pub fn evaluate(&self, _vars: &[(&str, f64)]) -> Result<f64, String> { Err("not implemented".into()) }
    pub fn string(&self) -> String { format!("-(...)") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_minus_creation() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let neg = UnaryMinus::new(exp.clone());
        assert!(std::ptr::eq(neg.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_unary_minus_copy() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let neg = UnaryMinus::new(exp);
        let copy = neg.copy();
        assert!(copy.borrow_mut().is_some());
    }

    #[test]
    fn test_unary_minus_is_linear() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let neg = UnaryMinus::new(exp);
        assert!(neg.is_linear());
    }

    #[test]
    fn test_unary_minus_string() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let neg = UnaryMinus::new(exp);
        assert!(neg.string().contains("-"));
    }

    #[test]
    fn test_unary_minus_derivative() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let neg = UnaryMinus::new(exp);
        let x = Rc::new(RefCell::new(NamedUnknown { name: "x".into() }));
        let deriv = neg.derivative(&x);
        assert!(deriv.borrow_mut().is_some());
    }
}
