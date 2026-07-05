// FILE: expr_intrp_gen_fct.rs
// occt: ExprIntrp_GenFct

//! Generator for functions in the expression interpreter.

/// Function generator
pub struct ExprIntrpGenFct;

impl ExprIntrpGenFct {
    /// Generate a function
    pub fn generate(name: &str, args: &[&str]) -> Result<String, String> {
        if name.is_empty() {
            Err("Empty function name".to_string())
        } else {
            Ok(format!("{}({})", name, args.join(",")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_function() {
        let result = ExprIntrpGenFct::generate("sin", &["x"]);
        assert!(result.is_ok());
    }
}
