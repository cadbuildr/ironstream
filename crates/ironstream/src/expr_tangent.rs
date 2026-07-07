// FILE: expr_tangent.rs
// occt: Expr_Tangent

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

/// Tangent function: tan(x).
#[derive(Clone, Debug)]
pub struct Tangent {
    base: UnaryExpression,
}

impl Tangent {
    pub fn new(exp: GeneralExpressionHandle) -> Self {
        Tangent {
            base: UnaryExpression { operand: exp },
        }
    }

    pub fn operand(&self) -> &GeneralExpressionHandle { self.base.operand() }
    pub fn shallow_simplified(&self) -> GeneralExpressionHandle { self.copy() }
    pub fn copy(&self) -> GeneralExpressionHandle { Rc::new(RefCell::new(GeneralExpression {})) }
    pub fn is_identical(&self, _other: &GeneralExpressionHandle) -> bool { false }
    pub fn is_linear(&self) -> bool { !self.base.contains_unknowns() }
    pub fn derivative(&self, x: &NamedUnknownHandle) -> GeneralExpressionHandle {
        if !self.base.contains(x) {
            Rc::new(RefCell::new(GeneralExpression {}))
        } else {
            Rc::new(RefCell::new(GeneralExpression {}))
        }
    }
    pub fn evaluate(&self, _vars: &[(&str, f64)]) -> Result<f64, String> { Err("not implemented".into()) }
    pub fn string(&self) -> String { format!("Tan(...)") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tangent_creation() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let tan = Tangent::new(exp.clone());
        assert!(std::ptr::eq(tan.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_tangent_is_linear() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let tan = Tangent::new(exp);
        assert!(tan.is_linear());
    }

    #[test]
    fn test_tangent_string() {
        let exp = Rc::new(RefCell::new(GeneralExpression {}));
        let tan = Tangent::new(exp);
        assert!(tan.string().contains("Tan"));
    }
}
