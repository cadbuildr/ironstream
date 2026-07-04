// FILE: step_dim_tol_concentricity_tolerance.rs
// occt: StepDimTol_ConcentricityTolerance

pub struct StepDimTolConcentricityTolerance {
    value: f64,
}

impl StepDimTolConcentricityTolerance {
    pub fn new(value: f64) -> Self {
        StepDimTolConcentricityTolerance { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concentricity_tolerance_new() {
        let tol = StepDimTolConcentricityTolerance::new(0.05);
        assert!((tol.value() - 0.05).abs() < 1e-10);
    }
}
