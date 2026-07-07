// FILE: expr_intrp_gen_exp.rs
// occt: ExprIntrp_GenExp

//! Generator for general expressions in the interpreter.

/// General expression generator
pub struct ExprIntrpGenExp;

impl ExprIntrpGenExp {
    /// Generate an expression
    pub fn generate(expr: &str) -> Result<String, String> {
        if expr.is_empty() {
            Err("Empty expression".to_string())
        } else {
            Ok(format!("expr({})", expr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let result = ExprIntrpGenExp::generate("x + 1");
        assert!(result.is_ok());
    }
}
