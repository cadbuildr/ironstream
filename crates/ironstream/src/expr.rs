// FILE: expr.rs
// occt: Expr

//! Mathematical expression system with support for functions, relations, and variables.
//! Provides expression trees for symbolic computation.

/// Utility functions for expression operations
pub struct Expr;

impl Expr {
    /// Get the sign of a value: returns -1.0, 0.0, or 1.0
    pub fn sign(val: f64) -> f64 {
        if val > 0.0 {
            1.0
        } else if val < 0.0 {
            -1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_sign_positive() {
        assert_eq!(Expr::sign(3.14), 1.0);
        assert_eq!(Expr::sign(0.001), 1.0);
    }

    #[test]
    fn test_expr_sign_negative() {
        assert_eq!(Expr::sign(-3.14), -1.0);
        assert_eq!(Expr::sign(-0.001), -1.0);
    }

    #[test]
    fn test_expr_sign_zero() {
        assert_eq!(Expr::sign(0.0), 0.0);
    }
}
