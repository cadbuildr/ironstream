// FILE: expr_intrp.rs
// occt: Expr_NumericValue, Expr_NamedUnknown, Expr_Sum,
//       Expr_Product, Expr_Exponent, Expr_UnaryMinus,
//       Expr_Sine, Expr_Cosine, ExprIntrp_GenExp

/// A symbolic expression node.
/// occt: Expr_GeneralExpression (simplified)
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Constant numeric value.
    Numeric(f64),
    /// Named variable.
    Unknown(String),
    /// Sum of two expressions.
    Sum(Box<Expr>, Box<Expr>),
    /// Difference (a - b).
    Difference(Box<Expr>, Box<Expr>),
    /// Product.
    Product(Box<Expr>, Box<Expr>),
    /// Division (a / b).
    Division(Box<Expr>, Box<Expr>),
    /// Power: base^exp.
    Exponent(Box<Expr>, Box<Expr>),
    /// Unary negation.
    UnaryMinus(Box<Expr>),
    /// Sine.
    Sine(Box<Expr>),
    /// Cosine.
    Cosine(Box<Expr>),
    /// Square root.
    Sqrt(Box<Expr>),
    /// Absolute value.
    Abs(Box<Expr>),
    /// Natural log.
    Log(Box<Expr>),
}

impl Expr {
    pub fn num(v: f64) -> Self { Self::Numeric(v) }
    pub fn var(name: impl Into<String>) -> Self { Self::Unknown(name.into()) }
    pub fn sum(a: Expr, b: Expr) -> Self { Self::Sum(Box::new(a), Box::new(b)) }
    pub fn diff(a: Expr, b: Expr) -> Self { Self::Difference(Box::new(a), Box::new(b)) }
    pub fn prod(a: Expr, b: Expr) -> Self { Self::Product(Box::new(a), Box::new(b)) }
    pub fn div(a: Expr, b: Expr) -> Self { Self::Division(Box::new(a), Box::new(b)) }
    pub fn pow(base: Expr, exp: Expr) -> Self { Self::Exponent(Box::new(base), Box::new(exp)) }
    pub fn neg(a: Expr) -> Self { Self::UnaryMinus(Box::new(a)) }
    pub fn sin(a: Expr) -> Self { Self::Sine(Box::new(a)) }
    pub fn cos(a: Expr) -> Self { Self::Cosine(Box::new(a)) }
    pub fn sqrt(a: Expr) -> Self { Self::Sqrt(Box::new(a)) }

    /// Evaluate with a variable binding map.
    pub fn eval(&self, vars: &[(&str, f64)]) -> Result<f64, String> {
        match self {
            Self::Numeric(v) => Ok(*v),
            Self::Unknown(name) => {
                vars.iter().find(|(k, _)| *k == name.as_str())
                    .map(|(_, v)| *v)
                    .ok_or_else(|| format!("unbound variable: {}", name))
            }
            Self::Sum(a, b) => Ok(a.eval(vars)? + b.eval(vars)?),
            Self::Difference(a, b) => Ok(a.eval(vars)? - b.eval(vars)?),
            Self::Product(a, b) => Ok(a.eval(vars)? * b.eval(vars)?),
            Self::Division(a, b) => {
                let bv = b.eval(vars)?;
                if bv.abs() < 1e-14 { return Err("division by zero".into()); }
                Ok(a.eval(vars)? / bv)
            }
            Self::Exponent(base, exp) => Ok(base.eval(vars)?.powf(exp.eval(vars)?)),
            Self::UnaryMinus(a) => Ok(-a.eval(vars)?),
            Self::Sine(a) => Ok(a.eval(vars)?.sin()),
            Self::Cosine(a) => Ok(a.eval(vars)?.cos()),
            Self::Sqrt(a) => {
                let v = a.eval(vars)?;
                if v < 0.0 { return Err("sqrt of negative".into()); }
                Ok(v.sqrt())
            }
            Self::Abs(a) => Ok(a.eval(vars)?.abs()),
            Self::Log(a) => {
                let v = a.eval(vars)?;
                if v <= 0.0 { return Err("log of non-positive".into()); }
                Ok(v.ln())
            }
        }
    }

    /// Check if expression contains a specific variable name.
    pub fn contains(&self, name: &str) -> bool {
        match self {
            Self::Unknown(n) => n == name,
            Self::Numeric(_) => false,
            Self::Sum(a, b) | Self::Difference(a, b) | Self::Product(a, b)
            | Self::Division(a, b) | Self::Exponent(a, b) => a.contains(name) || b.contains(name),
            Self::UnaryMinus(a) | Self::Sine(a) | Self::Cosine(a)
            | Self::Sqrt(a) | Self::Abs(a) | Self::Log(a) => a.contains(name),
        }
    }

    /// Count number of sub-expressions (depth-first).
    pub fn node_count(&self) -> usize {
        match self {
            Self::Numeric(_) | Self::Unknown(_) => 1,
            Self::Sum(a, b) | Self::Difference(a, b) | Self::Product(a, b)
            | Self::Division(a, b) | Self::Exponent(a, b) => 1 + a.node_count() + b.node_count(),
            Self::UnaryMinus(a) | Self::Sine(a) | Self::Cosine(a)
            | Self::Sqrt(a) | Self::Abs(a) | Self::Log(a) => 1 + a.node_count(),
        }
    }

    /// Simplify obvious constant-folding cases one level deep.
    pub fn simplify(self) -> Self {
        match self {
            Self::Sum(a, b) => {
                let (a, b) = (*a, *b);
                if let (Self::Numeric(x), Self::Numeric(y)) = (&a, &b) {
                    return Self::Numeric(x + y);
                }
                Self::Sum(Box::new(a), Box::new(b))
            }
            Self::Product(a, b) => {
                let (a, b) = (*a, *b);
                if let (Self::Numeric(x), Self::Numeric(y)) = (&a, &b) {
                    return Self::Numeric(x * y);
                }
                if let Self::Numeric(v) = &a { if *v == 0.0 { return Self::Numeric(0.0); } }
                if let Self::Numeric(v) = &b { if *v == 0.0 { return Self::Numeric(0.0); } }
                Self::Product(Box::new(a), Box::new(b))
            }
            other => other,
        }
    }
}

/// Simple named-variable store for expression evaluation.
#[derive(Clone, Debug, Default)]
pub struct ExprVariableStore {
    pub bindings: Vec<(String, f64)>,
}

impl ExprVariableStore {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, name: impl Into<String>, value: f64) {
        let k = name.into();
        if let Some(e) = self.bindings.iter_mut().find(|(n, _)| *n == k) {
            e.1 = value;
        } else {
            self.bindings.push((k, value));
        }
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.bindings.iter().find(|(n, _)| n == name).map(|(_, v)| *v)
    }

    pub fn as_slice(&self) -> Vec<(&str, f64)> {
        self.bindings.iter().map(|(k, v)| (k.as_str(), *v)).collect()
    }

    pub fn eval(&self, expr: &Expr) -> Result<f64, String> {
        let binds = self.as_slice();
        expr.eval(&binds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_eval() {
        let e = Expr::num(3.14);
        assert!((e.eval(&[]).unwrap() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn variable_eval() {
        let e = Expr::sum(Expr::var("x"), Expr::num(1.0));
        let v = e.eval(&[("x", 4.0)]).unwrap();
        assert!((v - 5.0).abs() < 1e-10);
    }

    #[test]
    fn product_and_power() {
        // x^2 * 2 at x=3 → 18
        let e = Expr::prod(
            Expr::pow(Expr::var("x"), Expr::num(2.0)),
            Expr::num(2.0),
        );
        assert!((e.eval(&[("x", 3.0)]).unwrap() - 18.0).abs() < 1e-10);
    }

    #[test]
    fn sin_cos() {
        use std::f64::consts::PI;
        let s = Expr::sin(Expr::num(PI / 2.0));
        assert!((s.eval(&[]).unwrap() - 1.0).abs() < 1e-10);
        let c = Expr::cos(Expr::num(0.0));
        assert!((c.eval(&[]).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn simplify_constants() {
        let e = Expr::sum(Expr::num(3.0), Expr::num(4.0)).simplify();
        assert_eq!(e, Expr::Numeric(7.0));
        let e2 = Expr::prod(Expr::num(0.0), Expr::var("x")).simplify();
        assert_eq!(e2, Expr::Numeric(0.0));
    }

    #[test]
    fn variable_store() {
        let mut s = ExprVariableStore::new();
        s.bind("x", 5.0);
        s.bind("y", 3.0);
        let e = Expr::diff(Expr::var("x"), Expr::var("y"));
        assert!((s.eval(&e).unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn unbound_variable_error() {
        let e = Expr::var("z");
        assert!(e.eval(&[("x", 1.0)]).is_err());
    }

    #[test]
    fn node_count() {
        let e = Expr::sum(Expr::num(1.0), Expr::prod(Expr::var("x"), Expr::num(2.0)));
        assert_eq!(e.node_count(), 5);
    }
}
