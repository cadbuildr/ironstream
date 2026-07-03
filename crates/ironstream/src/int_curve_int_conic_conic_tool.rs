// FILE: int_curve_int_conic_conic_tool.rs
// occt: IntCurve_IntConicConic_Tool

/// Tool for conic-conic intersection computations.
pub struct IntCurveIntConicConicTool {
    nb_solutions: i32,
}

impl IntCurveIntConicConicTool {
    /// Create tool
    pub fn new() -> Self {
        IntCurveIntConicConicTool { nb_solutions: 0 }
    }

    /// Get number of solutions
    pub fn nb_solutions(&self) -> i32 {
        self.nb_solutions
    }

    /// Set number of solutions
    pub fn set_nb_solutions(&mut self, n: i32) {
        self.nb_solutions = n;
    }
}

impl Default for IntCurveIntConicConicTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IntCurveIntConicConicTool::new();
        assert_eq!(tool.nb_solutions(), 0);
    }

    #[test]
    fn test_set_nb_solutions() {
        let mut tool = IntCurveIntConicConicTool::new();
        tool.set_nb_solutions(4);
        assert_eq!(tool.nb_solutions(), 4);
    }
}
