// FILE: gcc_ana_no_solution.rs
// occt: GccAna_NoSolution

use std::fmt;

/// Exception raised when no solution exists for a geometric problem.
#[derive(Debug, Clone)]
pub struct GccAnaNoSolution {
    message: String,
}

impl GccAnaNoSolution {
    pub fn new(msg: &str) -> Self {
        GccAnaNoSolution {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for GccAnaNoSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GccAna_NoSolution: {}", self.message)
    }
}

impl std::error::Error for GccAnaNoSolution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exception() {
        let exc = GccAnaNoSolution::new("Test message");
        assert_eq!(exc.message, "Test message");
    }

    #[test]
    fn test_display() {
        let exc = GccAnaNoSolution::new("No solution found");
        let s = format!("{}", exc);
        assert!(s.contains("No solution found"));
    }
}
