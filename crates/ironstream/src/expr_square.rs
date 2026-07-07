// FILE: expr_square.rs
// occt: Expr_Square

//! Square expression node, mirroring OCCT `Expr_Square` (Expr package):
//! - `ShallowSimplified`: numeric operand folds to its square;
//!   Square(SquareRoot(u)) collapses to u; Square(Square(u)) becomes
//!   Exponentiate(u, 4); Square(Exponentiate(u, p)) becomes
//!   Exponentiate(u, 2*p); otherwise the square node itself.
//! - `Derivative`: d/dX u^2 = 2 * du/dX * u (product simplified).
//! - `Evaluate`: square of the evaluated operand.
//! - `String`: "<op>^2", parenthesized when the operand is composite.
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
    Square(GeneralExpressionHandle),
    SquareRoot(GeneralExpressionHandle),
    Exponentiate(GeneralExpressionHandle, GeneralExpressionHandle),
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
            GeneralExpression::Square(op) | GeneralExpression::SquareRoot(op) => {
                op.borrow().contains(name)
            }
            GeneralExpression::Exponentiate(base, power) => {
                base.borrow().contains(name) || power.borrow().contains(name)
            }
            GeneralExpression::Product(ops) => ops.iter().any(|o| o.borrow().contains(name)),
        }
    }

    /// Does this expression contain any unknown?
    pub fn contains_unknowns(&self) -> bool {
        match self {
            GeneralExpression::NumericValue(_) => false,
            GeneralExpression::NamedUnknown(_) => true,
            GeneralExpression::Square(op) | GeneralExpression::SquareRoot(op) => {
                op.borrow().contains_unknowns()
            }
            GeneralExpression::Exponentiate(base, power) => {
                base.borrow().contains_unknowns() || power.borrow().contains_unknowns()
            }
            GeneralExpression::Product(ops) => {
                ops.iter().any(|o| o.borrow().contains_unknowns())
            }
        }
    }

    /// Derivative with respect to a named unknown; the local model supports
    /// the variants needed for Expr_Square's chain rule.
    pub fn derivative(&self, name: &str) -> Result<GeneralExpression, String> {
        if !self.contains(name) {
            return Ok(GeneralExpression::NumericValue(0.0));
        }
        match self {
            GeneralExpression::NumericValue(_) => Ok(GeneralExpression::NumericValue(0.0)),
            GeneralExpression::NamedUnknown(_) => Ok(GeneralExpression::NumericValue(1.0)),
            GeneralExpression::Square(op) => {
                // d/dx u^2 = 2 * u' * u
                let der = op.borrow().derivative(name)?;
                Ok(simplify_product(vec![
                    handle(GeneralExpression::NumericValue(2.0)),
                    handle(der),
                    op.clone(),
                ]))
            }
            GeneralExpression::SquareRoot(_) => {
                Err("derivative of SquareRoot needs a Division node, not in this model".to_string())
            }
            GeneralExpression::Exponentiate(_, _) => {
                Err("derivative of Exponentiate is not supported by this local model".to_string())
            }
            GeneralExpression::Product(ops) => {
                if ops.len() != 2 {
                    return Err("product derivative supported for 2 factors only".to_string());
                }
                let da = ops[0].borrow().derivative(name)?;
                let db = ops[1].borrow().derivative(name)?;
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
            GeneralExpression::Square(op) => {
                let v = op.borrow().evaluate(vars)?;
                Ok(v * v)
            }
            GeneralExpression::SquareRoot(op) => Ok(op.borrow().evaluate(vars)?.sqrt()),
            GeneralExpression::Exponentiate(base, power) => {
                let b = base.borrow().evaluate(vars)?;
                let p = power.borrow().evaluate(vars)?;
                Ok(b.powf(p))
            }
            GeneralExpression::Product(ops) => {
                let mut acc = 1.0;
                for op in ops {
                    acc *= op.borrow().evaluate(vars)?;
                }
                Ok(acc)
            }
        }
    }

    /// True for atomic nodes (no parentheses needed when raising to a power).
    fn is_atomic(&self) -> bool {
        matches!(
            self,
            GeneralExpression::NumericValue(_) | GeneralExpression::NamedUnknown(_)
        )
    }

    /// String representation.
    pub fn string(&self) -> String {
        match self {
            GeneralExpression::NumericValue(v) => format!("{}", v),
            GeneralExpression::NamedUnknown(n) => n.clone(),
            GeneralExpression::Square(op) => {
                let inner = op.borrow();
                if inner.is_atomic() {
                    format!("{}^2", inner.string())
                } else {
                    format!("({})^2", inner.string())
                }
            }
            GeneralExpression::SquareRoot(op) => format!("Sqrt({})", op.borrow().string()),
            GeneralExpression::Exponentiate(base, power) => {
                let b = base.borrow();
                let base_str = if b.is_atomic() {
                    b.string()
                } else {
                    format!("({})", b.string())
                };
                format!("{}^{}", base_str, power.borrow().string())
            }
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

/// Square function: x^2.
/// occt: Expr_Square (unary expression)
#[derive(Clone, Debug)]
pub struct Square {
    base: UnaryExpression,
}

impl Square {
    /// Creates the square of <exp>.
    pub fn new(exp: GeneralExpressionHandle) -> Self {
        Square {
            base: UnaryExpression { operand: exp },
        }
    }

    /// Get the operand.
    pub fn operand(&self) -> &GeneralExpressionHandle {
        self.base.operand()
    }

    /// Returns a GeneralExpression after a simplification of the arguments
    /// (Expr_Square::ShallowSimplified).
    pub fn shallow_simplified(&self) -> GeneralExpressionHandle {
        let op = self.base.operand();
        let simplified = match &*op.borrow() {
            GeneralExpression::NumericValue(v) => Some(GeneralExpression::NumericValue(v * v)),
            // Square(Sqrt(u)) -> u
            GeneralExpression::SquareRoot(inner) => Some(inner.borrow().clone()),
            // Square(Square(u)) -> u^4
            GeneralExpression::Square(inner) => Some(GeneralExpression::Exponentiate(
                inner.clone(),
                handle(GeneralExpression::NumericValue(4.0)),
            )),
            // Square(u^p) -> u^(2*p)
            GeneralExpression::Exponentiate(base, power) => {
                let newpow = simplify_product(vec![
                    handle(GeneralExpression::NumericValue(2.0)),
                    power.clone(),
                ]);
                Some(GeneralExpression::Exponentiate(
                    base.clone(),
                    handle(newpow),
                ))
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
        handle(GeneralExpression::Square(self.base.operand().clone()))
    }

    /// Tests if this and Other define the same expression.
    pub fn is_identical(&self, other: &GeneralExpressionHandle) -> bool {
        match &*other.borrow() {
            GeneralExpression::Square(op2) => *self.base.operand().borrow() == *op2.borrow(),
            _ => false,
        }
    }

    /// Is this expression linear (i.e., no unknowns)?
    pub fn is_linear(&self) -> bool {
        !self.base.contains_unknowns()
    }

    /// Returns the derivative with respect to X:
    /// d/dX u^2 = 2 * du/dX * u.
    pub fn derivative(&self, x: &NamedUnknownHandle) -> Result<GeneralExpressionHandle, String> {
        let name = x.borrow().name.clone();
        if !self.base.contains(x) {
            return Ok(handle(GeneralExpression::NumericValue(0.0)));
        }
        let op = self.base.operand();
        let der = op.borrow().derivative(&name)?;
        Ok(handle(simplify_product(vec![
            handle(GeneralExpression::NumericValue(2.0)),
            handle(der),
            op.clone(),
        ])))
    }

    /// Evaluates this expression given variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        let v = self.base.operand().borrow().evaluate(vars)?;
        Ok(v * v)
    }

    /// Returns a string representation of this expression.
    pub fn string(&self) -> String {
        let op = self.base.operand().borrow();
        if op.is_atomic() {
            format!("{}^2", op.string())
        } else {
            format!("({})^2", op.string())
        }
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
    fn test_square_creation() {
        let exp = var("x");
        let sq = Square::new(exp.clone());
        assert!(std::ptr::eq(sq.operand().as_ptr(), exp.as_ptr()));
    }

    #[test]
    fn test_square_copy_shares_operand() {
        let exp = var("x");
        let sq = Square::new(exp.clone());
        let copy = sq.copy();
        match &*copy.borrow() {
            GeneralExpression::Square(op) => assert!(Rc::ptr_eq(op, &exp)),
            other => panic!("expected Square node, got {:?}", other),
        };
    }

    #[test]
    fn test_square_is_linear() {
        assert!(Square::new(num(2.0)).is_linear());
        assert!(!Square::new(var("x")).is_linear());
    }

    #[test]
    fn test_square_string() {
        assert_eq!(Square::new(var("x")).string(), "x^2");
        let composite = handle(GeneralExpression::SquareRoot(var("x")));
        assert_eq!(Square::new(composite).string(), "(Sqrt(x))^2");
    }

    #[test]
    fn test_shallow_simplified_numeric_folds() {
        let sq = Square::new(num(3.0));
        let simplified = sq.shallow_simplified();
        assert_eq!(*simplified.borrow(), GeneralExpression::NumericValue(9.0));
    }

    #[test]
    fn test_shallow_simplified_sqrt_collapses() {
        let sq = Square::new(handle(GeneralExpression::SquareRoot(var("x"))));
        let simplified = sq.shallow_simplified();
        assert_eq!(
            *simplified.borrow(),
            GeneralExpression::NamedUnknown("x".to_string())
        );
    }

    #[test]
    fn test_shallow_simplified_square_of_square_is_fourth_power() {
        let x = var("x");
        let sq = Square::new(handle(GeneralExpression::Square(x.clone())));
        let simplified = sq.shallow_simplified();
        match &*simplified.borrow() {
            GeneralExpression::Exponentiate(base, power) => {
                assert!(Rc::ptr_eq(base, &x));
                assert_eq!(*power.borrow(), GeneralExpression::NumericValue(4.0));
            }
            other => panic!("expected Exponentiate node, got {:?}", other),
        };
    }

    #[test]
    fn test_shallow_simplified_square_of_power_doubles_exponent() {
        let x = var("x");
        let sq = Square::new(handle(GeneralExpression::Exponentiate(
            x.clone(),
            num(3.0),
        )));
        let simplified = sq.shallow_simplified();
        match &*simplified.borrow() {
            GeneralExpression::Exponentiate(base, power) => {
                assert!(Rc::ptr_eq(base, &x));
                assert_eq!(*power.borrow(), GeneralExpression::NumericValue(6.0));
            }
            other => panic!("expected Exponentiate node, got {:?}", other),
        };
    }

    #[test]
    fn test_shallow_simplified_symbolic_stays_square() {
        let sq = Square::new(var("x"));
        let simplified = sq.shallow_simplified();
        assert!(matches!(
            &*simplified.borrow(),
            GeneralExpression::Square(_)
        ));
    }

    #[test]
    fn test_is_identical() {
        let sq = Square::new(var("x"));
        let same = handle(GeneralExpression::Square(var("x")));
        let different = handle(GeneralExpression::Square(var("y")));
        let not_square = num(4.0);
        assert!(sq.is_identical(&same));
        assert!(!sq.is_identical(&different));
        assert!(!sq.is_identical(&not_square));
    }

    #[test]
    fn test_derivative_of_constant_is_zero() {
        let sq = Square::new(num(5.0));
        let deriv = sq.derivative(&unknown("x")).unwrap();
        assert_eq!(*deriv.borrow(), GeneralExpression::NumericValue(0.0));
    }

    #[test]
    fn test_derivative_of_x_squared_is_2x() {
        let sq = Square::new(var("x"));
        let deriv = sq.derivative(&unknown("x")).unwrap();
        assert_eq!(deriv.borrow().string(), "2*x");
        // Verify numerically at a sample point: d/dx x^2 at 3 is 6.
        assert!((deriv.borrow().evaluate(&[("x", 3.0)]).unwrap() - 6.0).abs() < 1e-15);
    }

    #[test]
    fn test_evaluate() {
        let sq = Square::new(var("x"));
        assert_eq!(sq.evaluate(&[("x", 4.0)]).unwrap(), 16.0);
        assert!(sq.evaluate(&[]).is_err()); // unbound variable
    }

    #[test]
    fn test_unary_base_sub_expression() {
        let exp = var("x");
        let sq = Square::new(exp.clone());
        assert_eq!(sq.base.nb_sub_expressions(), 1);
        assert!(Rc::ptr_eq(&sq.base.sub_expression(1).unwrap(), &exp));
        assert!(sq.base.sub_expression(2).is_none());
    }
}
