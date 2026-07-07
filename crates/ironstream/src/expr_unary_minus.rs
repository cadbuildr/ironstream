// FILE: expr_unary_minus.rs
// occt: Expr_UnaryMinus

//! Port of OCCT Expr_UnaryMinus (unary expression -u).
//!
//! External plumbing (Expr_GeneralExpression hierarchy) is modeled by a
//! small local expression tree `Expr`; the UnaryMinus behavior itself
//! (ShallowSimplified, Copy, IsIdentical, IsLinear, Derivative,
//! NDerivative, Evaluate, String) follows Expr_UnaryMinus.cxx.

/// Minimal local model of the Expr_GeneralExpression hierarchy.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Expr_NumericValue
    Numeric(f64),
    /// Expr_NamedUnknown
    Unknown(String),
    /// Expr_UnaryMinus
    UnaryMinus(Box<Expr>),
    /// Expr_Sum (binary)
    Sum(Box<Expr>, Box<Expr>),
    /// Expr_Product (binary)
    Product(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Number of direct sub-expressions.
    pub fn nb_sub_expressions(&self) -> usize {
        match self {
            Expr::Numeric(_) | Expr::Unknown(_) => 0,
            Expr::UnaryMinus(_) => 1,
            Expr::Sum(..) | Expr::Product(..) => 2,
        }
    }

    /// 1-indexed access to sub-expressions (Standard_OutOfRange on bad index).
    pub fn sub_expression(&self, i: usize) -> &Expr {
        match (self, i) {
            (Expr::UnaryMinus(a), 1) => a,
            (Expr::Sum(a, _), 1) | (Expr::Product(a, _), 1) => a,
            (Expr::Sum(_, b), 2) | (Expr::Product(_, b), 2) => b,
            _ => panic!("Standard_OutOfRange"),
        }
    }

    /// Does this expression contain the named unknown?
    pub fn contains(&self, name: &str) -> bool {
        match self {
            Expr::Numeric(_) => false,
            Expr::Unknown(n) => n == name,
            Expr::UnaryMinus(a) => a.contains(name),
            Expr::Sum(a, b) | Expr::Product(a, b) => a.contains(name) || b.contains(name),
        }
    }

    /// Is this expression linear?
    pub fn is_linear(&self) -> bool {
        match self {
            Expr::Numeric(_) | Expr::Unknown(_) => true,
            Expr::UnaryMinus(a) => a.is_linear(),
            Expr::Sum(a, b) => a.is_linear() && b.is_linear(),
            Expr::Product(a, b) => {
                a.is_linear()
                    && b.is_linear()
                    && !(a.contains_unknowns() && b.contains_unknowns())
            }
        }
    }

    /// Does this expression contain any unknowns?
    pub fn contains_unknowns(&self) -> bool {
        match self {
            Expr::Numeric(_) => false,
            Expr::Unknown(_) => true,
            Expr::UnaryMinus(a) => a.contains_unknowns(),
            Expr::Sum(a, b) | Expr::Product(a, b) => {
                a.contains_unknowns() || b.contains_unknowns()
            }
        }
    }

    /// Derivative with respect to the named unknown.
    pub fn derivative(&self, x: &str) -> Expr {
        match self {
            Expr::Numeric(_) => Expr::Numeric(0.0),
            Expr::Unknown(n) => Expr::Numeric(if n == x { 1.0 } else { 0.0 }),
            Expr::UnaryMinus(a) => UnaryMinus::new(a.derivative(x)).shallow_simplified(),
            Expr::Sum(a, b) => {
                let da = a.derivative(x);
                let db = b.derivative(x);
                match (&da, &db) {
                    (Expr::Numeric(u), Expr::Numeric(v)) => Expr::Numeric(u + v),
                    _ => Expr::Sum(Box::new(da), Box::new(db)),
                }
            }
            Expr::Product(a, b) => {
                let left = Expr::Product(Box::new(a.derivative(x)), b.clone());
                let right = Expr::Product(a.clone(), Box::new(b.derivative(x)));
                Expr::Sum(Box::new(left), Box::new(right))
            }
        }
    }

    /// Evaluate with variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        match self {
            Expr::Numeric(v) => Ok(*v),
            Expr::Unknown(n) => vars
                .iter()
                .find(|(name, _)| *name == n.as_str())
                .map(|(_, v)| *v)
                .ok_or_else(|| format!("Expr_NotEvaluable: {}", n)),
            Expr::UnaryMinus(a) => Ok(-a.evaluate(vars)?),
            Expr::Sum(a, b) => Ok(a.evaluate(vars)? + b.evaluate(vars)?),
            Expr::Product(a, b) => Ok(a.evaluate(vars)? * b.evaluate(vars)?),
        }
    }

    /// Textual form.
    pub fn string(&self) -> String {
        match self {
            Expr::Numeric(v) => format!("{}", v),
            Expr::Unknown(n) => n.clone(),
            Expr::UnaryMinus(a) => {
                if a.nb_sub_expressions() > 1 {
                    format!("-({})", a.string())
                } else {
                    format!("-{}", a.string())
                }
            }
            Expr::Sum(a, b) => format!("{}+{}", a.string(), b.string()),
            Expr::Product(a, b) => format!("({})*({})", a.string(), b.string()),
        }
    }
}

/// Unary minus expression: -operand. Port of Expr_UnaryMinus.
#[derive(Clone, Debug)]
pub struct UnaryMinus {
    operand: Expr,
}

impl UnaryMinus {
    /// Creates -<exp>.
    pub fn new(exp: Expr) -> Self {
        UnaryMinus { operand: exp }
    }

    /// The operand (SubExpression(1)).
    pub fn operand(&self) -> &Expr {
        &self.operand
    }

    /// -(numeric) -> numeric with negated value, -(-x) -> x, otherwise self.
    pub fn shallow_simplified(&self) -> Expr {
        if let Expr::Numeric(v) = &self.operand {
            return Expr::Numeric(-v);
        }
        if let Expr::UnaryMinus(inner) = &self.operand {
            return (**inner).clone();
        }
        self.copy()
    }

    /// Returns a copy of this expression.
    pub fn copy(&self) -> Expr {
        Expr::UnaryMinus(Box::new(self.operand.clone()))
    }

    /// Tests if this and Other define the same expression.
    pub fn is_identical(&self, other: &Expr) -> bool {
        if let Expr::UnaryMinus(inner) = other {
            self.operand == **inner
        } else {
            false
        }
    }

    /// Linear iff the operand is linear.
    pub fn is_linear(&self) -> bool {
        self.operand.is_linear()
    }

    /// d/dx (-u) = -(du/dx), shallow-simplified.
    pub fn derivative(&self, x: &str) -> Expr {
        let myder = self.operand.derivative(x);
        UnaryMinus::new(myder).shallow_simplified()
    }

    /// N-th derivative; N must be strictly positive (Standard_OutOfRange).
    pub fn n_derivative(&self, x: &str, n: i32) -> Result<Expr, String> {
        if n <= 0 {
            return Err("Standard_OutOfRange".into());
        }
        let mut der = self.operand.clone();
        for _ in 0..n {
            der = der.derivative(x);
        }
        Ok(UnaryMinus::new(der).shallow_simplified())
    }

    /// Evaluates this expression given variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        Ok(-self.operand.evaluate(vars)?)
    }

    /// "-op" or "-(op)" when the operand has more than one sub-expression.
    pub fn string(&self) -> String {
        if self.operand.nb_sub_expressions() > 1 {
            format!("-({})", self.operand.string())
        } else {
            format!("-{}", self.operand.string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x() -> Expr {
        Expr::Unknown("x".into())
    }

    #[test]
    fn test_creation_and_operand() {
        let um = UnaryMinus::new(x());
        assert_eq!(*um.operand(), x());
    }

    #[test]
    fn test_shallow_simplified_numeric() {
        let um = UnaryMinus::new(Expr::Numeric(3.5));
        assert_eq!(um.shallow_simplified(), Expr::Numeric(-3.5));
    }

    #[test]
    fn test_shallow_simplified_double_minus() {
        let um = UnaryMinus::new(Expr::UnaryMinus(Box::new(x())));
        assert_eq!(um.shallow_simplified(), x());
    }

    #[test]
    fn test_shallow_simplified_default_returns_self() {
        let um = UnaryMinus::new(x());
        assert_eq!(um.shallow_simplified(), um.copy());
    }

    #[test]
    fn test_copy_is_identical() {
        let um = UnaryMinus::new(x());
        assert!(um.is_identical(&um.copy()));
        assert!(!um.is_identical(&x()));
        let other = UnaryMinus::new(Expr::Unknown("y".into()));
        assert!(!um.is_identical(&other.copy()));
    }

    #[test]
    fn test_is_linear() {
        assert!(UnaryMinus::new(x()).is_linear());
        // -(x*y) is not linear
        let prod = Expr::Product(Box::new(x()), Box::new(Expr::Unknown("y".into())));
        assert!(!UnaryMinus::new(prod).is_linear());
    }

    #[test]
    fn test_derivative() {
        // d/dx (-x) = -1
        let um = UnaryMinus::new(x());
        assert_eq!(um.derivative("x"), Expr::Numeric(-1.0));
        // d/dy (-x) = -(0) = 0 numerically
        assert_eq!(um.derivative("y").evaluate(&[]).unwrap(), 0.0);
    }

    #[test]
    fn test_n_derivative() {
        let um = UnaryMinus::new(x());
        assert!(um.n_derivative("x", 0).is_err());
        assert_eq!(um.n_derivative("x", 1).unwrap(), Expr::Numeric(-1.0));
        assert_eq!(um.n_derivative("x", 2).unwrap().evaluate(&[]).unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate() {
        let um = UnaryMinus::new(x());
        assert_eq!(um.evaluate(&[("x", 7.0)]).unwrap(), -7.0);
        assert!(um.evaluate(&[]).is_err());
    }

    #[test]
    fn test_string() {
        assert_eq!(UnaryMinus::new(x()).string(), "-x");
        let sum = Expr::Sum(Box::new(x()), Box::new(Expr::Numeric(1.0)));
        assert_eq!(UnaryMinus::new(sum).string(), "-(x+1)");
    }
}
