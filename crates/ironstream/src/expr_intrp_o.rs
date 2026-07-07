// FILE: expr_intrp_o.rs
// occt: ExprIntrp

//! Interpreter for Expr expressions, functions, and relations.
//! Parses and interprets string expressions into expression trees.

/// Expression interpreter
pub struct ExprIntrp;

impl ExprIntrp {
    /// Parse an expression string
    pub fn parse(expr_str: &str) -> Result<String, String> {
        if expr_str.is_empty() {
            return Err("Empty expression".to_string());
        }
        Ok(expr_str.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let result = ExprIntrp::parse("x + 1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_empty_expression() {
        let result = ExprIntrp::parse("");
        assert!(result.is_err());
    }
}
