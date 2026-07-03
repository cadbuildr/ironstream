// FILE: f_em_tool_linear_jerk.rs
// occt: FEmTool_LinearJerk

/// Linear jerk criterion for FEM.
pub struct FemToolLinearJerk {
    jerk: f64,
}

impl FemToolLinearJerk {
    pub fn new() -> Self {
        FemToolLinearJerk { jerk: 0.0 }
    }

    pub fn compute(&mut self, a: f64, b: f64, c: f64) {
        self.jerk = (a * a + b * b + c * c).sqrt();
    }

    pub fn value(&self) -> f64 {
        self.jerk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let jerk = FemToolLinearJerk::new();
        assert_eq!(jerk.value(), 0.0);
    }

    #[test]
    fn test_compute() {
        let mut jerk = FemToolLinearJerk::new();
        jerk.compute(1.0, 1.0, 1.0);
        assert!((jerk.value() - 1.732).abs() < 0.01);
    }
}
