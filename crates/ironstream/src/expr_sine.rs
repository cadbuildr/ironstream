// FILE: expr_sine.rs
// occt: Expr_Sine

//! Sine expression node, mirroring OCCT `Expr_Sine` (Expr package):
//! - `ShallowSimplified`: sin(numeric) folds to a numeric value and
//!   Sin(ArcSin(u)) collapses to u; otherwise the sine node itself.
//! - `Derivative`: d/dX sin(u) = cos(u) * du/dX (chain rule), simplified.
//! - `Evaluate`: sin of the evaluated operand.
//! - `String`: "Sin(<operand>)".
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
    Sine(GeneralExpressionHandle),
    Cosine(GeneralExpressionHandle),
    ArcSine(GeneralExpressionHandle),
    Product(Vec<GeneralExpressionHandle>),
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
            GeneralExpression::Sine(op)
            | GeneralExpression::Cosine(op)
            | GeneralExpression::ArcSine(op) => op.borrow().contains(name),
            GeneralExpression::Product(ops) => ops.iter().any(|o| o.borrow().contains(name)),
        }
    }

    /// Does this expression contain any unknown?
    pub fn contains_unknowns(&self) -> bool {
        match self {
            GeneralExpression::NumericValue(_) => false,
            GeneralExpression::NamedUnknown(_) => true,
            GeneralExpression::Sine(op)
            | GeneralExpression::Cosine(op)
            | GeneralExpression::ArcSine(op) => op.borrow().contains_unknowns(),
            GeneralExpression::Product(ops) => {
                ops.iter().any(|o| o.borrow().contains_unknowns())
            }
        }
    }

    /// Derivative with respect to a named unknown; the local model supports
    /// all variants except ArcSine (whose derivative needs nodes outside
    /// this model).
    pub fn derivative(&self, name: &str) -> Result<GeneralExpression, String> {
        if !self.contains(name) {
            return Ok(GeneralExpression::NumericValue(0.0));
        }
        match self {
            GeneralExpression::NumericValue(_) => Ok(GeneralExpression::NumericValue(0.0)),
            GeneralExpression::NamedUnknown(_) => Ok(GeneralExpression::NumericValue(1.0)),
            GeneralExpression::Sine(op) => {
                let der = op.borrow().derivative(name)?;
                Ok(simplify_product(vec![
                    handle(GeneralExpression::Cosine(op.clone())),
                    handle(der),
                ]))
            }
            GeneralExpression::Cosine(op) => {
                let der = op.borrow().derivative(name)?;
                Ok(simplify_product(vec![
                    handle(GeneralExpression::NumericValue(-1.0)),
                    handle(GeneralExpression::Sine(op.clone())),
                    handle(der),
                ]))
            }
            GeneralExpression::ArcSine(_) => {
                Err("derivative of ArcSine is not supported by this local model".to_string())
            }
            GeneralExpression::Product(ops) => {
                // Product rule limited to two factors, enough for chain-rule results.
                if ops.len() != 2 {
                    return Err("product derivative supported for 2 factors only".to_string());
                }
                let da = ops[0].borrow().derivative(name)?;
                let db = ops[1].borrow().derivative(name)?;
                // (a*b)' = a'*b + a*b' ; keep it as a two-term product sum is not
                // in the model, so only handle the common case where one side is
                // constant with respect to `name`.
                if !ops[0].borrow().contains(name) {
                    Ok(simplify_product(vec![ops[0].clone(), handle(db)]))
                } else if !ops[1].borrow().contains(name) {
                    Ok(simplify_product(vec![handle(da), ops[1].clone()]))
                } else {
                    Err("general product rule needs a Sum node, not in this model".to_string())
                }
            }
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
            GeneralExpression::Sine(op) => Ok(op.borrow().evaluate(vars)?.sin()),
            GeneralExpression::Cosine(op) => Ok(op.borrow().evaluate(vars)?.cos()),
            GeneralExpression::ArcSine(op) => Ok(op.borrow().evaluate(vars)?.asin()),
            GeneralExpression::Product(ops) => {
                let mut acc = 1.0;
                for op in ops {
                    acc *= op.borrow().evaluate(vars)?;
                }
                Ok(acc)
            }
        }
    }

    /// String representation.
    pub fn string(&self) -> String {
        match self {
            GeneralExpression::NumericValue(v) => format!("{}", v),
            GeneralExpression::NamedUnknown(n) => n.clone(),
            GeneralExpression::Sine(op) => format!("Sin({})", op.borrow().string()),
            GeneralExpression::Cosine(op) => format!("Cos({})", op.borrow().string()),
            GeneralExpression::ArcSine(op) => format!("ASin({})", op.borrow().string()),
            GeneralExpression::Product(ops) => ops
                .iter()
                .map(|o| o.borrow().string())
                .collect::<Vec<_>>()
                .join("*"),
        }
    }
}

/// Builds a simplified product: folds numeric factors, drops unit factors,
/// collapses to 0 when any factor is 0 (Expr_Product::ShallowSimplified).
fn simplify_product(factors: Vec<GeneralExpressionHandle>) -> GeneralExpression {
    let mut numeric = 1.0;
    let mut symbolic: Vec<GeneralExpressionHandle> = Vec::new();
    for f in factors {
        let val = match &*f.borrow() {
            GeneralExpression::NumericValue(v) => Some(*v),
            _ => None,
        };
        match val {
            Some(v) => numeric *= v,
            None => symbolic.push(f),
        }
    }
    if numeric == 0.0 {
        return GeneralExpression::NumericValue(0.0);
    }
    if symbolic.is_empty() {
        return GeneralExpression::NumericValue(numeric);
    }
    if numeric == 1.0 && symbolic.len() == 1 {
        return symbolic[0].borrow().clone();
    }
    let mut ops = Vec::new();
    if numeric != 1.0 {
        ops.push(handle(GeneralExpression::NumericValue(numeric)));
    }
    ops.extend(symbolic);
    if ops.len() == 1 {
        return ops[0].borrow().clone();
    }
    GeneralExpression::Product(ops)
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

/// Sine function: sin(x).
/// occt: Expr_Sine // (unary expression)
#[derive(Clone, Debug)]
pub struct Sine {
    base: UnaryExpression,
}

impl Sine {
    /// Creates the sine of <exp>.
    pub fn new(exp: GeneralExpressionHandle) -> Self {
        Sine {
            base: UnaryExpression { operand: exp },
        }
    }

    /// Get the operand.
    pub fn operand(&self) -> &GeneralExpressionHandle {
        self.base.operand()
    }

    /// Returns a GeneralExpression after a simplification of the arguments
    /// (Expr_Sine::ShallowSimplified).
    pub fn shallow_simplified(&self) -> GeneralExpressionHandle {
        let op = self.base.operand();
        let simplified = match &*op.borrow() {
            GeneralExpression::NumericValue(v) => Some(GeneralExpression::NumericValue(v.sin())),
            GeneralExpression::ArcSine(inner) => Some(inner.borrow().clone()),
            _ => None,
        };
        match simplified {
            Some(e) => handle(e),
            None => self.copy(),
        }
    }

    /// Returns a copy of this expression sharing the operand (Expr::CopyShare).
    pub fn copy(&self) -> GeneralExpressionHandle {
        handle(GeneralExpression::Sine(self.base.operand().clone()))
    }

    /// Tests if this and Other define the same expression.
    pub fn is_identical(&self, other: &GeneralExpressionHandle) -> bool {
        match &*other.borrow() {
            GeneralExpression::Sine(op2) => *self.base.operand().borrow() == *op2.borrow(),
            _ => false,
        }
    }

    /// Is this expression linear (i.e., no unknowns)?
    pub fn is_linear(&self) -> bool {
        !self.base.contains_unknowns()
    }

    /// Returns the derivative with respect to X:
    /// d/dX sin(u) = cos(u) * du/dX.
    pub fn derivative(&self, x: &NamedUnknownHandle) -> Result<GeneralExpressionHandle, String> {
        let name = x.borrow().name.clone();
        if !self.base.contains(x) {
            return Ok(handle(GeneralExpression::NumericValue(0.0)));
        }
        let op = self.base.operand();
        let der = op.borrow().derivative(&name)?;
        Ok(handle(simplify_product(vec![
            handle(GeneralExpression::Cosine(op.clone())),
            handle(der),
        ])))
    }

    /// Evaluates this expression given variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        Ok(self.base.operand().borrow().evaluate(vars)?.sin())
    }

    /// Returns a string representation of this expression.
    pub fn string(&self) -> String {
        format!("Sin({})", self.base.operand().borrow().string())
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
    fn test_sine_creation() {
        let exp = var("x");
        let sine = Sine::new(exp.clone());
        assert!(std::ptr::eq(sine.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_sine_copy_shares_operand() {
        let exp = var("x");
        let sine = Sine::new(exp.clone());
        let copy = sine.copy();
        match &*copy.borrow() {
            GeneralExpression::Sine(op) => assert!(Rc::ptr_eq(op, &exp)),
            other => panic!("expected Sine node, got {:?}", other),
        };
    }

    #[test]
    fn test_sine_is_linear() {
        assert!(Sine::new(num(2.0)).is_linear());
        assert!(!Sine::new(var("x")).is_linear());
    }

    #[test]
    fn test_sine_string() {
        let sine = Sine::new(var("x"));
        assert_eq!(sine.string(), "Sin(x)");
    }

    #[test]
    fn test_shallow_simplified_numeric_folds() {
        let sine = Sine::new(num(0.5));
        let simplified = sine.shallow_simplified();
        match &*simplified.borrow() {
            GeneralExpression::NumericValue(v) => {
                assert!((v - 0.5f64.sin()).abs() < 1e-15);
            }
            other => panic!("expected numeric value, got {:?}", other),
        };
    }

    #[test]
    fn test_shallow_simplified_arcsine_collapses() {
        let x = var("x");
        let sine = Sine::new(handle(GeneralExpression::ArcSine(x.clone())));
        let simplified = sine.shallow_simplified();
        assert_eq!(
            *simplified.borrow(),
            GeneralExpression::NamedUnknown("x".to_string())
        );
    }

    #[test]
    fn test_shallow_simplified_symbolic_stays_sine() {
        let sine = Sine::new(var("x"));
        let simplified = sine.shallow_simplified();
        assert!(matches!(
            &*simplified.borrow(),
            GeneralExpression::Sine(_)
        ));
    }

    #[test]
    fn test_is_identical() {
        let sine = Sine::new(var("x"));
        let same = handle(GeneralExpression::Sine(var("x")));
        let different = handle(GeneralExpression::Sine(var("y")));
        let not_sine = num(1.0);
        assert!(sine.is_identical(&same));
        assert!(!sine.is_identical(&different));
        assert!(!sine.is_identical(&not_sine));
    }

    #[test]
    fn test_derivative_of_constant_is_zero() {
        let sine = Sine::new(num(3.0));
        let deriv = sine.derivative(&unknown("x")).unwrap();
        assert_eq!(*deriv.borrow(), GeneralExpression::NumericValue(0.0));
    }

    #[test]
    fn test_derivative_of_sin_x_is_cos_x() {
        let sine = Sine::new(var("x"));
        let deriv = sine.derivative(&unknown("x")).unwrap();
        // d/dx sin(x) = cos(x) * 1 = cos(x)
        assert_eq!(deriv.borrow().string(), "Cos(x)");
        // Verify numerically at a sample point.
        let at: f64 = 0.7;
        let expected = at.cos();
        assert!((deriv.borrow().evaluate(&[("x", at)]).unwrap() - expected).abs() < 1e-15);
    }

    #[test]
    fn test_evaluate() {
        let sine = Sine::new(var("x"));
        let v = sine.evaluate(&[("x", 1.2)]).unwrap();
        assert!((v - 1.2f64.sin()).abs() < 1e-15);
        assert!(sine.evaluate(&[]).is_err()); // unbound variable
    }

    #[test]
    fn test_unary_base_sub_expression() {
        let exp = var("x");
        let sine = Sine::new(exp.clone());
        assert_eq!(sine.base.nb_sub_expressions(), 1);
        assert!(Rc::ptr_eq(&sine.base.sub_expression(1).unwrap(), &exp));
        assert!(sine.base.sub_expression(2).is_none());
    }
}
