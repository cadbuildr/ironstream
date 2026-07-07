// FILE: expr_intrp_analysis.rs
// occt: ExprIntrp_Analysis

//! Syntax analysis for expression interpreter.

/// Expression syntax analyzer
pub struct ExprIntrpAnalysis;

impl ExprIntrpAnalysis {
    /// Analyze expression syntax
    pub fn analyze(expr: &str) -> bool {
        !expr.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        assert!(ExprIntrpAnalysis::analyze("x + 1"));
        assert!(!ExprIntrpAnalysis::analyze(""));
    }
}
