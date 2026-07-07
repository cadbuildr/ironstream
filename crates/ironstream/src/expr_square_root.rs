// FILE: expr_square_root.rs
// occt: Expr_SquareRoot

//! Port of OCCT Expr_SquareRoot (unary expression sqrt(u)).
//!
//! External plumbing (Expr_GeneralExpression hierarchy) is modeled by a
//! small local expression tree `Expr`; the SquareRoot behavior itself
//! (ShallowSimplified, Copy, IsIdentical, IsLinear, Derivative, Evaluate,
//! String) follows Expr_SquareRoot.cxx.

/// Minimal local model of the Expr_GeneralExpression hierarchy.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Expr_NumericValue
    Numeric(f64),
    /// Expr_NamedUnknown
    Unknown(String),
    /// Expr_UnaryMinus
    UnaryMinus(Box<Expr>),
    /// Expr_Square
    Square(Box<Expr>),
    /// Expr_SquareRoot
    SquareRoot(Box<Expr>),
    /// Expr_Sum (binary)
    Sum(Box<Expr>, Box<Expr>),
    /// Expr_Product (binary)
    Product(Box<Expr>, Box<Expr>),
    /// Expr_Division
    Division(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Number of direct sub-expressions.
    pub fn nb_sub_expressions(&self) -> usize {
        match self {
            Expr::Numeric(_) | Expr::Unknown(_) => 0,
            Expr::UnaryMinus(_) | Expr::Square(_) | Expr::SquareRoot(_) => 1,
            Expr::Sum(..) | Expr::Product(..) | Expr::Division(..) => 2,
        }
    }

    /// 1-indexed access to sub-expressions (Standard_OutOfRange on bad index).
    pub fn sub_expression(&self, i: usize) -> &Expr {
        match (self, i) {
            (Expr::UnaryMinus(a), 1) | (Expr::Square(a), 1) | (Expr::SquareRoot(a), 1) => a,
            (Expr::Sum(a, _), 1) | (Expr::Product(a, _), 1) | (Expr::Division(a, _), 1) => a,
            (Expr::Sum(_, b), 2) | (Expr::Product(_, b), 2) | (Expr::Division(_, b), 2) => b,
            _ => panic!("Standard_OutOfRange"),
        }
    }

    /// Does this expression contain the named unknown?
    pub fn contains(&self, name: &str) -> bool {
        match self {
            Expr::Numeric(_) => false,
            Expr::Unknown(n) => n == name,
            Expr::UnaryMinus(a) | Expr::Square(a) | Expr::SquareRoot(a) => a.contains(name),
            Expr::Sum(a, b) | Expr::Product(a, b) | Expr::Division(a, b) => {
                a.contains(name) || b.contains(name)
            }
        }
    }

    /// Does this expression contain any unknowns?
    pub fn contains_unknowns(&self) -> bool {
        match self {
            Expr::Numeric(_) => false,
            Expr::Unknown(_) => true,
            Expr::UnaryMinus(a) | Expr::Square(a) | Expr::SquareRoot(a) => a.contains_unknowns(),
            Expr::Sum(a, b) | Expr::Product(a, b) | Expr::Division(a, b) => {
                a.contains_unknowns() || b.contains_unknowns()
            }
        }
    }

    /// Simplifying sum constructor (mirrors ShallowSimplified of Expr_Sum
    /// for the numeric cases needed by derivatives).
    pub fn sum(a: Expr, b: Expr) -> Expr {
        match (&a, &b) {
            (Expr::Numeric(x), Expr::Numeric(y)) => return Expr::Numeric(x + y),
            (Expr::Numeric(x), _) if *x == 0.0 => return b,
            (_, Expr::Numeric(y)) if *y == 0.0 => return a,
            _ => {}
        }
        Expr::Sum(Box::new(a), Box::new(b))
    }

    /// Simplifying product constructor.
    pub fn product(a: Expr, b: Expr) -> Expr {
        match (&a, &b) {
            (Expr::Numeric(x), Expr::Numeric(y)) => return Expr::Numeric(x * y),
            (Expr::Numeric(x), _) if *x == 0.0 => return Expr::Numeric(0.0),
            (_, Expr::Numeric(y)) if *y == 0.0 => return Expr::Numeric(0.0),
            (Expr::Numeric(x), _) if *x == 1.0 => return b,
            (_, Expr::Numeric(y)) if *y == 1.0 => return a,
            _ => {}
        }
        Expr::Product(Box::new(a), Box::new(b))
    }

    /// Simplifying division constructor.
    pub fn division(a: Expr, b: Expr) -> Expr {
        match (&a, &b) {
            (Expr::Numeric(x), Expr::Numeric(y)) if *y != 0.0 => return Expr::Numeric(x / y),
            (Expr::Numeric(x), _) if *x == 0.0 => return Expr::Numeric(0.0),
            (_, Expr::Numeric(y)) if *y == 1.0 => return a,
            _ => {}
        }
        Expr::Division(Box::new(a), Box::new(b))
    }

    /// Simplifying unary-minus constructor.
    pub fn neg(a: Expr) -> Expr {
        match a {
            Expr::Numeric(v) => Expr::Numeric(-v),
            Expr::UnaryMinus(inner) => *inner,
            other => Expr::UnaryMinus(Box::new(other)),
        }
    }

    /// Derivative with respect to the named unknown.
    pub fn derivative(&self, x: &str) -> Expr {
        match self {
            Expr::Numeric(_) => Expr::Numeric(0.0),
            Expr::Unknown(n) => Expr::Numeric(if n == x { 1.0 } else { 0.0 }),
            Expr::UnaryMinus(a) => Expr::neg(a.derivative(x)),
            Expr::Square(a) => Expr::product(
                Expr::product(Expr::Numeric(2.0), (**a).clone()),
                a.derivative(x),
            ),
            Expr::SquareRoot(a) => Expr::division(
                a.derivative(x),
                Expr::product(Expr::Numeric(2.0), Expr::SquareRoot(a.clone())),
            ),
            Expr::Sum(a, b) => Expr::sum(a.derivative(x), b.derivative(x)),
            Expr::Product(a, b) => Expr::sum(
                Expr::product(a.derivative(x), (**b).clone()),
                Expr::product((**a).clone(), b.derivative(x)),
            ),
            Expr::Division(a, b) => Expr::division(
                Expr::sum(
                    Expr::product(a.derivative(x), (**b).clone()),
                    Expr::neg(Expr::product((**a).clone(), b.derivative(x))),
                ),
                Expr::Square(b.clone()),
            ),
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
            Expr::Square(a) => {
                let v = a.evaluate(vars)?;
                Ok(v * v)
            }
            Expr::SquareRoot(a) => Ok(a.evaluate(vars)?.sqrt()),
            Expr::Sum(a, b) => Ok(a.evaluate(vars)? + b.evaluate(vars)?),
            Expr::Product(a, b) => Ok(a.evaluate(vars)? * b.evaluate(vars)?),
            Expr::Division(a, b) => Ok(a.evaluate(vars)? / b.evaluate(vars)?),
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
            Expr::Square(a) => format!("({})^2", a.string()),
            Expr::SquareRoot(a) => format!("Sqrt({})", a.string()),
            Expr::Sum(a, b) => format!("{}+{}", a.string(), b.string()),
            Expr::Product(a, b) => format!("({})*({})", a.string(), b.string()),
            Expr::Division(a, b) => format!("({})/({})", a.string(), b.string()),
        }
    }
}

/// Square root expression: sqrt(operand). Port of Expr_SquareRoot.
#[derive(Clone, Debug)]
pub struct SquareRoot {
    operand: Expr,
}

impl SquareRoot {
    /// Creates the square root of `exp`.
    pub fn new(exp: Expr) -> Self {
        SquareRoot { operand: exp }
    }

    /// The operand (SubExpression(1)).
    pub fn operand(&self) -> &Expr {
        &self.operand
    }

    /// Returns a GeneralExpression after a simplification of the arguments:
    /// sqrt(numeric) -> numeric, sqrt(square(x)) -> x, otherwise self.
    pub fn shallow_simplified(&self) -> Expr {
        if let Expr::Numeric(v) = &self.operand {
            return Expr::Numeric(v.sqrt());
        }
        if let Expr::Square(inner) = &self.operand {
            return (**inner).clone();
        }
        self.copy()
    }

    /// Returns a copy of this expression.
    pub fn copy(&self) -> Expr {
        Expr::SquareRoot(Box::new(self.operand.clone()))
    }

    /// Tests if this and Other define the same expression.
    pub fn is_identical(&self, other: &Expr) -> bool {
        if let Expr::SquareRoot(inner) = other {
            self.operand == **inner
        } else {
            false
        }
    }

    /// Linear iff it contains no unknowns.
    pub fn is_linear(&self) -> bool {
        !self.operand.contains_unknowns()
    }

    /// d/dx sqrt(u) = u' / (2 * sqrt(u)); zero when x is not contained.
    pub fn derivative(&self, x: &str) -> Expr {
        if !self.operand.contains(x) {
            return Expr::Numeric(0.0);
        }
        let myder = self.operand.derivative(x);
        let sq = Expr::SquareRoot(Box::new(self.operand.clone()));
        let theprod = Expr::product(Expr::Numeric(2.0), sq);
        Expr::division(myder, theprod)
    }

    /// Evaluates this expression given variable bindings.
    pub fn evaluate(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        Ok(self.operand.evaluate(vars)?.sqrt())
    }

    /// "Sqrt(<operand>)".
    pub fn string(&self) -> String {
        format!("Sqrt({})", self.operand.string())
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
        let sqrt = SquareRoot::new(x());
        assert_eq!(*sqrt.operand(), Expr::Unknown("x".into()));
    }

    #[test]
    fn test_copy_is_identical() {
        let sqrt = SquareRoot::new(x());
        let copy = sqrt.copy();
        assert!(sqrt.is_identical(&copy));
        assert!(!sqrt.is_identical(&x()));
        let other = SquareRoot::new(Expr::Unknown("y".into()));
        assert!(!sqrt.is_identical(&other.copy()));
    }

    #[test]
    fn test_shallow_simplified_numeric() {
        let sqrt = SquareRoot::new(Expr::Numeric(9.0));
        assert_eq!(sqrt.shallow_simplified(), Expr::Numeric(3.0));
    }

    #[test]
    fn test_shallow_simplified_square() {
        let sqrt = SquareRoot::new(Expr::Square(Box::new(x())));
        assert_eq!(sqrt.shallow_simplified(), x());
    }

    #[test]
    fn test_shallow_simplified_default_returns_self() {
        let sqrt = SquareRoot::new(x());
        assert_eq!(sqrt.shallow_simplified(), sqrt.copy());
    }

    #[test]
    fn test_is_linear() {
        assert!(SquareRoot::new(Expr::Numeric(2.0)).is_linear());
        assert!(!SquareRoot::new(x()).is_linear());
    }

    #[test]
    fn test_evaluate() {
        let sqrt = SquareRoot::new(x());
        assert_eq!(sqrt.evaluate(&[("x", 16.0)]).unwrap(), 4.0);
        assert!(sqrt.evaluate(&[]).is_err());
    }

    #[test]
    fn test_derivative_not_contained_is_zero() {
        let sqrt = SquareRoot::new(x());
        assert_eq!(sqrt.derivative("y"), Expr::Numeric(0.0));
    }

    #[test]
    fn test_derivative_value() {
        // d/dx sqrt(x) at x=4 is 1/(2*2) = 0.25
        let sqrt = SquareRoot::new(x());
        let der = sqrt.derivative("x");
        let v = der.evaluate(&[("x", 4.0)]).unwrap();
        assert!((v - 0.25).abs() < 1e-12);
    }

    #[test]
    fn test_string() {
        let sqrt = SquareRoot::new(x());
        assert_eq!(sqrt.string(), "Sqrt(x)");
    }

    #[test]
    fn test_sub_expression() {
        let e = Expr::SquareRoot(Box::new(x()));
        assert_eq!(e.nb_sub_expressions(), 1);
        assert_eq!(*e.sub_expression(1), x());
    }
}
