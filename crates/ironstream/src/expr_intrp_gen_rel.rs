// FILE: expr_intrp_gen_rel.rs
// occt: ExprIntrp_GenRel

//! Generator for relations in the expression interpreter.

/// Relation generator
pub struct ExprIntrpGenRel;

impl ExprIntrpGenRel {
    /// Generate a relation
    pub fn generate(left: &str, op: &str, right: &str) -> Result<String, String> {
        if left.is_empty() || op.is_empty() || right.is_empty() {
            Err("Invalid relation".to_string())
        } else {
            Ok(format!("{} {} {}", left, op, right))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_relation() {
        let result = ExprIntrpGenRel::generate("x", "=", "5");
        assert!(result.is_ok());
    }
}
