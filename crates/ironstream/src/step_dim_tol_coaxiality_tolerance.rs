// FILE: step_dim_tol_coaxiality_tolerance.rs
// occt: StepDimTol_CoaxialityTolerance

pub struct StepDimTolCoaxialityTolerance {
    value: f64,
}

impl StepDimTolCoaxialityTolerance {
    pub fn new(value: f64) -> Self {
        StepDimTolCoaxialityTolerance { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coaxiality_tolerance_new() {
        let tol = StepDimTolCoaxialityTolerance::new(0.2);
        assert!((tol.value() - 0.2).abs() < 1e-10);
    }
}
