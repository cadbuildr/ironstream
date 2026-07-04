// FILE: step_dim_tol_cylindricity_tolerance.rs
// occt: StepDimTol_CylindricityTolerance

pub struct StepDimTolCylindricityTolerance {
    value: f64,
}

impl StepDimTolCylindricityTolerance {
    pub fn new(value: f64) -> Self {
        StepDimTolCylindricityTolerance { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylindricity_tolerance_new() {
        let tol = StepDimTolCylindricityTolerance::new(0.3);
        assert!((tol.value() - 0.3).abs() < 1e-10);
    }
}
