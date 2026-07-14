// FILE: expr_sign.rs
// occt: Expr_Sign

//! Sign expression node, mirroring OCCT `Expr_Sign` (Expr package):
//! - `ShallowSimplified`: sign(numeric) folds to a numeric -1/0/+1;
//!   otherwise the sign node itself.
//! - `Derivative`: always the numeric value 0 (as in OCCT, ignoring the
//!   discontinuity at 0).
//! - `Evaluate`: Expr::Sign of the evaluated operand.
//! - `String`: "Sign(<operand>)".
//!
//! The Expr expression hierarchy (Expr_GeneralExpression and friends) is
//! external plumbing modeled locally as a small enum-based expression tree.

use std::cell::RefCell;
use std::rc::Rc;

/// Represents a handle to a GeneralExpression (simulated via Rc).
pub type GeneralExpressionHandle = Rc<RefCell<GeneralExpression>>;

/// Represents a handle to a NamedUnknown (simulated via Rc).
pub type NamedUnknownHandle = Rc<RefCell<NamedUnknown>>;

/// A named unknown variable.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedUnknown {
    pub name: String,
}

/// Local model of the Expr_GeneralExpression hierarchy.
#[derive(Clone, Debug, PartialEq)]
pub enum GeneralExpression {
    NumericValue(f64),
    NamedUnknown(String),
    Sign(GeneralExpressionHandle),
}

pub fn handle(e: GeneralExpression) -> GeneralExpressionHandle {
    Rc::new(RefCell::new(e))
}

impl GeneralExpression {
    /// Does this expression contain the named unknown?
    pub fn contains(&self, name: &str) -> bool {
        match self {
            GeneralExpression::NumericValue(_) => false,
            GeneralExpression::NamedUnknown(n) => n == name,
            GeneralExpression::Sign(op) => op.borrow().contains(name),
        }
    }

    /// Does this expression contain any unknown?
    pub fn contains_unknowns(&self) -> bool {
        match self {
            GeneralExpression::NumericValue(_) => false,
            GeneralExpression::NamedUnknown(_) => true,
            GeneralExpression::Sign(op) => op.borrow().contains_unknowns(),
        }
    }

    /// Evaluates this expression given variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        match self {
            GeneralExpression::NumericValue(v) => Ok(*v),
            GeneralExpression::NamedUnknown(n) => vars
                .iter()
                .find(|(name, _)| name == n)
                .map(|(_, v)| *v)
                .ok_or_else(|| format!("unbound variable '{}'", n)),
            GeneralExpression::Sign(op) => Ok(compute_sign(op.borrow().evaluate(vars)?)),
        }
    }

    /// String representation.
    pub fn string(&self) -> String {
        match self {
            GeneralExpression::NumericValue(v) => format!("{}", v),
            GeneralExpression::NamedUnknown(n) => n.clone(),
            GeneralExpression::Sign(op) => format!("Sign({})", op.borrow().string()),
        }
    }
}

/// Unary expression base class (Expr_UnaryExpression).
#[derive(Clone, Debug)]
pub struct UnaryExpression {
    operand: GeneralExpressionHandle,
}

impl UnaryExpression {
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
        self.operand.borrow().contains_unknowns()
    }

    /// Does this expression contain the given unknown?
    pub fn contains(&self, unknown: &NamedUnknownHandle) -> bool {
        self.operand.borrow().contains(&unknown.borrow().name)
    }
}

/// Sign function: returns -1, 0, or 1 depending on the sign of the argument.
/// occt: Expr_Sign // (unary expression)
#[derive(Clone, Debug)]
pub struct Sign {
    base: UnaryExpression,
}

impl Sign {
    /// Creates the sign of <exp>.
    pub fn new(exp: GeneralExpressionHandle) -> Self {
        Sign {
            base: UnaryExpression { operand: exp },
        }
    }

    /// Get the operand.
    pub fn operand(&self) -> &GeneralExpressionHandle {
        self.base.operand()
    }

    /// Returns a GeneralExpression after a simplification of the arguments
    /// (Expr_Sign::ShallowSimplified): a numeric operand folds to
    /// Expr::Sign of its value; otherwise the sign node itself.
    pub fn shallow_simplified(&self) -> GeneralExpressionHandle {
        let op = self.base.operand();
        let simplified = match &*op.borrow() {
            GeneralExpression::NumericValue(v) => {
                Some(GeneralExpression::NumericValue(compute_sign(*v)))
            }
            _ => None,
        };
        match simplified {
            Some(e) => handle(e),
            None => self.copy(),
        }
    }

    /// Returns a copy of this expression sharing the operand (Expr::CopyShare).
    pub fn copy(&self) -> GeneralExpressionHandle {
        handle(GeneralExpression::Sign(self.base.operand().clone()))
    }

    /// Tests if this and Other define the same expression.
    pub fn is_identical(&self, other: &GeneralExpressionHandle) -> bool {
        match &*other.borrow() {
            GeneralExpression::Sign(op2) => *self.base.operand().borrow() == *op2.borrow(),
            _ => false,
        }
    }

    /// Is this expression linear (i.e., no unknowns)?
    pub fn is_linear(&self) -> bool {
        !self.base.contains_unknowns()
    }

    /// Returns the derivative with respect to X: always 0
    /// (Expr_Sign::Derivative returns new Expr_NumericValue(0.0)).
    pub fn derivative(&self, _x: &NamedUnknownHandle) -> GeneralExpressionHandle {
        handle(GeneralExpression::NumericValue(0.0))
    }

    /// Evaluates this expression given variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        Ok(compute_sign(self.base.operand().borrow().evaluate(vars)?))
    }

    /// Returns a string representation of this expression.
    pub fn string(&self) -> String {
        format!("Sign({})", self.base.operand().borrow().string())
    }
}

/// Helper: compute the sign of a real number (-1, 0, or 1), like Expr::Sign.
fn compute_sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(name: &str) -> GeneralExpressionHandle {
        handle(GeneralExpression::NamedUnknown(name.to_string()))
    }

    fn num(v: f64) -> GeneralExpressionHandle {
        handle(GeneralExpression::NumericValue(v))
    }

    fn unknown(name: &str) -> NamedUnknownHandle {
        Rc::new(RefCell::new(NamedUnknown {
            name: name.to_string(),
        }))
    }

    #[test]
    fn test_sign_creation() {
        let exp = var("x");
        let sign = Sign::new(exp.clone());
        assert!(std::ptr::eq(sign.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_sign_copy_shares_operand() {
        let exp = var("x");
        let sign = Sign::new(exp.clone());
        let copy = sign.copy();
        match &*copy.borrow() {
            GeneralExpression::Sign(op) => assert!(Rc::ptr_eq(op, &exp)),
            other => panic!("expected Sign node, got {:?}", other),
        };
    }

    #[test]
    fn test_sign_is_linear() {
        assert!(Sign::new(num(-2.0)).is_linear());
        assert!(!Sign::new(var("x")).is_linear());
    }

    #[test]
    fn test_sign_string() {
        let sign = Sign::new(var("x"));
        assert_eq!(sign.string(), "Sign(x)");
    }

    #[test]
    fn test_shallow_simplified_numeric_folds() {
        assert_eq!(
            *Sign::new(num(7.5)).shallow_simplified().borrow(),
            GeneralExpression::NumericValue(1.0)
        );
        assert_eq!(
            *Sign::new(num(-0.1)).shallow_simplified().borrow(),
            GeneralExpression::NumericValue(-1.0)
        );
        assert_eq!(
            *Sign::new(num(0.0)).shallow_simplified().borrow(),
            GeneralExpression::NumericValue(0.0)
        );
    }

    #[test]
    fn test_shallow_simplified_symbolic_stays_sign() {
        let sign = Sign::new(var("x"));
        let simplified = sign.shallow_simplified();
        assert!(matches!(&*simplified.borrow(), GeneralExpression::Sign(_)));
    }

    #[test]
    fn test_is_identical() {
        let sign = Sign::new(var("x"));
        let same = handle(GeneralExpression::Sign(var("x")));
        let different = handle(GeneralExpression::Sign(var("y")));
        let not_sign = num(1.0);
        assert!(sign.is_identical(&same));
        assert!(!sign.is_identical(&different));
        assert!(!sign.is_identical(&not_sign));
    }

    #[test]
    fn test_compute_sign_positive() {
        assert_eq!(compute_sign(5.0), 1.0);
    }

    #[test]
    fn test_compute_sign_negative() {
        assert_eq!(compute_sign(-5.0), -1.0);
    }

    #[test]
    fn test_compute_sign_zero() {
        assert_eq!(compute_sign(0.0), 0.0);
    }

    #[test]
    fn test_derivative_is_zero() {
        let sign = Sign::new(var("x"));
        let deriv = sign.derivative(&unknown("x"));
        assert_eq!(*deriv.borrow(), GeneralExpression::NumericValue(0.0));
    }

    #[test]
    fn test_evaluate() {
        let sign = Sign::new(var("x"));
        assert_eq!(sign.evaluate(&[("x", 42.0)]).unwrap(), 1.0);
        assert_eq!(sign.evaluate(&[("x", -3.5)]).unwrap(), -1.0);
        assert_eq!(sign.evaluate(&[("x", 0.0)]).unwrap(), 0.0);
        assert!(sign.evaluate(&[]).is_err()); // unbound variable
    }

    #[test]
    fn test_unary_base_sub_expression() {
        let exp = var("x");
        let sign = Sign::new(exp.clone());
        assert_eq!(sign.base.nb_sub_expressions(), 1);
        assert!(Rc::ptr_eq(&sign.base.sub_expression(1).unwrap(), &exp));
        assert!(sign.base.sub_expression(2).is_none());
    }
}
